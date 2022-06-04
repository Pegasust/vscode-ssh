
use std::{format, process};

use enum_dispatch::enum_dispatch;

pub struct VSCodeSSH {
    host_ip: String,
    host_user: Option<String>,
    host_abspath: Option<String>
}

impl VSCodeSSH {
    // Need to acquire link: 
    // vscode-remote://ssh-remote+felius.ddns.net/home/ubuntu_admin/git_server/vscode-ssh
    fn get_uri(&self)->String {
        format!("vscode-remote://ssh-remote+{}{}{}",
            match &self.host_user {
                Some(user) => user.clone() + "@",
                None => "".to_string()
            },
            &self.host_ip,
            self.host_abspath.clone().unwrap_or_else(|| "".to_string())
        )
    }
    pub fn from_ip(host_ip: &'_ str)-> Self {
        VSCodeSSH { host_ip: host_ip.to_string(), host_user: None, host_abspath: None }
    }
    pub fn from_abs(host_ip: &'_ str, host_abspath: &'_ str) -> Self {
        VSCodeSSH { host_ip: host_ip.to_string(), host_user: None, host_abspath: Some(host_abspath.to_string()) }
    }
    pub fn from_user(host_ip: &'_ str, host_user: &'_ str) -> Self {
        VSCodeSSH { host_ip: host_ip.to_string(), host_user: Some(host_user.to_string()), host_abspath: None }
    }
    pub fn from_all(host_ip: &'_ str, 
        host_user: &'_ str, 
        host_abspath: &'_ str)
        -> Self 
    {
        VSCodeSSH { 
            host_ip: host_ip.to_string(), 
            host_user: Some(host_user.to_string()), 
            host_abspath: Some(host_abspath.to_string())
        }
    }
}
#[enum_dispatch(Command)]
enum CommandEnum {
    VSCodeSSH
}

pub enum CommandOutput {
    Void(Result<(), String>),
    String(Result<String, String>),
    ProcessOutput(Result<std::process::Output, std::io::Error>)
}
impl CommandOutput {
    pub fn stabilize(self) -> CommandOutput {
        match self {
            a @ CommandOutput::String(_) => a,
            CommandOutput::Void(a) => 
                CommandOutput::String(a.and_then(|_| Ok("".to_string()))),
            CommandOutput::ProcessOutput(_) => todo!(),
        }
    }
}
#[enum_dispatch]
pub trait Command {
    fn apply_proc(&self)->process::Command;
    fn undo_proc(&self)->Option<process::Command>;

    fn perform(&self)->CommandOutput {
        CommandOutput::ProcessOutput(
            self.apply_proc().spawn()
            .and_then(|child| child.wait_with_output())
        )
    }
    fn drawback(&self)->Option<CommandOutput> {
        self.undo_proc().map(|mut cmd| CommandOutput::ProcessOutput(
            cmd.spawn().and_then(|child| child.wait_with_output())
        ))
    }
}

impl Command for VSCodeSSH {
    fn apply_proc(&self) -> process::Command {
        todo!()
    }

    fn undo_proc(&self) -> Option<process::Command> {
        Option::None
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn uri_test_c_nil_nil() {
        assert_eq!(VSCodeSSH::from_ip("felius.ddns.net").get_uri(), 
            "vscode-remote://ssh-remote+felius.ddns.net".to_string()
        );
    }
    #[test]
    fn uri_test_c_c() {
        assert_eq!(VSCodeSSH::from_user("felius.ddns.net", "ubuntu_admin").get_uri(), 
            "vscode-remote://ssh-remote+ubuntu_admin@felius.ddns.net".to_string());
    }
    #[test]
    fn uri_test_c_nil_c() {
        assert_eq!(VSCodeSSH::from_abs("felius.ddns.net", "/home/ubuntu_admin/git_server/vscode-ssh").get_uri(),
            "vscode-remote://ssh-remote+felius.ddns.net/home/ubuntu_admin/git_server/vscode-ssh".to_string()
        );
    }
    #[test]
    fn uri_test_c_c_c() {
        assert_eq!(VSCodeSSH::from_all("felius.ddns.net", "ubuntu_admin", "/home/ubuntu_admin/git_server").get_uri(),
        "vscode-remote://ssh-remote+ubuntu_admin@felius.ddns.net/home/ubuntu_admin/git_server".to_string());
    }
}