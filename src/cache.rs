use std::{collections::HashMap, fs, io};

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct RepoConfig {
    pub(crate) trunk: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct Storage {
    pub(crate) branches: HashMap<String, BranchStack>,
}

#[derive(Deserialize, Serialize, Debug)]
pub(crate) struct BranchStack {
    pub(crate) branch_name: String,
    pub(crate) parent_branch_name: Option<String>,
    pub(crate) children: Vec<String>,
}

pub(crate) fn write_config_file(config: RepoConfig) -> io::Result<()> {
    fs::write(
        ".git/.luxe_repo_config.json",
        serde_json::to_string_pretty(&config).unwrap(),
    )?;
    Ok(())
}

pub(crate) fn write_storage_file(config: Storage) -> io::Result<()> {
    fs::write(
        ".git/.luxe_storage.json",
        serde_json::to_string_pretty(&config).unwrap(),
    )?;
    Ok(())
}

pub(crate) fn read_storage_file() -> Storage {
    return {
        let data =
            fs::read_to_string(".git/.luxe_storage.json").expect("Error reading storage file");
        serde_json::from_str(&data).unwrap()
    };
}

fn read_config_file() -> RepoConfig {
    return {
        let data =
            fs::read_to_string(".git/.luxe_repo_config.json").expect("Error reading config file");
        serde_json::from_str(&data).unwrap()
    };
}

pub(crate) fn get_trunk_branch() -> String {
    let repo_config = read_config_file();
    return repo_config.trunk;
}

// pub(crate) fn get_parent_branch_stacks() -> Vec<BranchStack> {
//     let branches = read_storage_file().branches;
//     let x: Vec<BranchStack> = branches
//         .into_iter()
//         .map(|(_, score)| score)
//         .filter(|stack| stack.children.len() > 0)
//         .collect();
//     return x;
// }

pub(crate) fn get_branch_by_key(key: String) -> BranchStack {
    let branches = read_storage_file().branches;
    return branches
        .into_iter()
        .map(|(_, score)| score)
        .find(|stack| stack.branch_name == key)
        .unwrap();
}
