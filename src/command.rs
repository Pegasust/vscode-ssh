//! Command module
//! Adds some interfaces regarding defining a command
//! 
//! Provides:
//!  - [CommandOutput]: The output of a command execution
//!  - [Command]: Trait of an executable command
use std::{process::{self}, borrow::Borrow};

use enum_dispatch::enum_dispatch;

/// Represents all of the output types possible for a command execution
/// 
#[derive(Debug)]
pub enum CommandOutput {
    /// Command that produces nothing
    Void(Result<(), String>),
    /// Command that produces a string
    String(Result<String, String>),
    /// Command that produces seemingly stdout and stderr
    ProcessOutput(Result<std::process::Output, std::io::Error>),
    /// The state where the command output is unable to stablize into String
    Limbo
}
impl CommandOutput {
    /// Attempts to stablize the output into [CommandOutput::String].
    /// If unable to stablize, returns [CommandOutput::Limbo]
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
    /// Quick check if the command output is [`CommandOutput::Limbo`]
    pub fn is_limbo(&self) -> bool {
        matches!(self, Self::Limbo)
    }
}

impl From<CommandOutput> for Result<String, String> {
    /// Attempts to a CommandOutput to a String output
    fn from(val: CommandOutput) -> Self {
        let stablized = val.stabilize();
        match stablized {
            CommandOutput::String(a) => a,
            bad => 
                Err(format!("Unable to transform: {}", 
                    match bad {
                        CommandOutput::Limbo => "Is Limbo",
                        _ => "Unknown",
            }))
        }
    }
}
/// Interface for an executable command
#[enum_dispatch]
pub trait Command {
    /// Creates the process to carry out this particular command
    fn apply_proc(&self)->process::Command;
    /// Creates the process to undo the command
    /// if this command couldn't be undone, then returns a None
    fn undo_proc(&self)->Option<process::Command>;
    /// Runs this command in check_mode: does not change
    /// the system in anyway, only for testing the output.
    /// This mode may touch a cache in some way, but should
    /// not be destructive.
    fn check_proc(&self)->Option<process::Command> {
        None
    }

    /// Performs the command and returns a [CommandOutput]
    /// The default implementation creates process using
    /// [Command::apply_proc()] then executes it with output()
    fn perform(&self)->CommandOutput {
        CommandOutput::ProcessOutput(
            self.apply_proc().output()
        )
    }
    /// Optionally performs the undo of the executed command.
    /// The default implementation creates & queries if undo is possible
    /// by testing `self.undo_proc().is_some()` and executes the
    /// created process with output()
    fn drawback(&self)->Option<CommandOutput> {
        self.undo_proc()
            .map(|mut cmd| CommandOutput::ProcessOutput(
                cmd.output()
            ))
    }
    /// Optionally performs this command in check mode.
    /// The default implementation will "skip" if check mode not
    /// supported (check_proc returns Option::None)
    fn check(&self)->Result<CommandOutput, String> {
        self.check_proc().map(|mut cmd| CommandOutput::ProcessOutput(
            cmd.output()
        )).ok_or_else(|| "Command does not support check mode".to_owned())
    }
}
