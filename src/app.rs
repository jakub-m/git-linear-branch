use std::env;
use std::process::Command;
use std::string::FromUtf8Error;

use crate::args::Args;
use crate::storage::{BranchInfo, JsonStorage, Storage};
use lazy_static::lazy_static;
use regex::Regex;

const DEFAULT_STORAGE_FILENAME: &str = ".meta-for-git.json";
const SEP: &str = "-";

lazy_static! {
    static ref pat_linear_branch: Regex = Regex::new(r"^([^\s]+/\w+-\d+)").unwrap();
}

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    let storage = JsonStorage::new(DEFAULT_STORAGE_FILENAME)?;
    if args.args.is_empty() {
        list_branches(&storage)?;
        return Ok(());
    } else {
        let first_arg = args.args.get(0).unwrap();
        let prefix: String;
        let branch_name_parts: &[String];
        if looks_like_branch(&first_arg) || is_branch(&storage, &first_arg)? {
            prefix = first_arg.to_owned();
            branch_name_parts = &args.args[1..];
        } else {
            prefix = take_latest_prefix(&storage)?;
            branch_name_parts = &args.args;
        }
        let full_branch_name = construct_full_branch_name(&prefix, branch_name_parts);
        checkout_branch(&full_branch_name)?;
        return update_last_used_prefix(&storage, &prefix, &full_branch_name);
    }
}

fn looks_like_branch(branch: &str) -> bool {
    get_linear_prefix(branch).is_some()
}

fn is_branch(storage: &dyn Storage, branch_name: &str) -> Result<bool, String> {
    let found_branch = storage
        .list_branch_info()?
        .iter()
        .find(|b| b.prefix == branch_name)
        .map_or(false, |_| true);
    Ok(found_branch)
}

fn list_branches(storage: &dyn Storage) -> Result<(), String> {
    let mut branches = storage.list_branch_info()?;
    branches.sort_by_key(|b| -b.last_used.timestamp());
    for b in storage.list_branch_info()? {
        let prefix = b.prefix;
        println!("{prefix}");
    }
    Ok(())
}

fn get_linear_prefix<'a>(branch_name: &'a str) -> Option<&'a str> {
    pat_linear_branch.find(branch_name).map(|s| s.as_str())
}

fn take_latest_prefix(storage: &dyn Storage) -> Result<String, String> {
    let mut branches = storage.list_branch_info()?;
    branches.sort_by_key(|b| -b.last_used.timestamp());
    branches
        .get(0)
        .map_or(Err("no branches saved".to_string()), |b| {
            Ok(b.prefix.to_owned())
        })
}

fn construct_full_branch_name(prefix: &str, branch_name_parts: &[String]) -> String {
    let branch_name = branch_name_parts.join(SEP);
    format!("{prefix}{SEP}{branch_name}")
}

fn checkout_branch(branch_name: &str) -> Result<(), AppError> {
    let output = Command::new("git")
        .args(["checkout", "-b", branch_name])
        .output()?;
    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8(output.stdout)?)
    }
    if !output.stderr.is_empty() {
        println!("{}", String::from_utf8(output.stderr)?)
    }
    match output.status.success() {
        true => Ok(()),
        false => Err(AppError {
            message: "failed to checkout branch".to_string(),
        }),
    }
}

fn update_last_used_prefix(
    storage: &dyn Storage,
    prefix: &str,
    branch_name: &str,
) -> Result<(), String> {
    let info = BranchInfo {
        prefix: prefix.to_owned(),
        last_used: chrono::Utc::now(),
        name: branch_name.to_owned(),
    };
    storage.store_branch_info(&info)?;
    Ok(())
}

struct AppError {
    message: String,
}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError {
            message: format!("IO error: {}", value.to_string()),
        }
    }
}

impl From<FromUtf8Error> for AppError {
    fn from(value: FromUtf8Error) -> Self {
        AppError {
            message: format!("UTF8 error: {}", value.to_string()),
        }
    }
}

impl From<AppError> for String {
    fn from(value: AppError) -> Self {
        value.message
    }
}
