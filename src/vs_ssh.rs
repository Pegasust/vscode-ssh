

use std::process::{self, Stdio};
use crate::command::Command;

use super::anystr::*;

pub struct VSCodeSSH {
    host_ip: String,
    host_user: Option<String>,
    host_abspath: Option<String>,
    verbose: bool
}

impl VSCodeSSH {
    /// Returns the qualified uri to ssh into
    /// vscode-remote://ssh-remote+felius.ddns.net/home/ubuntu_admin/git_server/vscode-ssh
    pub fn get_uri(&self)->String {
        format!("vscode-remote://ssh-remote+{}{}{}",
            match &self.host_user.clone().and_then(|s| if s.is_empty() {None} else {Some(s)}) {
                Some(user) => user.clone() + "@",
                None => "".to_string()
            },
            &self.host_ip,
            self.host_abspath.clone().unwrap_or_else(|| "".to_string())
        )
    }

    pub fn new<Str0: AsRef<str>, Str1: AsRef<str>, Str2: AsRef<str>>(
        host_ip: Str0,
        host_user: Option<Str1>,
        host_abspath: Option<Str2>,
        verbose: Option<bool>
        ) -> Self 
    {
        VSCodeSSH { 
            host_ip: host_ip.as_ref().to_string(), 
            host_user: host_user.map(any_str_to_string), 
            host_abspath: host_abspath.map(any_str_to_string), 
            verbose: verbose.unwrap_or(false)
        }
    }
    pub fn reach_test_cmd() -> String {
        "echo \"Hello world\"".to_string()
    }
    pub fn from_ip<_AnyStr: AnyStr>(host_ip: _AnyStr)-> Self {
        Self::new(host_ip, anystr_none(), anystr_none(), None)
    }
    pub fn from_abs<S0:AnyStr, S1:AnyStr>(host_ip: S0, host_abspath: S1) -> Self {
        Self::new(host_ip, anystr_none(), anystr_some(host_abspath), None)
    }
    pub fn from_user<S0:AnyStr, S1:AnyStr>(host_ip: S0, host_user: S1) -> Self {
        Self::new(host_ip, anystr_some(host_user), anystr_none(), None)
    }
    pub fn from_auth<S0:AnyStr, S1:AnyStr, S2:AnyStr>(
        host_ip: S0, 
        host_user: S1, 
        host_abspath: S2)
        -> Self 
    {
        Self::new(host_ip, anystr_some(host_user), anystr_some(host_abspath), None)
    }
    pub fn make_verbose(mut self) -> Self {
        self.verbose = true;
        self
    }
}

impl Command for VSCodeSSH {
    fn apply_proc(&self) -> process::Command {
        let mut retval = process::Command::new("code");
        if self.verbose {
            retval.arg("--verbose");
        }
        retval
            .arg("--folder-uri")
            .arg(self.get_uri())
            // .stdout(Stdio::piped())
            // .stderr(Stdio::piped())
            // .stdin(Stdio::piped())
        ;

        retval
    }

    fn undo_proc(&self) -> Option<process::Command> {
        Option::None // operation not supported
    }
}


#[cfg(test)]
mod test {
    use std::ffi::OsStr;

    use crate::command::CommandOutput;

    use super::*;


    fn ssh_local() -> VSCodeSSH {
        VSCodeSSH::from_ip("localhost")
    }

    #[test]
    fn has_code_cmd() {
        const COMMAND_NOT_FOUND: i32 = 127;
        let output = ssh_local().perform();
        assert!(matches!(&output, CommandOutput::ProcessOutput(_)));
        match output {
            CommandOutput::ProcessOutput(a) => 
                assert!(a.unwrap().status.code().unwrap() != COMMAND_NOT_FOUND),
            _ => panic!()
        };
    }

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
        assert_eq!(VSCodeSSH::from_auth("felius.ddns.net", "ubuntu_admin", "/home/ubuntu_admin/git_server").get_uri(),
        "vscode-remote://ssh-remote+ubuntu_admin@felius.ddns.net/home/ubuntu_admin/git_server".to_string());
    }

    #[test]
    fn uri_test_empty_user() {
        // significance: notice how user's name is empty and there is no '@'
        assert_eq!(VSCodeSSH::from_user("felius.ddns.net", "").get_uri(),
            "vscode-remote://ssh-remote+felius.ddns.net".to_string())
    }

    #[test]
    fn vs_ssh_unsupported_undo() {
        assert!(VSCodeSSH::from_auth(
            "felius.ddns.net",
            "ubuntu_admin",
            "/home/ubuntu_admin/git_server/vscode-ssh"
        ).undo_proc().is_none());
    }
    

    #[test]
    fn ssh_creation_local() {
        let cmd = ssh_local().apply_proc();
        let matches: Vec<&str> = cmd.get_program().to_str().unwrap().matches("code").collect();
        assert_eq!(matches, ["code"]);
        let args: Vec<&OsStr> = cmd.get_args().collect();
        assert_eq!(args, &["--folder-uri", "vscode-remote://ssh-remote+localhost"]);
    }

    #[test]
    fn vs_ssh_local_perf() {
        let output = ssh_local().perform();
        if let CommandOutput::ProcessOutput(res) = output {
            assert!(res.unwrap().status.success())
        } else {
            panic!()
        };
    }
    #[test]
    fn vs_ssh_local_perf_verbose_output() {
        let cmd = ssh_local().make_verbose();
        let proc = cmd.apply_proc();
        println!("prog: {:?}; args {:?}", proc.get_program(), proc.get_args());
        println!("{}", 
            Into::<Result<String,String>>::into(cmd.perform())
            .unwrap_or_else(|err| err));
    }
    #[test]
    fn vs_ssh_local_output() {
        let cmd = ssh_local();
        let proc = cmd.apply_proc();
        println!("prog: {:?}; args {:?}", proc.get_program(), proc.get_args());
        println!("{}", 
            Into::<Result<String,String>>::into(cmd.perform())
            .unwrap_or_else(|err| err));

    }
}