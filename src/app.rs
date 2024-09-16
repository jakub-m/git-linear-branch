use std::env;
use std::process::Command;

use crate::args::Args;
use crate::err::AppError;
use crate::storage::{BranchInfo, JsonStorage, Storage};
use lazy_static::lazy_static;
use regex::Regex;

const DEFAULT_STORAGE_FILENAME: &str = ".git-linear-branch-meta.json";
const DEFAULT_LAST_BRANCHES_COUNT: usize = 3;
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
        //let prefix: String;
        //let full_branch_name: String;

        // - If the user passed a full branch name:
        //   - Check if there is stored branch with prefix for the full branch name.
        //   - If yes, update, if not, create new.
        // - If the user passed prefix and reminder
        //   - If the prefix exists, update
        //   - otherwise, create new
        // - If the user passed a reminder
        //    - find latest branch
        //    - update
        let now = chrono::Utc::now();
        let branch = if let Some(prefix) = get_linear_prefix(&first_arg) {
            let full_branch_name = construct_full_branch_name(prefix, &args.args[1..]);
            match storage.get_by_prefix(prefix)? {
                Some(branch) => BranchInfo {
                    name: full_branch_name.to_owned(),
                    last_used: now,
                    ..branch
                },
                None => BranchInfo {
                    prefix: prefix.to_owned(),
                    name: full_branch_name.to_owned(),
                    last_used: now,
                    original_title: title_from_branch_name(&full_branch_name),
                },
            }
        } else {
            let prefix = take_latest_prefix(&storage)?;
            let full_branch_name = construct_full_branch_name(&prefix, &args.args);
            BranchInfo {
                prefix,
                last_used: now,
                name: full_branch_name.to_owned(),
                original_title: title_from_branch_name(&full_branch_name),
            }
        };

        checkout_branch(&branch.name)?;
        storage.push_branch_info(&branch)?;
        storage.trim_to_latest(DEFAULT_LAST_BRANCHES_COUNT)?;
        Ok(())
    }
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

fn title_from_branch_name(s: &str) -> String {
    // TODO
    return s.to_owned();
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
