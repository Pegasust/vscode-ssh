# VSCode-ssh

A project that aims to make launching vscode on a remote machine much easier.

## What does this solve?

### Current inconvenience

Sometimes, typing `code .` in an ssh-ed remote doesn't invoke vscode with
ssh into the machine:

```console
(base) local_user@local_computer:local_path$ ssh remote:path

...
(base) user@remote:path$ code . 
-bash: code: command not found
```

VSCode provides an [undocumented way](https://github.com/microsoft/vscode-remote-release/issues/190) to invoke VSCode with ssh-ed uri:

```console
(base) local_user@local_computer:local_path$ code --folder-uri vscode-remote://ssh-remote+user@remote/path/
```

There is also an alternative but not used in this project:

```console
$ code --remote=ssh-remote+user@remote path/
```

### Declarative workspace

We also seek to provide a way to create a declarative workspace by specifying
the host and their paths instead of doing it imperatively. 

#### Declarative settings

```console
$ ls ~/.vscode-ssh-configs
host_aliases.yml  repositories.yml

$ vscode-ssh show-hosts
hosts:
- remote_ip:
  - 192.168.0.x
  - some.public.address.com
  - 127.0.0.1
  alias:
    school_remote
  env:
  - X11_DISPLAY=...
    ...
- alias:
    homelab
  remote-ip:
  - x.x.x.x
  default_user: user01

$ vscode-ssh show-repos
repositories:
- name: vscode-ssh
  path: /home/user/workspaces/vscode-ssh
- name: vscode-ssh-dev
  path: /home/dev/git_servers/vscode-ssh
  host: school_remote
```

#### Declarative workspace settings

vscode-ssh would also support configuring your own workspace using a
supported format so that you could launch it later on

```yaml
# vscode-ssh-workspace.yml
workspace:
- repository: vscode-ssh
- repository: vscode-ssh-dev
  user: just_a_user
  env:
  - RUST_BACKTRACE=1
- host: x.x.x.x
  path: /my_path/to/this_project
  user: i_am_admin
  env:
  - ENV_0
  - ENV_1=whatever
```

#### Simple repository/workspace management

```console
$ vscode-ssh workspace-history
/path/to/vscode-ssh-workspace.yml
/path/to/another/school-project.yml

$ vscode-ssh launch /path/to/vscode-ssh-workspace.yml
```