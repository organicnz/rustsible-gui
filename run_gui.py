#!/usr/bin/env python3
"""
GUI Launcher for Ubuntu Server Provisioning
Shows all options on one screen with checkboxes
"""

import tkinter as tk
from tkinter import ttk, messagebox, scrolledtext
import subprocess
import sys
import os
import json
from pathlib import Path

class ProvisioningGUI:
    def __init__(self, root):
        self.root = root
        self.root.title("Ubuntu Server Provisioning")
        self.root.geometry("900x950")
        self.root.resizable(True, True)

        # Cache file location
        self.cache_file = Path(__file__).parent / '.gui_config_cache.json'

        # Modern color scheme
        self.colors = {
            'bg': '#f5f7fa',
            'fg': '#2c3e50',
            'primary': '#3498db',
            'success': '#27ae60',
            'danger': '#e74c3c',
            'warning': '#f39c12',
            'info': '#16a085',
            'card_bg': '#ffffff',
            'border': '#e1e8ed',
            'text_gray': '#7f8c8d',
            'hover': '#2980b9'
        }

        # Set root background
        self.root.configure(bg=self.colors['bg'])

        # Variables
        self.target_ip = tk.StringVar()
        self.target_user = tk.StringVar(value="root")
        self.ssh_key_path = tk.StringVar(value="~/.ssh/id_rsa_gitlab")

        # Feature checkboxes
        self.features = {
            'fail2ban': tk.BooleanVar(value=True),
            'docker': tk.BooleanVar(value=True),
            'lemp': tk.BooleanVar(value=False),
            'swap': tk.BooleanVar(value=True),
            'cron': tk.BooleanVar(value=True),
            'devtools': tk.BooleanVar(value=False),
            'wordpress': tk.BooleanVar(value=False),
            'certbot': tk.BooleanVar(value=False),
            'system_hardening': tk.BooleanVar(value=True),
            'monitoring_detection': tk.BooleanVar(value=True),
            'network_security': tk.BooleanVar(value=False),
            'advanced_protection': tk.BooleanVar(value=False),
        }

        self.setup_styles()
        self.create_widgets()
        self.load_cache()  # Load cached settings after widgets are created

    def load_cache(self):
        """Load previously saved configuration from cache file"""
        if not self.cache_file.exists():
            return

        try:
            with open(self.cache_file, 'r') as f:
                cache = json.load(f)

            # Load connection info
            if 'target_ip' in cache:
                self.target_ip.set(cache['target_ip'])
            if 'target_user' in cache:
                self.target_user.set(cache['target_user'])
            if 'ssh_key_path' in cache:
                self.ssh_key_path.set(cache['ssh_key_path'])

            # Load feature selections
            if 'features' in cache:
                for feature, value in cache['features'].items():
                    if feature in self.features:
                        self.features[feature].set(value)

        except Exception as e:
            # Silently ignore cache errors
            pass

    def save_cache(self):
        """Save current configuration to cache file"""
        try:
            cache = {
                'target_ip': self.target_ip.get(),
                'target_user': self.target_user.get(),
                'ssh_key_path': self.ssh_key_path.get(),
                'features': {
                    name: var.get()
                    for name, var in self.features.items()
                }
            }

            with open(self.cache_file, 'w') as f:
                json.dump(cache, f, indent=2)

        except Exception as e:
            # Silently ignore cache save errors
            pass

    def setup_styles(self):
        """Configure modern ttk styles"""
        style = ttk.Style()

        # Configure main frame style
        style.configure('Main.TFrame', background=self.colors['bg'])

        # Configure card-like label frames
        style.configure('Card.TLabelframe',
                       background=self.colors['card_bg'],
                       borderwidth=2,
                       relief='flat')
        style.configure('Card.TLabelframe.Label',
                       background=self.colors['card_bg'],
                       foreground=self.colors['primary'],
                       font=('SF Pro Display', 13, 'bold'))

        # Configure labels
        style.configure('Title.TLabel',
                       background=self.colors['bg'],
                       foreground=self.colors['fg'],
                       font=('SF Pro Display', 24, 'bold'))
        style.configure('Subtitle.TLabel',
                       background=self.colors['card_bg'],
                       foreground=self.colors['text_gray'],
                       font=('SF Pro Text', 10))
        style.configure('Label.TLabel',
                       background=self.colors['card_bg'],
                       foreground=self.colors['fg'],
                       font=('SF Pro Text', 11))

        # Configure checkbuttons
        style.configure('TCheckbutton',
                       background=self.colors['card_bg'],
                       foreground=self.colors['fg'],
                       font=('SF Pro Text', 11))

        # Configure entries
        style.configure('TEntry',
                       fieldbackground='white',
                       font=('SF Mono', 11))

        # Configure primary button
        style.configure('Primary.TButton',
                       font=('SF Pro Text', 12, 'bold'),
                       padding=(20, 10))
        style.map('Primary.TButton',
                 background=[('active', self.colors['hover'])])

        # Configure secondary button
        style.configure('Secondary.TButton',
                       font=('SF Pro Text', 11),
                       padding=(15, 8))

    def create_widgets(self):
        # Main container with scrollbar
        main_frame = ttk.Frame(self.root, padding="20", style='Main.TFrame')
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))

        self.root.columnconfigure(0, weight=1)
        self.root.rowconfigure(0, weight=1)

        # Canvas for scrolling with modern styling
        canvas = tk.Canvas(main_frame, bg=self.colors['bg'], highlightthickness=0)
        scrollbar = ttk.Scrollbar(main_frame, orient="vertical", command=canvas.yview)
        scrollable_frame = ttk.Frame(canvas, style='Main.TFrame')

        scrollable_frame.bind(
            "<Configure>",
            lambda e: canvas.configure(scrollregion=canvas.bbox("all"))
        )

        canvas.create_window((0, 0), window=scrollable_frame, anchor="nw")
        canvas.configure(yscrollcommand=scrollbar.set)

        # Modern header with subtitle
        header_frame = ttk.Frame(scrollable_frame, style='Main.TFrame')
        header_frame.grid(row=0, column=0, columnspan=2, pady=(0, 20))

        title = ttk.Label(header_frame,
                         text="🖥️  Ubuntu Server Provisioning",
                         style='Title.TLabel')
        title.pack()

        subtitle = tk.Label(header_frame,
                           text="Configure and deploy your server with one click",
                           bg=self.colors['bg'],
                           fg=self.colors['text_gray'],
                           font=('SF Pro Text', 11))
        subtitle.pack(pady=(5, 0))

        # Connection Information Card
        conn_frame = ttk.LabelFrame(scrollable_frame,
                                    text="  Connection Information  ",
                                    padding="20",
                                    style='Card.TLabelframe')
        conn_frame.grid(row=1, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, 15))

        # Server IP
        ttk.Label(conn_frame, text="🖥️  Server IP Address:", style='Label.TLabel').grid(
            row=0, column=0, sticky=tk.W, pady=8)
        ip_entry = ttk.Entry(conn_frame, textvariable=self.target_ip, width=45, font=('SF Mono', 11))
        ip_entry.grid(row=0, column=1, sticky=(tk.W, tk.E), padx=(10, 0))

        # SSH Username
        ttk.Label(conn_frame, text="👤 SSH Username:", style='Label.TLabel').grid(
            row=1, column=0, sticky=tk.W, pady=8)
        user_entry = ttk.Entry(conn_frame, textvariable=self.target_user, width=45, font=('SF Mono', 11))
        user_entry.grid(row=1, column=1, sticky=(tk.W, tk.E), padx=(10, 0))

        # SSH Key Path
        ttk.Label(conn_frame, text="🔑 SSH Private Key:", style='Label.TLabel').grid(
            row=2, column=0, sticky=tk.W, pady=8)
        key_entry = ttk.Entry(conn_frame, textvariable=self.ssh_key_path, width=45, font=('SF Mono', 11))
        key_entry.grid(row=2, column=1, sticky=(tk.W, tk.E), padx=(10, 0))

        conn_frame.columnconfigure(1, weight=1)

        # Core Features Card
        core_frame = ttk.LabelFrame(scrollable_frame,
                                    text="  Core Features  ",
                                    padding="20",
                                    style='Card.TLabelframe')
        core_frame.grid(row=2, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, 15))

        ttk.Checkbutton(core_frame, text="🛡️  Fail2ban Intrusion Prevention",
                       variable=self.features['fail2ban']).grid(row=0, column=0, sticky=tk.W, pady=6)
        ttk.Checkbutton(core_frame, text="🐳 Docker & Docker Compose",
                       variable=self.features['docker']).grid(row=1, column=0, sticky=tk.W, pady=6)
        ttk.Checkbutton(core_frame, text="🌐 LEMP Stack (Nginx, MySQL, PHP)",
                       variable=self.features['lemp']).grid(row=2, column=0, sticky=tk.W, pady=6)
        ttk.Checkbutton(core_frame, text="💾 Swap Memory Configuration",
                       variable=self.features['swap']).grid(row=3, column=0, sticky=tk.W, pady=6)
        ttk.Checkbutton(core_frame, text="⏰ Automated Cron Jobs",
                       variable=self.features['cron']).grid(row=4, column=0, sticky=tk.W, pady=6)
        ttk.Checkbutton(core_frame, text="⚙️  Development Tools (Neovim, Node.js, Claude Code)",
                       variable=self.features['devtools']).grid(row=5, column=0, sticky=tk.W, pady=6)
        ttk.Checkbutton(core_frame, text="📝 WordPress CMS",
                       variable=self.features['wordpress']).grid(row=6, column=0, sticky=tk.W, pady=6)
        ttk.Checkbutton(core_frame, text="🔒 Certbot SSL/TLS Certificates",
                       variable=self.features['certbot']).grid(row=7, column=0, sticky=tk.W, pady=6)

        # Security Clusters Card
        security_frame = ttk.LabelFrame(scrollable_frame,
                                       text="  Security Clusters  ",
                                       padding="20",
                                       style='Card.TLabelframe')
        security_frame.grid(row=3, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, 15))

        # System Hardening
        sh_check = ttk.Checkbutton(security_frame, text="🔐 System Hardening",
                                   variable=self.features['system_hardening'])
        sh_check.grid(row=0, column=0, sticky=tk.W, pady=(6, 2))
        sh_label = ttk.Label(security_frame,
                            text="      • Kernel Hardening, AppArmor, Auto-updates",
                            style='Subtitle.TLabel')
        sh_label.grid(row=1, column=0, sticky=tk.W, padx=0, pady=(0, 6))

        # Monitoring & Detection
        md_check = ttk.Checkbutton(security_frame, text="📊 Monitoring & Detection",
                                   variable=self.features['monitoring_detection'])
        md_check.grid(row=2, column=0, sticky=tk.W, pady=(6, 2))
        md_label = ttk.Label(security_frame,
                            text="      • Lynis, AIDE, rkhunter, auditd, Logwatch",
                            style='Subtitle.TLabel')
        md_label.grid(row=3, column=0, sticky=tk.W, padx=0, pady=(0, 6))

        # Network Security
        ns_check = ttk.Checkbutton(security_frame, text="🌐 Network Security",
                                   variable=self.features['network_security'])
        ns_check.grid(row=4, column=0, sticky=tk.W, pady=(6, 2))
        ns_label = ttk.Label(security_frame,
                            text="      • IPv6 disable, Network IDS (Suricata)",
                            style='Subtitle.TLabel')
        ns_label.grid(row=5, column=0, sticky=tk.W, padx=0, pady=(0, 6))

        # Advanced Protection
        ap_check = ttk.Checkbutton(security_frame, text="🔑 Advanced Protection",
                                   variable=self.features['advanced_protection'])
        ap_check.grid(row=6, column=0, sticky=tk.W, pady=(6, 2))
        ap_label = ttk.Label(security_frame,
                            text="      • 2FA, Backups, USB restrictions",
                            style='Subtitle.TLabel')
        ap_label.grid(row=7, column=0, sticky=tk.W, padx=0, pady=(0, 6))

        # Action Buttons
        button_frame = ttk.Frame(scrollable_frame, style='Main.TFrame')
        button_frame.grid(row=4, column=0, columnspan=2, pady=25)

        # Primary launch button with modern styling
        launch_btn = tk.Button(button_frame,
                              text="🚀  Launch Provisioning",
                              command=self.launch_provisioning,
                              bg=self.colors['success'],
                              fg='white',
                              font=('SF Pro Text', 13, 'bold'),
                              padx=30,
                              pady=12,
                              relief='flat',
                              cursor='hand2',
                              activebackground=self.colors['info'],
                              activeforeground='white')
        launch_btn.pack(side=tk.LEFT, padx=8)

        # Bind hover effects for launch button
        launch_btn.bind('<Enter>', lambda e: launch_btn.config(bg=self.colors['info']))
        launch_btn.bind('<Leave>', lambda e: launch_btn.config(bg=self.colors['success']))

        # Cancel button with subtle styling
        cancel_btn = tk.Button(button_frame,
                              text="✕  Cancel",
                              command=self.root.quit,
                              bg=self.colors['card_bg'],
                              fg=self.colors['text_gray'],
                              font=('SF Pro Text', 12),
                              padx=25,
                              pady=12,
                              relief='flat',
                              cursor='hand2',
                              activebackground=self.colors['border'],
                              activeforeground=self.colors['fg'],
                              bd=1,
                              highlightthickness=1,
                              highlightbackground=self.colors['border'])
        cancel_btn.pack(side=tk.LEFT, padx=8)

        # Bind hover effects for cancel button
        cancel_btn.bind('<Enter>', lambda e: cancel_btn.config(bg=self.colors['border']))
        cancel_btn.bind('<Leave>', lambda e: cancel_btn.config(bg=self.colors['card_bg']))

        # Cache indicator footer
        footer_frame = ttk.Frame(scrollable_frame, style='Main.TFrame')
        footer_frame.grid(row=5, column=0, columnspan=2, pady=(10, 0))

        cache_label = tk.Label(footer_frame,
                              text="💾 Settings are automatically saved",
                              bg=self.colors['bg'],
                              fg=self.colors['text_gray'],
                              font=('SF Pro Text', 10, 'italic'))
        cache_label.pack()

        # Pack canvas and scrollbar
        canvas.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        scrollbar.grid(row=0, column=1, sticky=(tk.N, tk.S))

        main_frame.columnconfigure(0, weight=1)
        main_frame.rowconfigure(0, weight=1)

    def launch_provisioning(self):
        # Validate inputs
        if not self.target_ip.get():
            messagebox.showerror("Error", "Please enter a server IP address")
            return

        # Build command
        cmd = [
            'ansible-playbook',
            'playbook.yml',
            '-e', f"target_ip={self.target_ip.get()}",
            '-e', f"target_user={self.target_user.get()}",
            '-e', f"ssh_key_path={self.ssh_key_path.get()}",
            '-e', f"prompt_enable_fail2ban={'yes' if self.features['fail2ban'].get() else 'no'}",
            '-e', f"prompt_install_docker={'yes' if self.features['docker'].get() else 'no'}",
            '-e', f"prompt_install_lemp={'yes' if self.features['lemp'].get() else 'no'}",
            '-e', f"prompt_enable_swap={'yes' if self.features['swap'].get() else 'no'}",
            '-e', f"prompt_enable_cron_jobs={'yes' if self.features['cron'].get() else 'no'}",
            '-e', f"prompt_install_dev_tools={'yes' if self.features['devtools'].get() else 'no'}",
            '-e', f"prompt_install_wordpress={'yes' if self.features['wordpress'].get() else 'no'}",
            '-e', f"prompt_install_certbot={'yes' if self.features['certbot'].get() else 'no'}",
        ]

        # Add security clusters
        if self.features['system_hardening'].get():
            cmd.extend([
                '-e', 'enable_kernel_hardening=true',
                '-e', 'enable_apparmor=true',
                '-e', 'enable_secure_shm=true',
                '-e', 'enable_unattended_upgrades=true',
            ])

        if self.features['monitoring_detection'].get():
            cmd.extend([
                '-e', 'enable_lynis=true',
                '-e', 'enable_rkhunter=true',
                '-e', 'enable_aide=true',
                '-e', 'enable_auditd=true',
                '-e', 'enable_logwatch=true',
            ])

        if self.features['network_security'].get():
            cmd.extend([
                '-e', 'disable_ipv6=false',
                '-e', 'enable_suricata=false',
            ])

        if self.features['advanced_protection'].get():
            cmd.extend([
                '-e', 'enable_ssh_2fa=false',
                '-e', 'enable_backups=false',
                '-e', 'enable_usb_restrictions=false',
            ])

        # Validate prerequisites
        if self.features['wordpress'].get() and not self.features['lemp'].get():
            messagebox.showerror("Error", "WordPress requires LEMP stack to be enabled")
            return

        # Show confirmation
        selected = [name for name, var in self.features.items() if var.get()]
        msg = f"Server: {self.target_ip.get()}\n"
        msg += f"User: {self.target_user.get()}\n"
        msg += f"Features: {len(selected)} selected\n\n"
        msg += "Launch provisioning?"

        if not messagebox.askyesno("Confirm", msg):
            return

        # Save configuration to cache
        self.save_cache()

        # Hide main window
        self.root.withdraw()

        # Create modern terminal window to show output
        terminal_window = tk.Toplevel(self.root)
        terminal_window.title("🚀 Provisioning Progress")
        terminal_window.geometry("1000x650")
        terminal_window.configure(bg=self.colors['bg'])

        # Header for terminal window
        header = tk.Frame(terminal_window, bg=self.colors['primary'], height=50)
        header.pack(fill=tk.X, pady=(0, 10))
        header.pack_propagate(False)

        tk.Label(header,
                text="📊 Live Ansible Provisioning Output",
                bg=self.colors['primary'],
                fg='white',
                font=('SF Pro Display', 14, 'bold')).pack(pady=15)

        # Terminal text area with modern styling
        text_area = scrolledtext.ScrolledText(terminal_window,
                                              wrap=tk.WORD,
                                              font=('SF Mono', 10),
                                              bg='#1e1e1e',
                                              fg='#d4d4d4',
                                              insertbackground='white',
                                              selectbackground=self.colors['primary'],
                                              selectforeground='white',
                                              padx=15,
                                              pady=15)
        text_area.pack(fill=tk.BOTH, expand=True, padx=15, pady=(0, 15))

        # Run command
        try:
            process = subprocess.Popen(cmd, stdout=subprocess.PIPE,
                                     stderr=subprocess.STDOUT,
                                     text=True, bufsize=1)

            for line in process.stdout:
                text_area.insert(tk.END, line)
                text_area.see(tk.END)
                terminal_window.update()

            process.wait()

            if process.returncode == 0:
                messagebox.showinfo("Success", "Provisioning completed successfully!")
            else:
                messagebox.showerror("Error", f"Provisioning failed with exit code {process.returncode}")

        except Exception as e:
            messagebox.showerror("Error", f"Failed to run playbook: {str(e)}")

        finally:
            terminal_window.destroy()
            self.root.deiconify()

def main():
    root = tk.Tk()

    # Center window on screen
    app = ProvisioningGUI(root)

    # Center the window
    root.update_idletasks()
    width = root.winfo_width()
    height = root.winfo_height()
    x = (root.winfo_screenwidth() // 2) - (width // 2)
    y = (root.winfo_screenheight() // 2) - (height // 2)
    root.geometry(f'{width}x{height}+{x}+{y}')

    root.mainloop()

if __name__ == '__main__':
    main()
