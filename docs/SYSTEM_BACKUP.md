# System backup contract

The security role renders `roles/security/templates/backup_script.sh.j2` into
`/usr/local/bin/system_backup.sh`. Its existing root cron invokes this script.
The script requires Bash, rsync, tar, flock and standard Linux coreutils.
Before copying, it budgets twice the source disk usage plus a 1 GiB reserve
on the actual destination filesystem. Insufficient capacity is an explicit
failure before staging begins. This estimate is conservative, and changing
sources can still exceed it. MySQL dump size is not included in the estimate.

Backups hold an exclusive lock, copy configured source directories into private
staging, optionally capture MySQL, create a gzip tar archive, and validate its
complete table of contents before publication. Archives are mode 0600 in a 0700
destination. A hard link publishes the complete archive on the same filesystem
without overwriting an existing file. The destination is excluded when nested
inside a source. At least one source or database must be captured.

Any failed copy, dump, compression or validation exits nonzero. Failed staging
is preserved with restricted permissions for investigation; operators must
inspect it before removing it. Repeated failures therefore require disk-space
monitoring and intervention. A failed run never starts retention or reports
completion. Legacy timestamp-only archives are never deleted by this producer.
After publication, retention only removes this producer's completed filename
pattern and always protects the newly published archive.

Run the Linux integration tests with Jinja2, rsync and flock installed:

```sh
python3 -m unittest discover -s tests -v
```

The tests use temporary fixtures and injected command failures. They do not
read a production database or system configuration. This file backup does not
provide an application-consistent database snapshot, offsite storage, encryption
or a complete machine restore. FoodShare's separate encrypted database/Vault
recovery service provides its own recovery unit; verify both workflows according
to their actual coverage. Do not run a broad provisioning playbook merely to
update this script. Review rendered output and install only the intended file.
