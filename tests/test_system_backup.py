"""Run on Linux: python3 -m unittest discover -s tests -v (requires Jinja2/rsync)."""
import fcntl
import os
from pathlib import Path
import shlex
import subprocess
import tempfile
import time
import unittest

from jinja2 import Environment, StrictUndefined


class SystemBackupTest(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.source = self.root / "source with spaces"
        self.source.mkdir()
        (self.source / "precious.txt").write_text("recover me")
        self.destination = self.root / "backups"
        self.destination.mkdir()
        self.bin = self.root / "bin"
        self.bin.mkdir()
        self.environment = dict(os.environ, PATH=f"{self.bin}:/usr/bin:/bin")
        self.legacy = self.destination / "20200101_020001.tar.gz"
        self.legacy.write_text("preserve legacy")
        self.old = self.destination / "system-backup-20200101T020001Z-abcdefgh.tar.gz"
        self.old.write_text("old completed object")
        for path in (self.old, self.legacy):
            os.utime(path, (time.time() - 86400 * 60,) * 2)

    def render(self, mysql=False):
        template = Path(__file__).resolve().parents[1] / "roles/security/templates/backup_script.sh.j2"
        environment = Environment(undefined=StrictUndefined)
        environment.filters["quote"] = lambda value: shlex.quote(str(value))
        script = self.root / "backup.sh"
        script.write_text(environment.from_string(template.read_text()).render(
            backup_destination=str(self.destination), backup_retention_days=7,
            backup_directories=[str(self.source)], backup_mysql=mysql))
        return script

    def run_backup(self, mysql=False):
        return subprocess.run(["bash", str(self.render(mysql))], env=self.environment,
                              text=True, capture_output=True, timeout=20)

    def fake(self, name, body):
        executable = self.bin / name
        executable.write_text("#!/bin/bash\n" + body + "\n")
        executable.chmod(0o700)

    def assert_failed_safely(self, result):
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn("Backup completed", result.stdout)
        self.assertTrue(self.old.exists())
        self.assertTrue(self.legacy.exists())
        self.assertEqual(list(self.destination.glob("system-backup-*.tar.gz")), [self.old])
        stages = [p for p in self.destination.glob(".system-backup.*") if p.is_dir()]
        self.assertEqual(len(stages), 1)
        self.assertEqual(stages[0].stat().st_mode & 0o777, 0o700)

    def test_success_validates_publishes_restricts_and_then_prunes(self):
        result = self.run_backup()
        self.assertEqual(result.returncode, 0, result.stderr)
        archives = list(self.destination.glob("system-backup-*.tar.gz"))
        self.assertEqual(len(archives), 1)
        self.assertEqual(archives[0].stat().st_mode & 0o777, 0o600)
        self.assertEqual(self.destination.stat().st_mode & 0o777, 0o700)
        self.assertFalse(self.old.exists())
        self.assertTrue(self.legacy.exists())
        recovered = subprocess.check_output(["tar", "-xOzf", str(archives[0]),
                                           "." + str(self.source / "precious.txt")])
        self.assertEqual(recovered, b"recover me")
        self.assertEqual([p for p in self.destination.glob(".system-backup.*") if p.is_dir()], [])

    def test_copy_failure_preserves_prior_backups_and_staging(self):
        self.fake("rsync", "exit 23")
        self.assert_failed_safely(self.run_backup())

    def test_archive_failure_never_publishes(self):
        self.fake("tar", "exit 2")
        self.assert_failed_safely(self.run_backup())

    def test_invalid_archive_never_publishes(self):
        self.fake("tar", 'if [[ "$1" = -czf ]]; then echo broken > "$2"; else exit 2; fi')
        self.assert_failed_safely(self.run_backup())

    def test_database_dump_failure_never_publishes(self):
        self.fake("mysqldump", "echo partial; exit 1")
        self.assert_failed_safely(self.run_backup(mysql=True))

    def test_concurrent_invocation_cannot_copy_or_prune(self):
        with (self.destination / ".system-backup.lock").open("w") as lock:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
            result = self.run_backup()
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual([p for p in self.destination.glob(".system-backup.*") if p.is_dir()], [])
        self.assertTrue(self.old.exists())

    def test_destination_nested_in_source_is_excluded(self):
        self.destination = self.source / "nested backups"
        self.destination.mkdir()
        (self.destination / "must-not-copy").write_text("private previous backup")
        result = self.run_backup()
        self.assertEqual(result.returncode, 0, result.stderr)
        archive, = self.destination.glob("system-backup-*.tar.gz")
        listing = subprocess.check_output(["tar", "-tzf", str(archive)], text=True)
        self.assertIn("precious.txt", listing)
        self.assertNotIn("nested backups", listing)

    def test_symlink_destination_is_rejected(self):
        original = self.destination
        self.destination = self.root / "linked-backups"
        self.destination.symlink_to(original, target_is_directory=True)
        result = self.run_backup()
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(list(original.glob(".system-backup.*")), [])


    def test_insufficient_space_fails_before_copying(self):
        self.fake("df", 'printf "Filesystem 1024-blocks Used Available Capacity Mounted on\\nfixture 100 99 1 99%% /\\n"')
        result = self.run_backup()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("insufficient space", result.stderr)
        self.assertTrue(self.old.exists())
        self.assertEqual([p for p in self.destination.glob(".system-backup.*") if p.is_dir()], [])

    def test_all_sources_absent_is_not_an_empty_success(self):
        (self.source / "precious.txt").unlink()
        self.source.rmdir()
        self.assert_failed_safely(self.run_backup())


if __name__ == "__main__":
    unittest.main()
