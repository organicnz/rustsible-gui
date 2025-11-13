#!/usr/bin/env python3
"""
Interactive Ansible Playbook Launcher
Provides checkbox selection for features with visual indicators
"""

import subprocess
import sys
import os
from pathlib import Path

# Get the directory where this script is located
SCRIPT_DIR = Path(__file__).parent
VENV_DIR = SCRIPT_DIR / ".venv"
VENV_PYTHON = VENV_DIR / "bin" / "python3"

def setup_venv():
    """Create virtual environment and install dependencies if needed"""
    if not VENV_DIR.exists():
        print("📦 Setting up virtual environment...")
        subprocess.check_call([sys.executable, "-m", "venv", str(VENV_DIR)])
        print("📦 Installing inquirer package...")
        subprocess.check_call([str(VENV_PYTHON), "-m", "pip", "install", "--quiet", "inquirer"])
        print("✅ Setup complete!\n")

    # Re-run this script with the venv Python
    if sys.executable != str(VENV_PYTHON):
        os.execv(str(VENV_PYTHON), [str(VENV_PYTHON)] + sys.argv)

# Ensure we're running in the venv
if not str(VENV_PYTHON) in sys.executable:
    setup_venv()

import inquirer

def main():
    print("\n" + "═" * 65)
    print("   UBUNTU SERVER PROVISIONING - Interactive Setup")
    print("═" * 65)
    print()

    # Show preview of all questions
    print("📋 \033[1;36mQUESTIONS OVERVIEW\033[0m")
    print("─" * 65)
    print()
    print("  \033[1;33m1.\033[0m 🖥️  Server IP address")
    print("  \033[1;33m2.\033[0m 👤 SSH username")
    print("  \033[1;33m3.\033[0m 🔑 SSH private key path")
    print("  \033[1;33m4.\033[0m 📦 Select features to install (12 options available)")
    print()
    print("     \033[1;90mCore Features:\033[0m")
    print("       • Fail2ban, Docker, LEMP, Swap, Cron Jobs")
    print("       • Development Tools, WordPress, Certbot")
    print()
    print("     \033[1;90mSecurity Clusters:\033[0m")
    print("       • 🔐 System Hardening (4 features)")
    print("       • 📊 Monitoring & Detection (5 features)")
    print("       • 🌐 Network Security (2 features)")
    print("       • 🔑 Advanced Protection (3 features)")
    print()
    print("─" * 65)
    print("\033[1;32mReady?\033[0m Press ENTER to begin, or Ctrl+C to cancel")
    print()
    input()

    print("\n" + "═" * 65)
    print("   ANSWERING QUESTIONS")
    print("═" * 65)
    print()

    # Combined questions - connection info + features
    all_questions = [
        inquirer.Text('target_ip',
                     message="🖥️  Server IP address",
                     validate=lambda _, x: len(x) > 0),
        inquirer.Text('target_user',
                     message="👤 SSH username",
                     default="root"),
        inquirer.Text('ssh_key_path',
                     message="🔑 SSH private key path",
                     default="~/.ssh/id_rsa_gitlab"),
        inquirer.Checkbox('features',
                         message="📦 Select features to install",
                         choices=[
                             ('🛡️  Fail2ban Intrusion Prevention', 'fail2ban'),
                             ('🐳 Docker & Docker Compose', 'docker'),
                             ('🌐 LEMP Stack (Nginx, MySQL, PHP)', 'lemp'),
                             ('💾 Swap Memory Configuration', 'swap'),
                             ('⏰ Automated Cron Jobs', 'cron'),
                             ('⚙️  Development Tools (Neovim, Node.js, Claude Code)', 'devtools'),
                             ('📝 WordPress CMS', 'wordpress'),
                             ('🔒 Certbot SSL/TLS Certificates', 'certbot'),
                             ('─' * 40, 'separator1'),  # Visual separator
                             ('🔐 System Hardening (Kernel, AppArmor, Auto-updates)', 'system_hardening'),
                             ('📊 Monitoring & Detection (Lynis, AIDE, rkhunter, Logs)', 'monitoring_detection'),
                             ('🌐 Network Security (IPv6 disable, IDS)', 'network_security'),
                             ('🔑 Advanced Protection (2FA, Backups, USB)', 'advanced_protection'),
                         ],
                         default=['fail2ban', 'docker', 'swap', 'cron', 'system_hardening', 'monitoring_detection']),
    ]

    answers = inquirer.prompt(all_questions)

    if not answers:
        print("\n❌ Setup cancelled")
        sys.exit(1)

    # Extract connection info
    connection_answers = {
        'target_ip': answers['target_ip'],
        'target_user': answers['target_user'],
        'ssh_key_path': answers['ssh_key_path']
    }

    # Extract and clean features (remove separators)
    selected_features = [f for f in answers['features'] if not f.startswith('separator')]

    # Display selection summary
    print("\n" + "═" * 65)
    print("   CONFIGURATION SUMMARY")
    print("═" * 65 + "\n")

    print(f"📍 Target Server: \033[1;36m{connection_answers['target_ip']}\033[0m")
    print(f"👤 SSH User:      \033[1;36m{connection_answers['target_user']}\033[0m")
    print(f"🔑 SSH Key:       \033[1;36m{connection_answers['ssh_key_path']}\033[0m\n")

    print("Selected Features:")
    feature_names = {
        'fail2ban': 'Fail2ban Intrusion Prevention',
        'docker': 'Docker & Docker Compose',
        'lemp': 'LEMP Stack',
        'swap': 'Swap Memory Configuration',
        'cron': 'Automated Cron Jobs',
        'devtools': 'Development Tools',
        'wordpress': 'WordPress CMS',
        'certbot': 'Certbot SSL/TLS',
        'system_hardening': 'System Hardening',
        'monitoring_detection': 'Monitoring & Detection',
        'network_security': 'Network Security',
        'advanced_protection': 'Advanced Protection'
    }

    for feature in ['fail2ban', 'docker', 'lemp', 'swap', 'cron', 'devtools', 'wordpress', 'certbot',
                    'system_hardening', 'monitoring_detection', 'network_security', 'advanced_protection']:
        if feature in selected_features:
            print(f"  \033[1;32m✓\033[0m {feature_names[feature]}")
        else:
            print(f"  \033[1;90m✗\033[0m {feature_names[feature]}")

    # Show security cluster breakdowns if selected
    if 'system_hardening' in selected_features:
        print("\n  \033[1;33m▸ System Hardening includes:\033[0m")
        print("    • Kernel Hardening (sysctl) - Network attack protection")
        print("    • AppArmor Enforcement - Mandatory access control")
        print("    • Secure Shared Memory - Privilege escalation prevention")
        print("    • Unattended Security Upgrades - Auto-patch vulnerabilities")

    if 'monitoring_detection' in selected_features:
        print("\n  \033[1;33m▸ Monitoring & Detection includes:\033[0m")
        print("    • Lynis Security Auditing - Comprehensive security scanner")
        print("    • Rootkit Detection (rkhunter) - Daily rootkit/backdoor scans")
        print("    • File Integrity Monitoring (AIDE) - Detect unauthorized changes")
        print("    • Audit Logging (auditd) - System call and security event logs")
        print("    • Log Monitoring (Logwatch) - Daily log analysis reports")

    if 'network_security' in selected_features:
        print("\n  \033[1;33m▸ Network Security includes:\033[0m")
        print("    • IPv6 Disable - Reduce attack surface (optional)")
        print("    • Network IDS (Suricata) - Deep packet inspection and threat detection")

    if 'advanced_protection' in selected_features:
        print("\n  \033[1;33m▸ Advanced Protection includes:\033[0m")
        print("    • 2FA SSH Authentication - Google Authenticator (requires manual setup)")
        print("    • Automated Backups - Scheduled system backups with retention")
        print("    • USB Restrictions - Block USB storage devices")

    # Validate prerequisites
    if 'wordpress' in selected_features and 'lemp' not in selected_features:
        print("\n\033[1;31m❌ ERROR:\033[0m WordPress requires LEMP stack to be installed.")
        sys.exit(1)

    # Confirmation
    print("\n" + "─" * 65)
    confirm = inquirer.confirm("Continue with this configuration?", default=True)

    if not confirm:
        print("\n❌ Setup cancelled")
        sys.exit(1)

    # Build ansible-playbook command
    cmd = [
        'ansible-playbook',
        'playbook.yml',
        '-e', f"target_ip={connection_answers['target_ip']}",
        '-e', f"target_user={connection_answers['target_user']}",
        '-e', f"ssh_key_path={connection_answers['ssh_key_path']}",
        '-e', f"prompt_enable_fail2ban={'yes' if 'fail2ban' in selected_features else 'no'}",
        '-e', f"prompt_install_docker={'yes' if 'docker' in selected_features else 'no'}",
        '-e', f"prompt_install_lemp={'yes' if 'lemp' in selected_features else 'no'}",
        '-e', f"prompt_enable_swap={'yes' if 'swap' in selected_features else 'no'}",
        '-e', f"prompt_enable_cron_jobs={'yes' if 'cron' in selected_features else 'no'}",
        '-e', f"prompt_install_dev_tools={'yes' if 'devtools' in selected_features else 'no'}",
        '-e', f"prompt_install_wordpress={'yes' if 'wordpress' in selected_features else 'no'}",
        '-e', f"prompt_install_certbot={'yes' if 'certbot' in selected_features else 'no'}",
    ]

    # Add security cluster features if selected
    if 'system_hardening' in selected_features:
        cmd.extend([
            '-e', 'enable_kernel_hardening=true',
            '-e', 'enable_apparmor=true',
            '-e', 'enable_secure_shm=true',
            '-e', 'enable_unattended_upgrades=true',
        ])

    if 'monitoring_detection' in selected_features:
        cmd.extend([
            '-e', 'enable_lynis=true',
            '-e', 'enable_rkhunter=true',
            '-e', 'enable_aide=true',
            '-e', 'enable_auditd=true',
            '-e', 'enable_logwatch=true',
        ])

    if 'network_security' in selected_features:
        cmd.extend([
            '-e', 'disable_ipv6=false',  # Optional, user can enable in vars
            '-e', 'enable_suricata=false',  # Resource intensive, disabled by default
        ])

    if 'advanced_protection' in selected_features:
        cmd.extend([
            '-e', 'enable_ssh_2fa=false',  # Requires manual setup post-install
            '-e', 'enable_backups=false',  # Requires configuration
            '-e', 'enable_usb_restrictions=false',  # Can break USB devices
        ])

    print("\n" + "═" * 65)
    print("   🚀 STARTING ANSIBLE PLAYBOOK")
    print("═" * 65)
    print()
    print("\033[1;36m┌─────────────────────────────────────────────────────────┐\033[0m")
    print("\033[1;36m│  💡 TIP: Watch for green [OK] and yellow [CHANGED]     │\033[0m")
    print("\033[1;36m│       Red [FAILED] means something went wrong           │\033[0m")
    print("\033[1;36m└─────────────────────────────────────────────────────────┘\033[0m")
    print()

    # Run ansible-playbook
    import time
    start_time = time.time()

    try:
        result = subprocess.run(cmd, check=True)

        # Success message
        elapsed = time.time() - start_time
        minutes = int(elapsed // 60)
        seconds = int(elapsed % 60)

        print("\n" + "═" * 65)
        print("\033[1;32m   ✅ PROVISIONING COMPLETED SUCCESSFULLY!\033[0m")
        print("═" * 65)
        print()
        print(f"  ⏱️  Time elapsed: \033[1;36m{minutes}m {seconds}s\033[0m")
        print(f"  🎯 Server: \033[1;36m{connection_answers['target_ip']}\033[0m")
        print(f"  📦 Features installed: \033[1;36m{len(selected_features)}\033[0m")
        print()
        print("\033[1;33m  Next Steps:\033[0m")
        print("  • SSH into your server: \033[1;90mssh {}@{}\033[0m".format(
            connection_answers['target_user'],
            connection_answers['target_ip']))

        if 'docker' in selected_features:
            print("  • Check Docker: \033[1;90mdocker --version\033[0m")
        if 'fail2ban' in selected_features:
            print("  • Check Fail2ban: \033[1;90msudo fail2ban-client status\033[0m")
        if 'lemp' in selected_features:
            print("  • Visit your server in browser: \033[1;90mhttp://{}\033[0m".format(
                connection_answers['target_ip']))

        print()
        print("\033[1;32m  🎉 Your server is ready to use!\033[0m")
        print()

    except subprocess.CalledProcessError as e:
        elapsed = time.time() - start_time
        minutes = int(elapsed // 60)
        seconds = int(elapsed % 60)

        print("\n" + "═" * 65)
        print("\033[1;31m   ❌ PROVISIONING FAILED\033[0m")
        print("═" * 65)
        print()
        print(f"  ⏱️  Time elapsed: \033[1;36m{minutes}m {seconds}s\033[0m")
        print(f"  💥 Exit code: \033[1;31m{e.returncode}\033[0m")
        print()
        print("\033[1;33m  Troubleshooting:\033[0m")
        print("  • Check the error messages above")
        print("  • Verify SSH connection: \033[1;90mssh {}@{}\033[0m".format(
            connection_answers['target_user'],
            connection_answers['target_ip']))
        print("  • Try again with verbose output: \033[1;90mansible-playbook playbook.yml -vv\033[0m")
        print()
        sys.exit(e.returncode)

    except KeyboardInterrupt:
        print("\n\n" + "═" * 65)
        print("\033[1;33m   ⚠️  PLAYBOOK INTERRUPTED BY USER\033[0m")
        print("═" * 65)
        print()
        print("  The provisioning was stopped. Your server may be in an incomplete state.")
        print("  You can re-run the playbook to continue or complete the setup.")
        print()
        sys.exit(130)

if __name__ == '__main__':
    main()
