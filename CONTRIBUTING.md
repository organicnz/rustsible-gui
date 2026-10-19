# Contributing to Rustsible

Thanks for your interest in contributing! This project combines a Rust desktop GUI (`gui-egui/`) with an Ansible provisioning engine (`rustsible.yml` + `roles/`). Here's how to get involved.

## What to work on

Check the [Issues](https://github.com/organicnz/rustsible-gui/issues) page for open tasks. We tag issues to help you find something that fits your skills.

- **good first issue** — small fixes, docs, UI polish
- **bug** — something is broken
- **enhancement** — new features or improvements
- **gui** — Rust/egui frontend work
- **ansible** — playbook, roles, or variable changes

## Getting started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- [Ansible](https://docs.ansible.com/ansible/latest/installation_guide/index.html) 2.9+
- [Ansible Galaxy](https://docs.ansible.com/ansible/latest/galaxy/user_guide.html) for role dependencies

### Build & run the GUI

```bash
# Clone the repo
git clone https://github.com/organicnz/rustsible-gui.git
cd rustsible-gui

# Build the release binary
make build

# Or run in dev mode
make gui
```

### Run the Ansible engine directly

```bash
# Install required roles
ansible-galaxy install oefenweb.fail2ban

# Configure your server
vim inventory.ini
vim vars/default.yml

# Dry-run first
ansible-playbook rustsible.yml --check

# Run provisioning
ansible-playbook rustsible.yml
```

## Development workflow

1. **Fork** the repository
2. **Create a branch** (`git checkout -b feat/my-change` or `fix/my-bug`)
3. **Make your changes** — keep them scoped and focused
4. **Test** — build the GUI (`cargo build --release`) and/or dry-run the playbook
5. **Commit** with clear, descriptive messages
6. **Open a Pull Request** against `main`

## Code style

### Rust (gui-egui/)

- Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix warnings
- Module structure: `main.rs` (entry), `app.rs` (state), `ui.rs` (rendering), `config.rs` (serde), `provisioning.rs` (subprocess), `style.rs` (theme)

### Ansible (roles/, rustsible.yml)

- Use `snake_case` for all variable and role names
- Prefix interactive input variables with `prompt_`
- Prefix feature flags with `enable_*` or `install_*`
- Always use `become: yes` for privileged tasks
- Add tags to every task for selective execution

## Adding a new feature to the GUI

1. Add the config field to `ProvisioningConfig` in `config.rs` (use `#[serde(default)]` for backward compatibility)
2. Add the corresponding Ansible variable mapping in `provisioning.rs`
3. Add the UI control in `ui.rs` under the appropriate `NavSection`
4. Wire the variable into the `ansible-playbook -e` flags

## Reporting bugs

Open an issue with:

- **What you expected** to happen
- **What actually** happened
- **Steps to reproduce**
- **OS and version** (e.g. macOS 14, Ubuntu 22.04)
- **Rust version** (`rustc --version`) for GUI issues
- **Ansible version** (`ansible --version`) for playbook issues

## Questions?

Feel free to ask in the [Discussions](https://github.com/organicnz/rustsible-gui/discussions) page or reach out via issues.

## License

By contributing, you agree that your contributions will be licensed under the MIT License.
