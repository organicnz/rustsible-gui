#!/bin/bash
# GUI launcher for Ubuntu Server Provisioning
# Shows all options on one screen with visual checkboxes

cd /Users/organic/dev/work/ansible/ansible-ubuntu

# Display ASCII banner
cat << "EOF"

╔══════════════════════════════════════════════════════════════════════╗
║                                                                      ║
║   ██╗   ██╗██████╗ ██╗   ██╗███╗   ██╗████████╗██╗   ██╗          ║
║   ██║   ██║██╔══██╗██║   ██║████╗  ██║╚══██╔══╝██║   ██║          ║
║   ██║   ██║██████╔╝██║   ██║██╔██╗ ██║   ██║   ██║   ██║          ║
║   ██║   ██║██╔══██╗██║   ██║██║╚██╗██║   ██║   ██║   ██║          ║
║   ╚██████╔╝██████╔╝╚██████╔╝██║ ╚████║   ██║   ╚██████╔╝          ║
║    ╚═════╝ ╚═════╝  ╚═════╝ ╚═╝  ╚═══╝   ╚═╝    ╚═════╝           ║
║                                                                      ║
║            🖥️  GRAPHICAL USER INTERFACE LAUNCHER  🖥️              ║
║                                                                      ║
║        Automated Server Provisioning with Visual Controls           ║
║                                                                      ║
╚══════════════════════════════════════════════════════════════════════╝

EOF

echo "🚀 Starting GUI application..."
echo ""

# Use Homebrew Python 3.13 with tkinter support
if [ -f /opt/homebrew/bin/python3.13 ]; then
    echo "Using Python 3.13 with tkinter support..."
    /opt/homebrew/bin/python3.13 run_gui.py
elif [ -f /opt/homebrew/bin/python3 ]; then
    # Try generic Homebrew python3
    /opt/homebrew/bin/python3 run_gui.py
else
    # Fall back to system Python
    echo "WARNING: Using system Python - install python-tk@3.13 for better compatibility"
    python3 run_gui.py
fi
