
use vscode_ssh::{CommandEnum::{self}, VSCodeSSH, command::Command};

fn ssh_local() -> VSCodeSSH {
    VSCodeSSH::from_ip("localhost")
}

fn ssh_felius() -> VSCodeSSH {
    VSCodeSSH::from_auth(
        "felius.ddns.net", 
        "ubuntu_admin", 
        "/home/ubuntu_admin/git_server/vscode-ssh"
    )
}

#[test]
fn vs_ssh_cannot_undo() {
    let ssh_subcmd: CommandEnum = ssh_felius().into();
    assert!(ssh_subcmd.undo_proc().is_none());
}

#[test]
fn vs_ssh_perform_end_well() {
    assert!(!ssh_felius().perform().stabilize().is_limbo())
}

#[test]
fn ssh_local_end_well() {
    assert!(!ssh_local().perform().stabilize().is_limbo())
}