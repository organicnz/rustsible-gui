# Updated Initial Ansible Server Setup on Ubuntu 23.04

This updated Ansible playbook will execute a initial server setup for Ubuntu 23.04 systems, as explained in the guide on
[Initial Ansible Server Setup Guide for Ubuntu 18.04](https://www.digitalocean.com/community/tutorials/how-to-use-ansible-to-automate-initial-server-setup-on-ubuntu-18-04).
A number of containers will be created with the options specified in the `vars/default.yml` variable file.

## Settings

- `added_user`: the name of the remote sudo user to create.
- `copy_local_key`: path to a local SSH public key that will be copied as authorized key for the new user. By default, it copies the key from the current system user running Ansible.
- `sys_packages`: array with list of packages that should be installed.


## Running this Playbook

Quick Steps:

### 1. Obtain the playbook
```shell
git clone https://gitlab.com/organicnz/ansible-ubuntu-2304.git
cd ansible-playbooks/setup_ubuntu2304
```

### 2. Customize Options

```shell
nano vars/default.yml
```

```yml
#vars/default.yml
---
added_user: organic
copy_local_key: "{{ lookup('file', lookup('env','HOME') + '/.ssh/id_rsa.pub') }}"
sys_packages: [ 'curl', 'vim', 'git', 'ufw']
```

### 3. Run the Playbook

```bash
ansible-playbook -l [target] -i [inventory file] -u [remote user] playbook.yml
```

In our case we can use this command
```bash
ansible-playbook playbook.yml -l webservers -i inventory.ini -u root -k

ansible-playbook playbook.yml -l webservers -i inventory.ini -u root --ask-become-pass
```

If root is already disabled and user is created
```bash
ansible-playbook playbook.yml -l webservers -i inventory.ini -u organic --ask-become-pass
```

### Troubleshooting

Failed to connect to the host via ssh: root Permission denied (publickey)., unreachable: true}
```bash
eval 'ssh-agent' && ssh-add ~/.ssh/id_rsa.pem 
```

In my case for Azure
```bash
eval 'ssh-agent' && ssh-add ~/.ssh/azure_id_rsa.pem 
```


For more information on how to run this Ansible setup, please check this DigitalOcean guide: [How to Use Ansible to Automate Initial Server Setup on Ubuntu 20.04](https://www.digitalocean.com/community/tutorials/how-to-use-ansible-to-automate-initial-server-setup-on-ubuntu-20-04).