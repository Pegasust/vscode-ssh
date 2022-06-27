# Design decisions

## Allow workspace to have different hosts?

## How does the configured user behave with existing SSH?

## How to support connection via SSH and existing VSCode Server?


## Hacks

- Env:
  - VSSH_CONFIG: Absolute paths for all configs

### Appendable configs

```yml
# vssh-config.yml
hosts:
- name: school_remote  # primary key
  addresses: [127.0.0.1, 192.168.0.3, some.public.address.com]
  env: {X11_DISPLAY: 192.168.0.4:0, SOME_OTHER_ENV: true}
  default_user: user01

repositories:
- name: vscode-ssh      # primary key
  path: /home/user/workspaces/vscode-ssh
  host: school_remote   # foreign: hosts
  env:
    REPOS_ENV: hello world
  debug:
    expected_remote: 
    - vscode-remote://ssh-remote+user01@127.0.0.1/home/user/workspaces/vscode-ssh
    - vscode-remote://ssh-remote+user01@192.168.0.3/home/user/workspaces/vscode-ssh
    - vscode-remote://ssh-remote+user01@some.public.address.com/home/user/workspaces/vscode-ssh

- name: vscode-ssh-dev
  path: /home/dev/git_servers/vscode-ssh
  host: 127.0.0.1
  debug:
    expected_remote:
    - vscode-remote://ssh-remote@127.0.0.1/home/dev/git_servers/vscode-ssh

workspaces: # currently, workspace should point to the same host
- name: vscode-stable   # primary key
  repositories:
  - name: vscode-ssh
  - path: /home/user/workspaces/vscode-ssh-plugins # implicit declaration; anonymous repos
  user: workspace_user # supply if want to override TODO: Implicit derivation?
  host: workspace-host # supply if want to override TODO: Implicit derivation?
  env: {WORKSPACE_ENV: my workspace env}
- name: vscode-dev
  repositories:
  - name: vscode-ssh-dev
  # implicit declaration
  - name: vscode-ssh-dev-infrastructure
    path: /home/dev/git_servers/infrastructures/vscode-ssh
```
