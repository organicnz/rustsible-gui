<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  interface ProvisioningConfig {
    ip_address: string;
    ssh_user: string;
    ssh_password: string;
    hostname: string;
    fail2ban: boolean;
    docker: boolean;
    swap: boolean;
    lemp: boolean;
    devtools: boolean;
    wordpress: boolean;
    certbot: boolean;
    system_hardening: boolean;
    apparmor: boolean;
    rootkit_detection: boolean;
    file_integrity: boolean;
    audit_logging: boolean;
    log_monitoring: boolean;
    advanced_protection: boolean;
    cron_jobs: boolean;
    periodic_reboot: boolean;
    reboot_hour: string;
  }

  let config: ProvisioningConfig = $state({
    ip_address: "",
    ssh_user: "root",
    ssh_password: "",
    hostname: "",
    fail2ban: true,
    docker: true,
    swap: true,
    lemp: false,
    devtools: true,
    wordpress: false,
    certbot: false,
    system_hardening: false,
    apparmor: false,
    rootkit_detection: false,
    file_integrity: false,
    audit_logging: false,
    log_monitoring: false,
    advanced_protection: false,
    cron_jobs: true,
    periodic_reboot: false,
    reboot_hour: "3",
  });

  let provisioning = $state(false);
  let result = $state("");
  let error = $state("");

  const rebootOptions = [
    { value: "1", label: "Daily at 1:00 AM" },
    { value: "2", label: "Daily at 2:00 AM" },
    { value: "3", label: "Daily at 3:00 AM" },
    { value: "4", label: "Daily at 4:00 AM" },
    { value: "5", label: "Daily at 5:00 AM" },
    { value: "*/6", label: "Every 6 hours" },
    { value: "*/12", label: "Every 12 hours" },
    { value: "*/24", label: "Every 24 hours" },
  ];

  onMount(async () => {
    try {
      const cached = await invoke<ProvisioningConfig>("load_cache");
      config = cached;
    } catch (e) {
      console.error("Failed to load cache:", e);
    }
  });

  async function saveConfiguration() {
    try {
      await invoke("save_cache", { config });
    } catch (e) {
      console.error("Failed to save cache:", e);
    }
  }

  async function launchProvisioning(event: Event) {
    event.preventDefault();
    provisioning = true;
    result = "";
    error = "";

    try {
      const output = await invoke<string>("launch_provisioning", { config });
      result = output;
    } catch (e) {
      error = String(e);
    } finally {
      provisioning = false;
    }
  }

  $effect(() => {
    // Auto-save configuration whenever it changes
    saveConfiguration();
  });
</script>

<main class="container">
  <header>
    <h1>🖥️ Ubuntu Server Provisioning</h1>
    <p class="subtitle">Configure and deploy your server with one click</p>
    <div class="divider"></div>
  </header>

  <form onsubmit={launchProvisioning}>
    <!-- Connection Information Card -->
    <section class="card">
      <h2>📡 Connection Information</h2>

      <div class="form-group">
        <label for="ip">IP Address</label>
        <input
          id="ip"
          type="text"
          bind:value={config.ip_address}
          placeholder="192.168.1.100"
          required
        />
      </div>

      <div class="form-group">
        <label for="user">SSH User</label>
        <input
          id="user"
          type="text"
          bind:value={config.ssh_user}
          placeholder="root"
          required
        />
      </div>

      <div class="form-group">
        <label for="password">SSH Password</label>
        <input
          id="password"
          type="password"
          bind:value={config.ssh_password}
          placeholder="Enter SSH password"
        />
      </div>

      <div class="form-group">
        <label for="hostname">Hostname (optional)</label>
        <input
          id="hostname"
          type="text"
          bind:value={config.hostname}
          placeholder="my-server"
        />
      </div>
    </section>

    <!-- Core Features Card -->
    <section class="card">
      <h2>⚡ Core Features</h2>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.fail2ban} />
        <span>🛡️ Fail2ban Intrusion Prevention</span>
      </label>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.docker} />
        <span>🐳 Docker & Docker Compose</span>
      </label>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.swap} />
        <span>💾 Swap Memory (auto-sized)</span>
      </label>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.lemp} />
        <span>🌐 LEMP Stack (Nginx, MySQL, PHP)</span>
      </label>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.devtools} />
        <span>⚙️ Development Tools (Neovim, Node.js, Claude Code)</span>
      </label>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.wordpress} />
        <span>📝 WordPress CMS</span>
      </label>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.certbot} />
        <span>🔒 Certbot SSL/TLS Certificates</span>
      </label>
    </section>

    <!-- Security Clusters Card -->
    <section class="card">
      <h2>🔒 Security Clusters</h2>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.system_hardening} />
        <span>🔐 System Hardening</span>
      </label>
      <p class="checkbox-desc">• Kernel hardening, secure shared memory</p>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.apparmor} />
        <span>🛡️ AppArmor Enforcement</span>
      </label>
      <p class="checkbox-desc">• Mandatory access control</p>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.rootkit_detection} />
        <span>🔍 Rootkit Detection (rkhunter)</span>
      </label>
      <p class="checkbox-desc">• Daily scans for malware</p>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.file_integrity} />
        <span>📁 File Integrity Monitoring (AIDE)</span>
      </label>
      <p class="checkbox-desc">• Tracks unauthorized file changes</p>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.audit_logging} />
        <span>📊 Audit Logging (auditd)</span>
      </label>
      <p class="checkbox-desc">• System call auditing</p>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.log_monitoring} />
        <span>📋 Log Monitoring (Logwatch)</span>
      </label>
      <p class="checkbox-desc">• Daily log analysis reports</p>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.advanced_protection} />
        <span>🚀 Advanced Protection</span>
      </label>
      <p class="checkbox-desc">• 2FA, Backups, USB restrictions</p>
    </section>

    <!-- Maintenance Settings Card -->
    <section class="card">
      <h2>🔧 Maintenance Settings</h2>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.cron_jobs} />
        <span>⏰ Automated Updates & Cron Jobs</span>
      </label>

      <label class="checkbox">
        <input type="checkbox" bind:checked={config.periodic_reboot} />
        <span>🔄 Periodic System Reboot</span>
      </label>

      {#if config.periodic_reboot}
        <div class="form-group" style="margin-top: 1rem;">
          <label for="reboot_hour">Reboot Schedule</label>
          <select id="reboot_hour" bind:value={config.reboot_hour}>
            {#each rebootOptions as option}
              <option value={option.value}>{option.label}</option>
            {/each}
          </select>
          <p class="checkbox-desc">
            {rebootOptions.find((o) => o.value === config.reboot_hour)?.label || ""}
          </p>
        </div>
      {/if}
    </section>

    <!-- Action Buttons -->
    <div class="button-group">
      <button
        type="submit"
        class="primary"
        disabled={provisioning}
      >
        {provisioning ? "🔄 Provisioning..." : "🚀 Launch Provisioning"}
      </button>
    </div>

    <!-- Result Display -->
    {#if result}
      <div class="result success">
        <h3>✅ Success!</h3>
        <pre>{result}</pre>
      </div>
    {/if}

    {#if error}
      <div class="result error">
        <h3>❌ Error</h3>
        <pre>{error}</pre>
      </div>
    {/if}
  </form>

  <footer>
    <p>💾 Settings are automatically saved and restored</p>
  </footer>
</main>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
    background: #1a1d23;
    color: #e8eaed;
    overflow-y: scroll;
  }

  .container {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem 1.5rem;
  }

  header {
    text-align: center;
    margin-bottom: 2rem;
  }

  h1 {
    font-size: 2rem;
    font-weight: 700;
    margin: 0 0 0.5rem 0;
    color: #e8eaed;
  }

  .subtitle {
    font-size: 0.9rem;
    color: #9aa0a6;
    margin: 0;
  }

  .divider {
    height: 1px;
    background: #3a3f4b;
    margin: 1.5rem 0;
  }

  .card {
    background: #252930;
    border: 1px solid #3a3f4b;
    border-radius: 12px;
    padding: 1.5rem;
    margin-bottom: 1.5rem;
    transition: border-color 0.2s;
  }

  .card:hover {
    border-color: #5dade2;
  }

  .card h2 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0 0 1.5rem 0;
    color: #e8eaed;
  }

  .form-group {
    margin-bottom: 1.25rem;
  }

  .form-group:last-child {
    margin-bottom: 0;
  }

  label {
    display: block;
    font-size: 0.9rem;
    font-weight: 500;
    margin-bottom: 0.5rem;
    color: #9aa0a6;
  }

  input[type="text"],
  input[type="password"],
  select {
    width: 100%;
    padding: 0.75rem;
    background: #2a2e38;
    border: 1px solid #3a3f4b;
    border-radius: 8px;
    color: #e8eaed;
    font-size: 0.95rem;
    transition: border-color 0.2s, background 0.2s;
  }

  input[type="text"]:focus,
  input[type="password"]:focus,
  select:focus {
    outline: none;
    border-color: #5dade2;
    background: #2d323a;
  }

  input::placeholder {
    color: #5f6368;
  }

  .checkbox {
    display: flex;
    align-items: center;
    padding: 0.75rem 0;
    cursor: pointer;
    user-select: none;
  }

  .checkbox input[type="checkbox"] {
    width: 1.25rem;
    height: 1.25rem;
    margin-right: 0.75rem;
    cursor: pointer;
    flex-shrink: 0;
  }

  .checkbox span {
    font-size: 0.95rem;
    color: #e8eaed;
  }

  .checkbox-desc {
    margin: 0 0 0.5rem 2rem;
    font-size: 0.85rem;
    color: #9aa0a6;
  }

  .button-group {
    display: flex;
    gap: 1rem;
    margin-top: 2rem;
  }

  button {
    padding: 1rem 2rem;
    font-size: 1rem;
    font-weight: 600;
    border: none;
    border-radius: 8px;
    cursor: pointer;
    transition: all 0.2s;
  }

  button.primary {
    background: linear-gradient(135deg, #2ecc71 0%, #27ae60 100%);
    color: white;
    flex: 1;
  }

  button.primary:hover:not(:disabled) {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(46, 204, 113, 0.3);
  }

  button:disabled {
    opacity: 0.6;
    cursor: not-allowed;
  }

  .result {
    margin-top: 2rem;
    padding: 1.5rem;
    border-radius: 12px;
    border: 1px solid;
  }

  .result.success {
    background: rgba(46, 204, 113, 0.1);
    border-color: #2ecc71;
  }

  .result.error {
    background: rgba(231, 76, 60, 0.1);
    border-color: #e74c3c;
  }

  .result h3 {
    margin: 0 0 1rem 0;
    font-size: 1.1rem;
  }

  .result pre {
    background: #1a1d23;
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    margin: 0;
    font-size: 0.85rem;
    line-height: 1.5;
  }

  footer {
    text-align: center;
    margin-top: 3rem;
    padding-top: 2rem;
    border-top: 1px solid #3a3f4b;
  }

  footer p {
    font-size: 0.85rem;
    color: #5f6368;
    margin: 0;
  }

  /* Scrollbar styling */
  :global(::-webkit-scrollbar) {
    width: 10px;
  }

  :global(::-webkit-scrollbar-track) {
    background: #1a1d23;
  }

  :global(::-webkit-scrollbar-thumb) {
    background: #3a3f4b;
    border-radius: 5px;
  }

  :global(::-webkit-scrollbar-thumb:hover) {
    background: #5dade2;
  }
</style>
