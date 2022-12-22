mod cache;
mod git;

use std::collections::HashMap;

use cache::{
    get_trunk_branch, write_config_file, write_storage_file, BranchStack, RepoConfig, Storage,
};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use git::{add_all, commit};

use crate::{
    cache::{get_branch_by_key, read_storage_file},
    git::branch::{get_current_branch_name, switch_branch},
};

#[derive(Debug, Parser)]
#[command(name = "sd")]
#[command(about = "A versioning CLI for stacked diffs", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Branch(Branch),
    Commit(Commit),
    Init(Init),
    List(List),
    Pull(Pull),
}

#[derive(Debug, Args)]
struct Pull {}

#[derive(Debug, Args)]
struct Commit {
    #[arg(short, long)]
    all: bool,
    #[arg(short, long)]
    message: Option<String>,
}

#[derive(Debug, Args)]
struct List {}

#[derive(Debug, Args)]
struct Init {}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
struct Branch {
    #[command(subcommand)]
    command: Option<BranchCommands>,
}

#[derive(Debug, Subcommand)]
enum BranchCommands {
    Track(BranchTrack),
    Create(BranchCreate),
}

#[derive(Debug, Args)]
struct BranchTrack {}

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
struct BranchCreate {
    branch: String,
}

#[derive(Debug)]
struct FullBranchList {
    branch_name: String,
    children: Vec<FullBranchList>,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Branch(branch) => {
            let branch_cmd = branch.command.unwrap();
            match branch_cmd {
                BranchCommands::Track(_) => {
                    let branch = &git::branch::get_current_branch_name();
                    let mut storage = read_storage_file();
                    let all_branches = &storage.branches;

                    if all_branches.contains_key(branch) {
                        let existing_branch = all_branches.get(branch).unwrap();
                        println!(
                            "{} is already tracked and is a parent of {}",
                            branch,
                            existing_branch.parent_branch_name.as_ref().unwrap()
                        );
                        return;
                    }

                    let new_branch_stack = BranchStack {
                        branch_name: branch.to_string(),
                        parent_branch_name: Some(get_trunk_branch()),
                        children: Vec::new(),
                    };
                    storage
                        .branches
                        .insert(branch.to_string(), new_branch_stack);
                    let result = write_storage_file(storage);

                    if result.is_err() {
                        eprintln!("{}", result.unwrap_err());
                    }
                }
                BranchCommands::Create(create) => {
                    let new_branch = &create.branch;

                    let parent_branch = &get_current_branch_name();
                    let mut storage = read_storage_file();
                    let all_branches = &storage.branches;

                    if !all_branches.contains_key(parent_branch) {
                        println!(
                            "{} {}",
                            parent_branch.to_string().red().bold(),
                            "is not a valid parent branch. Please track branch first".red(),
                        );
                        return;
                    }

                    if all_branches.contains_key(new_branch) {
                        println!(
                            "{} {}",
                            new_branch.to_string().red().bold(),
                            "already exists. Please give this branch a new name".red(),
                        );
                        return;
                    }

                    let new_branch_stack = BranchStack {
                        branch_name: create.branch.to_string(),
                        parent_branch_name: Some(parent_branch.to_string()),
                        children: Vec::new(),
                    };

                    let updated_branch = all_branches.get(parent_branch).unwrap();
                    let mut updated_children = updated_branch.children.clone();
                    updated_children.push(create.branch.to_string());

                    let updated_branch_stack = BranchStack {
                        branch_name: updated_branch.branch_name.to_string(),
                        parent_branch_name: updated_branch.parent_branch_name.clone(),
                        children: updated_children,
                    };

                    storage.branches.remove(parent_branch);
                    storage
                        .branches
                        .insert(parent_branch.to_string(), updated_branch_stack);
                    storage
                        .branches
                        .insert(create.branch.to_string(), new_branch_stack);
                    let result = write_storage_file(storage);

                    switch_branch(&create.branch, true, false, false);
                    if result.is_err() {
                        eprintln!("{}", result.unwrap_err());
                    }
                }
            }
        }
        Commands::Commit(commit) => {
            let is_all = commit.all;
            let message = &commit.message.unwrap();

            println!("{} - {}", is_all, message);
            if is_all {
                add_all::add_all();
            }

            commit::commit(message, false);
        }
        Commands::List(_) => {
            let trunk = &get_trunk_branch();
            let trunk_branch = get_branch_by_key(trunk.to_string());

            let full_branch_list = FullBranchList {
                branch_name: trunk.to_string(),
                children: get_children(trunk_branch),
            };

            print_full_branch_list(full_branch_list);
        }
        Commands::Pull(_) => {
            let branch_name = git::branch::get_current_branch_name();
            git::pull::pull(branch_name);
        }
        Commands::Init(_) => {
            let repo_config = RepoConfig {
                trunk: "main".to_string(),
            };
            let result = write_config_file(repo_config);

            let storage = Storage {
                branches: HashMap::new(),
            };
            let storage_result = write_storage_file(storage);

            if result.is_err() || storage_result.is_err() {
                println!("{}", "Unable to initialize repo".red());
            } else {
                println!("Successfully initialized repository!")
            }
        }
    }
}

fn get_children(branch: BranchStack) -> Vec<FullBranchList> {
    let mut children = Vec::new();

    for child in branch.children {
        let child_branch = get_branch_by_key(child.to_string());
        let child_branch = FullBranchList {
            branch_name: child_branch.branch_name.to_string(),
            children: get_children(child_branch),
        };
        children.push(child_branch);
    }

    return children;
}

fn print_full_branch_list(branches: FullBranchList) {
    println!("{}", branches.branch_name.color(get_branch_color(0)));
    print_children(branches.children, 0);
}

fn print_children(children: Vec<FullBranchList>, mut i: i32) {
    i += 1;
    for child in children {
        let mut spacing = "".to_string();
        for _ in 0..i {
            spacing += "  ";
        }
        println!("{spacing}{}", child.branch_name.color(get_branch_color(i)));
        print_children(child.children, i);
    }
}

fn get_branch_color(i: i32) -> &'static str {
    match i {
        0 => "blue",
        1 => "cyan",
        2 => "green",
        3 => "yellow",
        4 => "magenta",
        _ => "Red",
    }
}
