#!/bin/bash
# Demo script to show how the beautified Ansible output will appear
# Run with: bash docs/demo_beautified_output.sh

# Color codes
BOLD='\033[1m'
GREEN='\033[1;32m'
RED='\033[1;31m'
YELLOW='\033[1;33m'
CYAN='\033[1;36m'
GRAY='\033[1;90m'
RESET='\033[0m'

# Simulate the playbook output
clear
echo ""
echo "ANSIBLE PLAYBOOK EXECUTION DEMO"
echo "================================="
echo ""
echo "After answering all interactive prompts, you'll see:"
echo ""

# Phase 1: Connection Information
echo "┌─────────────────────────────────────────────────────────────┐"
echo "│           CONNECTION INFORMATION                            │"
echo "└─────────────────────────────────────────────────────────────┘"
echo -e "  Target Server: ${CYAN}192.168.1.100${RESET}"
echo -e "  SSH User:      ${CYAN}root${RESET}"
echo -e "  SSH Key:       ${CYAN}~/.ssh/id_rsa_gitlab${RESET}"
echo ""

# Phase 2: Selected Features
echo "┌─────────────────────────────────────────────────────────────┐"
echo "│           SELECTED FEATURES                                 │"
echo "└─────────────────────────────────────────────────────────────┘"
echo -e "  ${GREEN}✓${RESET} Fail2ban Intrusion Prevention"
echo -e "  ${GREEN}✓${RESET} Docker & Docker Compose"
echo -e "  ${RED}✗${RESET} LEMP Stack (Nginx, MySQL, PHP)"
echo -e "  ${GREEN}✓${RESET} Swap Memory Configuration"
echo -e "  ${GREEN}✓${RESET} Automated Cron Jobs"
echo -e "  ${RED}✗${RESET} Development Tools"
echo -e "  ${RED}✗${RESET} WordPress CMS"
echo -e "  ${RED}✗${RESET} Certbot SSL/TLS Certificates"
echo ""
echo -e "  ${YELLOW}▸${RESET} Total features enabled: ${BOLD}4${RESET}"
echo ""

# Simulate some task execution
echo "... [Ansible tasks executing] ..."
echo ""
sleep 1

# Phase 3: Provisioning Start
echo "╔═════════════════════════════════════════════════════════════╗"
echo "║         UBUNTU SERVER PROVISIONING STARTED                  ║"
echo "╚═════════════════════════════════════════════════════════════╝"
echo -e "  ${CYAN}▸${RESET} Target Host:  ${YELLOW}192.168.1.100${RESET}"
echo -e "  ${CYAN}▸${RESET} OS:           ${YELLOW}Ubuntu 22.04${RESET}"
echo -e "  ${CYAN}▸${RESET} Architecture: ${YELLOW}x86_64${RESET}"
echo ""

# Phase 4: Active Features
echo "┌─────────────────────────────────────────────────────────────┐"
echo "│           ACTIVE FEATURES FOR THIS RUN                      │"
echo "└─────────────────────────────────────────────────────────────┘"
echo -e "  ${GREEN}●${RESET} ${GREEN}ENABLED${RESET}   Fail2ban Intrusion Prevention"
echo -e "  ${GREEN}●${RESET} ${GREEN}ENABLED${RESET}   Docker & Docker Compose"
echo -e "  ${GRAY}○${RESET} ${GRAY}DISABLED${RESET}  LEMP Stack"
echo -e "  ${GREEN}●${RESET} ${GREEN}ENABLED${RESET}   Swap Memory"
echo -e "  ${GREEN}●${RESET} ${GREEN}ENABLED${RESET}   Cron Jobs"
echo -e "  ${GRAY}○${RESET} ${GRAY}DISABLED${RESET}  Development Tools"
echo -e "  ${GRAY}○${RESET} ${GRAY}DISABLED${RESET}  WordPress"
echo -e "  ${GRAY}○${RESET} ${GRAY}DISABLED${RESET}  Certbot"
echo ""

# Simulate more task execution
echo "... [Ansible roles executing] ..."
echo ""
sleep 1

# Phase 5: Completion - No reboot scenario
echo "╔═════════════════════════════════════════════════════════════╗"
echo "║         PROVISIONING COMPLETED SUCCESSFULLY                 ║"
echo "╚═════════════════════════════════════════════════════════════╝"
echo -e "${GREEN}✓${RESET}  System Status: ${GREEN}No reboot required${RESET}"
echo ""

# Show alternative: reboot required
echo "--- Alternative Output (if reboot required) ---"
echo ""
echo "╔═════════════════════════════════════════════════════════════╗"
echo "║         PROVISIONING COMPLETED SUCCESSFULLY                 ║"
echo "╚═════════════════════════════════════════════════════════════╝"
echo -e "${YELLOW}⚠${RESET}  ${YELLOW}WARNING:${RESET} Reboot is REQUIRED (kernel or system updates)"
echo -e "  ${CYAN}▸${RESET} Run with tag ${CYAN}-t reboot${RESET} to reboot now"
echo ""

echo "========================================="
echo "DEMO COMPLETE"
echo ""
echo "To see this in your actual playbook:"
echo "  ansible-playbook playbook.yml"
echo ""
