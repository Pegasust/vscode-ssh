//! Schemas
//! This contains the schemas used throughout the application
//! necessary to pass through the clients and the servers

use std::{collections::HashMap, hash::Hash};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
enum MayRefer<T: Serialize, R: Hash+Serialize=String> {
    Underlying(T),
    Referred(R)
}

#[derive(Deserialize, Serialize, Debug)]
struct Workspace {
    name: String,
    repositories: Vec<MayRefer<Repository>>,
    host: MayRefer<Host>,
    user: Option<String>,
    env: HashMap<String, String>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Repository {
    name: String,
    path: String,
    host: Option<MayRefer<Host>>,
    env: HashMap<String, String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Host {
    // the given hostname; if None, then prompt user, depending on a Strategy
    name: String,
    addresses: Vec<String>,
    env: HashMap<String, String>,
    default_user: Option<String>
}

#[derive(Deserialize, Serialize, Debug)]
struct Hosts(Vec<Host>);

#[derive(Deserialize, Serialize, Debug)]
struct Repositories(Vec<Repository>);

#[derive(Deserialize, Serialize, Debug)]
struct Workspaces(Vec<Workspace>);

#[derive(Deserialize, Serialize, Debug)]
struct VSSHConfig {
    hosts: Hosts,
    repositories: Repositories,
    workspaces: Workspaces,
}
