//! VSCodeSSH lib module
//! This module provides library for various commands and
//! implementation for VSCodeSSH
//! 

#![warn(missing_docs)]
pub mod command;
pub mod vs_ssh;
pub mod common;
pub mod proto;

use command::{Command, CommandOutput};
use std::process;
pub use vs_ssh::VSCodeSSH;

use enum_dispatch::enum_dispatch;

/// Contains all the command impl
#[enum_dispatch(Command)]
pub enum CommandEnum {
    /// The command to invoke VSCode by using ssh to a machine. This is 
    /// essentially equivalent to `ssh user@remote.uri:/some/path/ code .`
    /// but is more portable, as long as invoker has vscode installed.
    VSCodeSSH(VSCodeSSH)
}
