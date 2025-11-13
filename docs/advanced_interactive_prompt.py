#!/usr/bin/env python3
"""
Advanced Interactive Feature Selection for Ansible Playbook

This script provides a checkbox-based interactive menu for selecting features.
It's an alternative to Ansible's vars_prompt for better UX.

Requirements:
    pip install inquirer

Usage:
    python3 docs/advanced_interactive_prompt.py

    Or integrate with ansible-playbook:
    ansible-playbook playbook.yml -e "$(python3 docs/advanced_interactive_prompt.py --export-vars)"
"""

import sys
import json

try:
    import inquirer
    from inquirer.themes import GreenPassion
except ImportError:
    print("Error: inquirer package not found.")
    print("Install with: pip3 install inquirer")
    sys.exit(1)


class Colors:
    """ANSI color codes for terminal output"""
    HEADER = '\033[95m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
    END = '\033[0m'


def print_header():
    """Print a beautiful header"""
    print("\n" + "=" * 65)
    print(f"{Colors.BOLD}{Colors.CYAN}ANSIBLE UBUNTU SERVER PROVISIONING{Colors.END}")
    print(f"{Colors.BOLD}Interactive Feature Selection{Colors.END}")
    print("=" * 65 + "\n")


def get_connection_info():
    """Prompt for connection information"""
    print(f"{Colors.BOLD}Connection Information{Colors.END}\n")

    questions = [
        inquirer.Text(
            'target_ip',
            message="Server IP address",
            validate=lambda _, x: len(x) > 0 and '.' in x
        ),
        inquirer.Text(
            'target_user',
            message="SSH username",
            default="root"
        ),
        inquirer.Text(
            'ssh_key_path',
            message="SSH private key path",
            default="~/.ssh/id_rsa_gitlab"
        ),
    ]

    return inquirer.prompt(questions, theme=GreenPassion())


def get_feature_selection():
    """Prompt for feature selection with checkboxes"""
    print(f"\n{Colors.BOLD}Feature Selection{Colors.END}")
    print("Use arrow keys to navigate, SPACE to select/deselect, ENTER to confirm\n")

    features = [
        inquirer.Checkbox(
            'features',
            message="Select features to install",
            choices=[
                ('Fail2ban - Intrusion Prevention System', 'fail2ban'),
                ('Docker & Docker Compose', 'docker'),
                ('LEMP Stack (Nginx, MySQL, PHP)', 'lemp'),
                ('Swap Memory (auto-sized)', 'swap'),
                ('Automated Cron Jobs', 'cron_jobs'),
                ('Development Tools (Neovim, Node.js, Claude Code)', 'dev_tools'),
                ('WordPress CMS', 'wordpress'),
                ('Certbot SSL/TLS Certificates', 'certbot'),
            ],
            default=['fail2ban', 'docker', 'swap', 'cron_jobs']
        ),
    ]

    return inquirer.prompt(features, theme=GreenPassion())


def validate_selections(connection, features):
    """Validate feature dependencies"""
    selected_features = features.get('features', [])
    warnings = []
    errors = []

    # WordPress requires LEMP
    if 'wordpress' in selected_features and 'lemp' not in selected_features:
        errors.append("WordPress requires LEMP stack. Please select LEMP or deselect WordPress.")

    # Certbot works best with Nginx
    if 'certbot' in selected_features and 'lemp' not in selected_features:
        warnings.append("Certbot works best with Nginx (LEMP stack). Consider enabling LEMP.")

    return warnings, errors


def display_summary(connection, features):
    """Display a beautiful summary of selections"""
    print("\n" + "┌" + "─" * 63 + "┐")
    print("│" + " " * 20 + "CONFIGURATION SUMMARY" + " " * 22 + "│")
    print("└" + "─" * 63 + "┘\n")

    # Connection info
    print(f"{Colors.BOLD}Connection:{Colors.END}")
    print(f"  {Colors.CYAN}▸{Colors.END} Server:  {Colors.YELLOW}{connection['target_ip']}{Colors.END}")
    print(f"  {Colors.CYAN}▸{Colors.END} User:    {Colors.YELLOW}{connection['target_user']}{Colors.END}")
    print(f"  {Colors.CYAN}▸{Colors.END} SSH Key: {Colors.YELLOW}{connection['ssh_key_path']}{Colors.END}\n")

    # Features
    print(f"{Colors.BOLD}Selected Features:{Colors.END}")

    selected_features = features.get('features', [])
    all_features = {
        'fail2ban': 'Fail2ban Intrusion Prevention',
        'docker': 'Docker & Docker Compose',
        'lemp': 'LEMP Stack',
        'swap': 'Swap Memory Configuration',
        'cron_jobs': 'Automated Cron Jobs',
        'dev_tools': 'Development Tools',
        'wordpress': 'WordPress CMS',
        'certbot': 'Certbot SSL/TLS'
    }

    for key, name in all_features.items():
        if key in selected_features:
            print(f"  {Colors.GREEN}✓{Colors.END} {name}")
        else:
            print(f"  {Colors.RED}✗{Colors.END} {name}")

    print(f"\n  {Colors.YELLOW}▸{Colors.END} Total: {Colors.BOLD}{len(selected_features)}{Colors.END} features enabled\n")


def export_ansible_vars(connection, features):
    """Export selections as Ansible extra vars"""
    selected_features = features.get('features', [])

    vars_dict = {
        'target_ip': connection['target_ip'],
        'target_user': connection['target_user'],
        'ssh_key_path': connection['ssh_key_path'],
        'prompt_enable_fail2ban': 'yes' if 'fail2ban' in selected_features else 'no',
        'prompt_install_docker': 'yes' if 'docker' in selected_features else 'no',
        'prompt_install_lemp': 'yes' if 'lemp' in selected_features else 'no',
        'prompt_enable_swap': 'yes' if 'swap' in selected_features else 'no',
        'prompt_enable_cron_jobs': 'yes' if 'cron_jobs' in selected_features else 'no',
        'prompt_install_dev_tools': 'yes' if 'dev_tools' in selected_features else 'no',
        'prompt_install_wordpress': 'yes' if 'wordpress' in selected_features else 'no',
        'prompt_install_certbot': 'yes' if 'certbot' in selected_features else 'no',
    }

    return vars_dict


def main():
    """Main function"""
    # Check for --export-vars flag
    export_mode = '--export-vars' in sys.argv

    if not export_mode:
        print_header()

    # Get user input
    connection = get_connection_info()
    if not connection:
        sys.exit(1)

    features = get_feature_selection()
    if not features:
        sys.exit(1)

    # Validate
    warnings, errors = validate_selections(connection, features)

    if errors:
        print(f"\n{Colors.RED}{Colors.BOLD}Errors:{Colors.END}")
        for error in errors:
            print(f"  {Colors.RED}✗{Colors.END} {error}")
        print()
        sys.exit(1)

    if warnings and not export_mode:
        print(f"\n{Colors.YELLOW}{Colors.BOLD}Warnings:{Colors.END}")
        for warning in warnings:
            print(f"  {Colors.YELLOW}⚠{Colors.END} {warning}")

    # Display summary
    if not export_mode:
        display_summary(connection, features)

        # Confirm
        confirm = inquirer.confirm("Proceed with this configuration?", default=True)
        if not confirm:
            print(f"\n{Colors.RED}Configuration cancelled.{Colors.END}\n")
            sys.exit(1)

        print(f"\n{Colors.GREEN}{Colors.BOLD}Configuration confirmed!{Colors.END}")
        print(f"\nTo run the playbook with these settings:")
        print(f"  {Colors.CYAN}ansible-playbook playbook.yml -e '{json.dumps(export_ansible_vars(connection, features))}'{Colors.END}\n")
    else:
        # Export mode: output vars for ansible-playbook -e
        print(json.dumps(export_ansible_vars(connection, features)))


if __name__ == '__main__':
    try:
        main()
    except KeyboardInterrupt:
        print(f"\n\n{Colors.RED}Cancelled by user.{Colors.END}\n")
        sys.exit(1)
