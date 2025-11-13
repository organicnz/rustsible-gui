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

        # Modern dark mode color scheme with extended palette
        self.colors = {
            'bg': '#1a1d23',           # Dark background
            'fg': '#e8eaed',           # Light text
            'primary': '#5dade2',      # Bright blue
            'primary_hover': '#3498db', # Primary hover
            'success': '#2ecc71',      # Bright green
            'success_hover': '#27ae60', # Success hover
            'danger': '#e74c3c',       # Red
            'danger_hover': '#c0392b',  # Danger hover
            'warning': '#f39c12',      # Orange
            'warning_hover': '#e67e22', # Warning hover
            'info': '#1abc9c',         # Teal
            'card_bg': '#252930',      # Dark card background
            'card_hover': '#2d323a',   # Card hover
            'border': '#3a3f4b',       # Dark border
            'border_light': '#4a505f', # Lighter border
            'text_gray': '#a0a6b1',    # Light gray text
            'text_dim': '#6b7280',     # Dimmed text
            'input_bg': '#2a2e38',     # Input background
            'input_focus': '#323844',  # Input focus
            'hover': '#3498db',        # Hover blue
            'accent': '#9b59b6',       # Purple accent
        }

        # Spacing scale (8px base)
        self.spacing = {
            'xs': 4,
            'sm': 8,
            'md': 16,
            'lg': 24,
            'xl': 32,
            'xxl': 48
        }

        # Set root background
        self.root.configure(bg=self.colors['bg'])

        # Variables
        self.target_ip = tk.StringVar()
        self.target_user = tk.StringVar(value="root")
        self.ssh_key_path = tk.StringVar(value="~/.ssh/id_rsa_gitlab")
        self.hostname = tk.StringVar()

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
            'periodic_reboot': tk.BooleanVar(value=False),
        }

        # Periodic reboot configuration
        self.reboot_hour = tk.StringVar(value="3")

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
            if 'hostname' in cache:
                self.hostname.set(cache['hostname'])

            # Load feature selections
            if 'features' in cache:
                for feature, value in cache['features'].items():
                    if feature in self.features:
                        self.features[feature].set(value)

            # Load reboot hour configuration
            if 'reboot_hour' in cache:
                self.reboot_hour.set(cache['reboot_hour'])

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
                'hostname': self.hostname.get(),
                'features': {
                    name: var.get()
                    for name, var in self.features.items()
                },
                'reboot_hour': self.reboot_hour.get()
            }

            with open(self.cache_file, 'w') as f:
                json.dump(cache, f, indent=2)

        except Exception as e:
            # Silently ignore cache save errors
            pass

    def setup_styles(self):
        """Configure modern ttk styles with best practices"""
        style = ttk.Style()

        # Configure main frame style
        style.configure('Main.TFrame', background=self.colors['bg'])

        # Configure card-like label frames with better styling
        style.configure('Card.TLabelframe',
                       background=self.colors['card_bg'],
                       borderwidth=1,
                       relief='solid',
                       bordercolor=self.colors['border_light'])
        style.configure('Card.TLabelframe.Label',
                       background=self.colors['card_bg'],
                       foreground=self.colors['primary'],
                       font=('SF Pro Display', 14, 'bold'),
                       padding=(self.spacing['sm'], self.spacing['xs']))

        # Configure labels with better hierarchy
        style.configure('Title.TLabel',
                       background=self.colors['bg'],
                       foreground=self.colors['fg'],
                       font=('SF Pro Display', 26, 'bold'))
        style.configure('Subtitle.TLabel',
                       background=self.colors['bg'],
                       foreground=self.colors['text_gray'],
                       font=('SF Pro Text', 12))
        style.configure('Label.TLabel',
                       background=self.colors['card_bg'],
                       foreground=self.colors['fg'],
                       font=('SF Pro Text', 11, 'normal'))
        style.configure('SectionLabel.TLabel',
                       background=self.colors['card_bg'],
                       foreground=self.colors['text_dim'],
                       font=('SF Pro Text', 10, 'bold'))

        # Configure checkbuttons with better spacing
        style.configure('TCheckbutton',
                       background=self.colors['card_bg'],
                       foreground=self.colors['fg'],
                       font=('SF Pro Text', 11),
                       padding=self.spacing['xs'])
        style.map('TCheckbutton',
                 background=[('active', self.colors['card_hover'])],
                 foreground=[('active', self.colors['fg'])])

        # Configure entries with better styling
        style.configure('TEntry',
                       fieldbackground=self.colors['input_bg'],
                       foreground=self.colors['fg'],
                       font=('SF Mono', 11),
                       borderwidth=1,
                       relief='solid',
                       padding=self.spacing['sm'])
        style.map('TEntry',
                 fieldbackground=[('focus', self.colors['input_focus'])],
                 bordercolor=[('focus', self.colors['primary'])])

        # Configure primary button with enhanced styling
        style.configure('Primary.TButton',
                       font=('SF Pro Text', 13, 'bold'),
                       padding=(self.spacing['lg'], self.spacing['md']),
                       borderwidth=0,
                       relief='flat')
        style.map('Primary.TButton',
                 background=[('active', self.colors['primary_hover']),
                           ('pressed', self.colors['primary'])],
                 foreground=[('active', 'white')])

        # Configure secondary button
        style.configure('Secondary.TButton',
                       font=('SF Pro Text', 11),
                       padding=(self.spacing['md'], self.spacing['sm']),
                       borderwidth=1,
                       relief='solid')
        style.map('Secondary.TButton',
                 background=[('active', self.colors['card_hover'])],
                 bordercolor=[('active', self.colors['border_light'])])

    def create_widgets(self):
        # Main container with scrollbar
        main_frame = ttk.Frame(self.root, padding=str(self.spacing['lg']), style='Main.TFrame')
        main_frame.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))

        self.root.columnconfigure(0, weight=1)
        self.root.rowconfigure(0, weight=1)

        # Canvas for scrolling with modern styling (store as instance variable)
        self.canvas = tk.Canvas(main_frame, bg=self.colors['bg'], highlightthickness=0)
        scrollbar = ttk.Scrollbar(main_frame, orient="vertical", command=self.canvas.yview)
        scrollable_frame = ttk.Frame(self.canvas, style='Main.TFrame')

        scrollable_frame.bind(
            "<Configure>",
            lambda e: self.canvas.configure(scrollregion=self.canvas.bbox("all"))
        )

        self.canvas.create_window((0, 0), window=scrollable_frame, anchor="nw")
        self.canvas.configure(yscrollcommand=scrollbar.set)

        # Modern header with enhanced styling
        header_frame = ttk.Frame(scrollable_frame, style='Main.TFrame')
        header_frame.grid(row=0, column=0, columnspan=2, pady=(0, self.spacing['xl']))

        title = ttk.Label(header_frame,
                         text="🖥️  Ubuntu Server Provisioning",
                         style='Title.TLabel')
        title.pack()

        subtitle = ttk.Label(header_frame,
                           text="Configure and deploy your server with one click",
                           style='Subtitle.TLabel')
        subtitle.pack(pady=(self.spacing['sm'], 0))

        # Divider line
        divider = tk.Frame(header_frame, bg=self.colors['border'], height=1)
        divider.pack(fill=tk.X, pady=(self.spacing['md'], 0))

        # Connection Information Card with enhanced styling
        conn_frame = ttk.LabelFrame(scrollable_frame,
                                    text="  📡 Connection Information  ",
                                    padding=str(self.spacing['lg']),
                                    style='Card.TLabelframe')
        conn_frame.grid(row=1, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, self.spacing['md']))

        # Server IP
        ttk.Label(conn_frame, text="🖥️  Server IP Address:", style='Label.TLabel').grid(
            row=0, column=0, sticky=tk.W, pady=self.spacing['sm'])
        ip_entry = ttk.Entry(conn_frame, textvariable=self.target_ip, width=45, font=('SF Mono', 11))
        ip_entry.grid(row=0, column=1, sticky=(tk.W, tk.E), padx=(self.spacing['md'], 0))

        # SSH Username
        ttk.Label(conn_frame, text="👤 SSH Username:", style='Label.TLabel').grid(
            row=1, column=0, sticky=tk.W, pady=self.spacing['sm'])
        user_entry = ttk.Entry(conn_frame, textvariable=self.target_user, width=45, font=('SF Mono', 11))
        user_entry.grid(row=1, column=1, sticky=(tk.W, tk.E), padx=(self.spacing['md'], 0))

        # SSH Key Path
        ttk.Label(conn_frame, text="🔑 SSH Private Key:", style='Label.TLabel').grid(
            row=2, column=0, sticky=tk.W, pady=self.spacing['sm'])
        key_entry = ttk.Entry(conn_frame, textvariable=self.ssh_key_path, width=45, font=('SF Mono', 11))
        key_entry.grid(row=2, column=1, sticky=(tk.W, tk.E), padx=(self.spacing['md'], 0))

        # Hostname
        ttk.Label(conn_frame, text="🏷️  Hostname (optional):", style='Label.TLabel').grid(
            row=3, column=0, sticky=tk.W, pady=self.spacing['sm'])
        hostname_entry = ttk.Entry(conn_frame, textvariable=self.hostname, width=45, font=('SF Mono', 11))
        hostname_entry.grid(row=3, column=1, sticky=(tk.W, tk.E), padx=(self.spacing['md'], 0))

        conn_frame.columnconfigure(1, weight=1)

        # Core Features Card with enhanced styling
        core_frame = ttk.LabelFrame(scrollable_frame,
                                    text="  ⚡ Core Features  ",
                                    padding=str(self.spacing['lg']),
                                    style='Card.TLabelframe')
        core_frame.grid(row=2, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, self.spacing['md']))

        ttk.Checkbutton(core_frame, text="🛡️  Fail2ban Intrusion Prevention",
                       variable=self.features['fail2ban']).grid(row=0, column=0, sticky=tk.W, pady=self.spacing['xs'])
        ttk.Checkbutton(core_frame, text="🐳 Docker & Docker Compose",
                       variable=self.features['docker']).grid(row=1, column=0, sticky=tk.W, pady=self.spacing['xs'])
        ttk.Checkbutton(core_frame, text="🌐 LEMP Stack (Nginx, MySQL, PHP)",
                       variable=self.features['lemp']).grid(row=2, column=0, sticky=tk.W, pady=self.spacing['xs'])
        ttk.Checkbutton(core_frame, text="💾 Swap Memory Configuration",
                       variable=self.features['swap']).grid(row=3, column=0, sticky=tk.W, pady=self.spacing['xs'])
        ttk.Checkbutton(core_frame, text="⏰ Automated Cron Jobs",
                       variable=self.features['cron']).grid(row=4, column=0, sticky=tk.W, pady=self.spacing['xs'])
        ttk.Checkbutton(core_frame, text="⚙️  Development Tools (Neovim, Node.js, Claude Code)",
                       variable=self.features['devtools']).grid(row=5, column=0, sticky=tk.W, pady=self.spacing['xs'])
        ttk.Checkbutton(core_frame, text="📝 WordPress CMS",
                       variable=self.features['wordpress']).grid(row=6, column=0, sticky=tk.W, pady=self.spacing['xs'])
        ttk.Checkbutton(core_frame, text="🔒 Certbot SSL/TLS Certificates",
                       variable=self.features['certbot']).grid(row=7, column=0, sticky=tk.W, pady=self.spacing['xs'])

        # Security Clusters Card with enhanced styling
        security_frame = ttk.LabelFrame(scrollable_frame,
                                       text="  🔒 Security Clusters  ",
                                       padding=str(self.spacing['lg']),
                                       style='Card.TLabelframe')
        security_frame.grid(row=3, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, self.spacing['md']))

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

        # Maintenance Settings Card with enhanced styling
        maint_frame = ttk.LabelFrame(scrollable_frame,
                                     text="  🔧 Maintenance Settings  ",
                                     padding=str(self.spacing['lg']),
                                     style='Card.TLabelframe')
        maint_frame.grid(row=4, column=0, columnspan=2, sticky=(tk.W, tk.E), pady=(0, self.spacing['md']))

        # Periodic Reboot checkbox
        reboot_check = ttk.Checkbutton(maint_frame, text="🔄 Periodic System Reboot",
                                       variable=self.features['periodic_reboot'],
                                       command=self.toggle_reboot_config)
        reboot_check.grid(row=0, column=0, sticky=tk.W, pady=(0, self.spacing['sm']))

        # Warning label
        warning_label = ttk.Label(maint_frame,
                                 text="      ⚠️  Warning: Automatically reboots server at scheduled time",
                                 style='Subtitle.TLabel')
        warning_label.grid(row=1, column=0, sticky=tk.W, pady=(0, self.spacing['sm']))

        # Reboot hour configuration frame
        reboot_config_frame = ttk.Frame(maint_frame, style='Card.TLabelframe')
        reboot_config_frame.grid(row=2, column=0, sticky=tk.W, pady=(self.spacing['sm'], 0))

        ttk.Label(reboot_config_frame, text="      ⏰ Reboot Schedule:", style='Label.TLabel').grid(
            row=0, column=0, sticky=tk.W, padx=(0, self.spacing['md']))

        # Dropdown for reboot hour
        self.reboot_hour_combo = ttk.Combobox(reboot_config_frame,
                                              textvariable=self.reboot_hour,
                                              values=["1", "2", "3", "4", "5", "*/6", "*/12", "*/24"],
                                              width=15,
                                              state='readonly',
                                              font=('SF Mono', 11))
        self.reboot_hour_combo.grid(row=0, column=1, sticky=tk.W)

        ttk.Label(reboot_config_frame, text=" hour(s)", style='Label.TLabel').grid(
            row=0, column=2, sticky=tk.W, padx=(self.spacing['xs'], 0))

        # Schedule description
        self.reboot_desc_label = ttk.Label(maint_frame,
                                          text="      • Daily at 3:00 AM",
                                          style='Subtitle.TLabel')
        self.reboot_desc_label.grid(row=3, column=0, sticky=tk.W, pady=(self.spacing['xs'], 0))

        # Bind reboot hour change to update description
        self.reboot_hour.trace_add('write', self.update_reboot_description)

        # Initially disable/enable based on checkbox state
        self.toggle_reboot_config()

        # Action Buttons with enhanced styling
        button_frame = ttk.Frame(scrollable_frame, style='Main.TFrame')
        button_frame.grid(row=5, column=0, columnspan=2, pady=(self.spacing['xl'], self.spacing['lg']))

        # Primary launch button with gradient-like effect
        launch_btn = tk.Button(button_frame,
                              text="🚀  Launch Provisioning",
                              command=self.launch_provisioning,
                              bg=self.colors['success'],
                              fg='white',
                              font=('SF Pro Text', 14, 'bold'),
                              padx=self.spacing['xl'],
                              pady=self.spacing['md'],
                              relief='flat',
                              cursor='hand2',
                              borderwidth=0,
                              highlightthickness=0,
                              activebackground=self.colors['success_hover'],
                              activeforeground='white')
        launch_btn.pack(side=tk.LEFT, padx=self.spacing['sm'])

        # Secondary exit button
        exit_btn = tk.Button(button_frame,
                            text="✕  Exit",
                            command=self.root.quit,
                            bg=self.colors['card_bg'],
                            fg=self.colors['text_gray'],
                            font=('SF Pro Text', 12),
                            padx=self.spacing['lg'],
                            pady=self.spacing['sm'],
                            relief='solid',
                            borderwidth=1,
                            cursor='hand2',
                            highlightthickness=0,
                            activebackground=self.colors['card_hover'],
                            activeforeground=self.colors['fg'])
        exit_btn.pack(side=tk.LEFT, padx=self.spacing['sm'])

        # Bind hover effects with smooth transitions
        def on_launch_enter(e):
            launch_btn.config(bg=self.colors['success_hover'])
        def on_launch_leave(e):
            launch_btn.config(bg=self.colors['success'])
        def on_exit_enter(e):
            exit_btn.config(bg=self.colors['card_hover'], fg=self.colors['fg'])
        def on_exit_leave(e):
            exit_btn.config(bg=self.colors['card_bg'], fg=self.colors['text_gray'])

        launch_btn.bind('<Enter>', on_launch_enter)
        launch_btn.bind('<Leave>', on_launch_leave)
        exit_btn.bind('<Enter>', on_exit_enter)
        exit_btn.bind('<Leave>', on_exit_leave)

        # Footer with enhanced styling
        footer_frame = ttk.Frame(scrollable_frame, style='Main.TFrame')
        footer_frame.grid(row=6, column=0, columnspan=2, pady=(self.spacing['lg'], 0))

        # Divider above footer
        footer_divider = tk.Frame(footer_frame, bg=self.colors['border'], height=1)
        footer_divider.pack(fill=tk.X, pady=(0, self.spacing['md']))

        cache_label = tk.Label(footer_frame,
                              text="💾  Settings are automatically saved and restored",
                              bg=self.colors['bg'],
                              fg=self.colors['text_dim'],
                              font=('SF Pro Text', 10))
        cache_label.pack()

        # Pack canvas and scrollbar
        self.canvas.grid(row=0, column=0, sticky=(tk.W, tk.E, tk.N, tk.S))
        scrollbar.grid(row=0, column=1, sticky=(tk.N, tk.S))

        main_frame.columnconfigure(0, weight=1)
        main_frame.rowconfigure(0, weight=1)

        # Enable trackpad/mousewheel scrolling for all platforms
        self._bind_mousewheel(self.canvas)

    def _bind_mousewheel(self, widget):
        """Bind mousewheel/trackpad scrolling events to the canvas"""
        import platform

        def on_mousewheel(event):
            """Handle mousewheel and trackpad scrolling"""
            # macOS uses event.delta directly (typically ±1 for trackpad)
            # Windows uses larger values (±120)
            self.canvas.yview_scroll(int(-1 * event.delta), "units")

        def on_linux_scroll_up(event):
            """Handle Linux scroll up"""
            self.canvas.yview_scroll(-1, "units")

        def on_linux_scroll_down(event):
            """Handle Linux scroll down"""
            self.canvas.yview_scroll(1, "units")

        # Detect platform
        system = platform.system()

        if system == "Darwin":  # macOS
            # macOS requires binding to the widget and all its children
            widget.bind("<MouseWheel>", on_mousewheel)
            self.root.bind("<MouseWheel>", on_mousewheel)
            # Also bind to all child widgets
            def bind_to_children(w):
                w.bind("<MouseWheel>", on_mousewheel, add="+")
                for child in w.winfo_children():
                    bind_to_children(child)
            # Bind after widget is fully created
            self.root.after(100, lambda: bind_to_children(self.root))
        elif system == "Windows":
            self.root.bind("<MouseWheel>", on_mousewheel)
        else:  # Linux
            widget.bind("<Button-4>", on_linux_scroll_up)
            widget.bind("<Button-5>", on_linux_scroll_down)
            self.root.bind("<Button-4>", on_linux_scroll_up)
            self.root.bind("<Button-5>", on_linux_scroll_down)

    def toggle_reboot_config(self):
        """Enable/disable reboot hour selection based on checkbox state"""
        if self.features['periodic_reboot'].get():
            self.reboot_hour_combo.config(state='readonly')
        else:
            self.reboot_hour_combo.config(state='disabled')

    def update_reboot_description(self, *args):
        """Update the reboot schedule description based on selected hour"""
        hour = self.reboot_hour.get()
        descriptions = {
            "1": "Daily at 1:00 AM",
            "2": "Daily at 2:00 AM",
            "3": "Daily at 3:00 AM",
            "4": "Daily at 4:00 AM",
            "5": "Daily at 5:00 AM",
            "*/6": "Every 6 hours",
            "*/12": "Every 12 hours (twice daily)",
            "*/24": "Every 24 hours (once daily)",
        }
        desc = descriptions.get(hour, f"Custom schedule: hour {hour}")
        self.reboot_desc_label.config(text=f"      • {desc}")

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
            '-e', f"target_hostname={self.hostname.get()}",
            '-e', f"prompt_enable_fail2ban={'yes' if self.features['fail2ban'].get() else 'no'}",
            '-e', f"prompt_install_docker={'yes' if self.features['docker'].get() else 'no'}",
            '-e', f"prompt_install_lemp={'yes' if self.features['lemp'].get() else 'no'}",
            '-e', f"prompt_enable_swap={'yes' if self.features['swap'].get() else 'no'}",
            '-e', f"prompt_enable_cron_jobs={'yes' if self.features['cron'].get() else 'no'}",
            '-e', f"prompt_install_dev_tools={'yes' if self.features['devtools'].get() else 'no'}",
            '-e', f"prompt_install_wordpress={'yes' if self.features['wordpress'].get() else 'no'}",
            '-e', f"prompt_install_certbot={'yes' if self.features['certbot'].get() else 'no'}",
            '-e', f"prompt_enable_periodic_reboot={'yes' if self.features['periodic_reboot'].get() else 'no'}",
            '-e', f"prompt_reboot_hour={self.reboot_hour.get()}",
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

            terminal_window.destroy()

            if process.returncode == 0:
                self.show_success_dialog()
            else:
                self.show_error_dialog(process.returncode)

        except Exception as e:
            terminal_window.destroy()
            self.show_exception_dialog(str(e))

        finally:
            self.root.deiconify()

    def show_success_dialog(self):
        """Show beautiful success dialog"""
        dialog = tk.Toplevel(self.root)
        dialog.title("✅ Success")
        dialog.geometry("500x400")
        dialog.configure(bg=self.colors['success'])
        dialog.transient(self.root)
        dialog.grab_set()

        # Success icon and message
        icon_label = tk.Label(dialog,
                             text="✅",
                             bg=self.colors['success'],
                             fg='white',
                             font=('SF Pro Display', 72))
        icon_label.pack(pady=(40, 20))

        title_label = tk.Label(dialog,
                              text="Provisioning Completed!",
                              bg=self.colors['success'],
                              fg='white',
                              font=('SF Pro Display', 20, 'bold'))
        title_label.pack()

        subtitle_label = tk.Label(dialog,
                                 text="Your server is ready to use",
                                 bg=self.colors['success'],
                                 fg='white',
                                 font=('SF Pro Text', 12))
        subtitle_label.pack(pady=(5, 30))

        # Info frame
        info_frame = tk.Frame(dialog, bg='#1e2329', padx=20, pady=15)
        info_frame.pack(fill=tk.X, padx=30, pady=(0, 20))

        tk.Label(info_frame,
                text="Next Steps:",
                bg='#1e2329',
                fg='#e8eaed',
                font=('SF Pro Text', 12, 'bold')).pack(anchor=tk.W, pady=(0, 10))

        tk.Label(info_frame,
                text=f"• SSH into your server: ssh {self.target_user.get()}@{self.target_ip.get()}",
                bg='#1e2329',
                fg='#a0a6b1',
                font=('SF Mono', 10)).pack(anchor=tk.W, pady=2)

        tk.Label(info_frame,
                text="• Check installed services and verify everything works",
                bg='#1e2329',
                fg='#a0a6b1',
                font=('SF Pro Text', 10)).pack(anchor=tk.W, pady=2)

        # Close button
        close_btn = tk.Button(dialog,
                             text="✓  Done",
                             command=dialog.destroy,
                             bg='#1e2329',
                             fg=self.colors['success'],
                             font=('SF Pro Text', 12, 'bold'),
                             padx=40,
                             pady=10,
                             relief='flat',
                             cursor='hand2')
        close_btn.pack(pady=(0, 30))

    def show_error_dialog(self, exit_code):
        """Show beautiful error dialog"""
        dialog = tk.Toplevel(self.root)
        dialog.title("❌ Error")
        dialog.geometry("550x450")
        dialog.configure(bg=self.colors['danger'])
        dialog.transient(self.root)
        dialog.grab_set()

        # Error icon and message
        icon_label = tk.Label(dialog,
                             text="❌",
                             bg=self.colors['danger'],
                             fg='white',
                             font=('SF Pro Display', 72))
        icon_label.pack(pady=(40, 20))

        title_label = tk.Label(dialog,
                              text="Provisioning Failed",
                              bg=self.colors['danger'],
                              fg='white',
                              font=('SF Pro Display', 20, 'bold'))
        title_label.pack()

        subtitle_label = tk.Label(dialog,
                                 text=f"Exit code: {exit_code}",
                                 bg=self.colors['danger'],
                                 fg='white',
                                 font=('SF Pro Text', 12))
        subtitle_label.pack(pady=(5, 30))

        # Troubleshooting frame
        info_frame = tk.Frame(dialog, bg='#1e2329', padx=20, pady=15)
        info_frame.pack(fill=tk.BOTH, expand=True, padx=30, pady=(0, 20))

        tk.Label(info_frame,
                text="Troubleshooting:",
                bg='#1e2329',
                fg='#e8eaed',
                font=('SF Pro Text', 12, 'bold')).pack(anchor=tk.W, pady=(0, 10))

        tk.Label(info_frame,
                text="• Check the terminal output above for error messages",
                bg='#1e2329',
                fg='#a0a6b1',
                font=('SF Pro Text', 10)).pack(anchor=tk.W, pady=2)

        tk.Label(info_frame,
                text=f"• Verify SSH connection: ssh {self.target_user.get()}@{self.target_ip.get()}",
                bg='#1e2329',
                fg='#a0a6b1',
                font=('SF Mono', 9)).pack(anchor=tk.W, pady=2)

        tk.Label(info_frame,
                text="• Check your server's connectivity and credentials",
                bg='#1e2329',
                fg='#a0a6b1',
                font=('SF Pro Text', 10)).pack(anchor=tk.W, pady=2)

        tk.Label(info_frame,
                text="• Try running with verbose output: ansible-playbook playbook.yml -vv",
                bg='#1e2329',
                fg='#a0a6b1',
                font=('SF Mono', 9)).pack(anchor=tk.W, pady=2)

        # Close button
        close_btn = tk.Button(dialog,
                             text="✕  Close",
                             command=dialog.destroy,
                             bg='#1e2329',
                             fg=self.colors['danger'],
                             font=('SF Pro Text', 12, 'bold'),
                             padx=40,
                             pady=10,
                             relief='flat',
                             cursor='hand2')
        close_btn.pack(pady=(0, 30))

    def show_exception_dialog(self, error_msg):
        """Show beautiful exception dialog"""
        dialog = tk.Toplevel(self.root)
        dialog.title("⚠️ Exception")
        dialog.geometry("550x400")
        dialog.configure(bg=self.colors['warning'])
        dialog.transient(self.root)
        dialog.grab_set()

        # Warning icon and message
        icon_label = tk.Label(dialog,
                             text="⚠️",
                             bg=self.colors['warning'],
                             fg='white',
                             font=('SF Pro Display', 72))
        icon_label.pack(pady=(40, 20))

        title_label = tk.Label(dialog,
                              text="Unexpected Error",
                              bg=self.colors['warning'],
                              fg='white',
                              font=('SF Pro Display', 20, 'bold'))
        title_label.pack()

        subtitle_label = tk.Label(dialog,
                                 text="Failed to run playbook",
                                 bg=self.colors['warning'],
                                 fg='white',
                                 font=('SF Pro Text', 12))
        subtitle_label.pack(pady=(5, 30))

        # Error details frame
        info_frame = tk.Frame(dialog, bg='#1e2329', padx=20, pady=15)
        info_frame.pack(fill=tk.BOTH, expand=True, padx=30, pady=(0, 20))

        tk.Label(info_frame,
                text="Error Details:",
                bg='#1e2329',
                fg='#e8eaed',
                font=('SF Pro Text', 12, 'bold')).pack(anchor=tk.W, pady=(0, 10))

        error_text = tk.Text(info_frame,
                            bg='#2a2e38',
                            fg=self.colors['danger'],
                            font=('SF Mono', 9),
                            height=6,
                            wrap=tk.WORD,
                            relief='flat',
                            padx=10,
                            pady=10)
        error_text.insert('1.0', error_msg)
        error_text.config(state='disabled')
        error_text.pack(fill=tk.BOTH, expand=True)

        # Close button
        close_btn = tk.Button(dialog,
                             text="✕  Close",
                             command=dialog.destroy,
                             bg='#1e2329',
                             fg=self.colors['warning'],
                             font=('SF Pro Text', 12, 'bold'),
                             padx=40,
                             pady=10,
                             relief='flat',
                             cursor='hand2')
        close_btn.pack(pady=(0, 30))

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
