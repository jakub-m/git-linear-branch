use std::env;
use std::process::Command;

use crate::args::Args;
use crate::err::AppError;
use crate::storage::{BranchInfo, JsonStorage, Storage};
use lazy_static::lazy_static;
use regex::Regex;

const DEFAULT_STORAGE_FILENAME: &str = ".git-linear-branch-meta.json";
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
        let full_branch_name: String;
        if is_stored_prefix(&storage, &first_arg)? || looks_like_branch(&first_arg) {
            prefix = get_linear_prefix(&first_arg)
                .expect(&format!(
                    "bug: expected {} to be a legal prefix here",
                    first_arg
                ))
                .to_string();
            if args.args.len() == 1 {
                // Single arg. is treat as a whole new branch name
                full_branch_name = first_arg.to_owned();
            } else {
                full_branch_name = construct_full_branch_name(&prefix, &args.args[1..]);
            }
        } else {
            prefix = take_latest_prefix(&storage)?;
            full_branch_name = construct_full_branch_name(&prefix, &args.args);
        }
        checkout_branch(&full_branch_name)?;
        return update_last_used_prefix(&storage, &prefix, &full_branch_name);
    }
}

fn looks_like_branch(branch: &str) -> bool {
    get_linear_prefix(branch).is_some()
}

fn is_stored_prefix(storage: &dyn Storage, branch_name: &str) -> Result<bool, String> {
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
    for b in branches {
        let prefix = b.prefix;
        let branch_name = b.name;
        println!("{prefix}\t{branch_name}");
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
    let parts = [&[prefix.to_string()], branch_name_parts].concat();
    parts.join(SEP)
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
        false => Err(AppError::new("failed to checkout branch")),
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
    storage.push_branch_info(&info)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::get_linear_prefix;

    #[test]
    fn get_prefix() {
        assert_eq!(get_linear_prefix("foo"), None);
        assert_eq!(get_linear_prefix("foo/bar"), None);
        assert_eq!(get_linear_prefix("foo/bar-123"), Some("foo/bar-123"));
        assert_eq!(get_linear_prefix("foo/bar-123-quux"), Some("foo/bar-123"));
    }
}
