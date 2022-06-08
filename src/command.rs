
use std::{process::{self}};

use enum_dispatch::enum_dispatch;

#[derive(Debug)]
pub enum CommandOutput {
    // Command that produces nothing
    Void(Result<(), String>),
    // Command that produces a string
    String(Result<String, String>),
    // Command that produces seemingly stdout and stderr
    ProcessOutput(Result<std::process::Output, std::io::Error>),
    // The state where the command output is unable to stablize into String
    Limbo
}
impl CommandOutput {
    /// Attempts to stablize the output into Self::String.
    /// If unable to stablize, returns Self::Limbo
    pub fn stabilize(self) -> CommandOutput {
        match self {
            a @ CommandOutput::String(_) => a,
            CommandOutput::Void(a) => 
                CommandOutput::String(a.map(|_| "".to_string())),
            CommandOutput::ProcessOutput(proc_res) => CommandOutput::String(
                proc_res.map_err(|err| err.to_string())
                        .map(|output| 
                            format!("{:?}", output)
                        )
            ),
            _ => CommandOutput::Limbo
        }
    }
    pub fn is_limbo(&self) -> bool {
        match self {
            Self::Limbo => true,
            _ => false,
        }
    }
}

impl Into<Result<String, String>> for CommandOutput {
    fn into(self) -> Result<String, String> {
        let stablized = self.stabilize();
        match stablized {
            CommandOutput::String(a) => a,
            bad @ _ => Err(format!("Unable to transform: {}", match bad {
                CommandOutput::Limbo => "Is Limbo",
                _ => "Unknown",
            }))
        }
    }
}
#[enum_dispatch]
pub trait Command {
    /// Creates the process to carry out this particular command
    fn apply_proc(&self)->process::Command;
    /// Creates the process to undo the command
    /// if this command couldn't be undone, then returns a None
    fn undo_proc(&self)->Option<process::Command>;

    fn perform(&self)->CommandOutput {
        CommandOutput::ProcessOutput(
            self.apply_proc().spawn()
                .and_then(|child| child.wait_with_output())
        )
    }
    fn drawback(&self)->Option<CommandOutput> {
        self.undo_proc()
            .map(|mut cmd| CommandOutput::ProcessOutput(
                cmd.spawn().and_then(|child| child.wait_with_output())
            ))
    }
}