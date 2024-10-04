use std::env;
use std::process::Command;

use crate::args::Args;
use crate::err::AppError;
use crate::storage::{BranchInfo, JsonStorage, Storage};
use lazy_static::lazy_static;
use regex::Regex;

const DEFAULT_STORAGE_FILENAME: &str = ".git-linear-branch-meta.json";
const DEFAULT_LAST_BRANCHES_COUNT: usize = 5;
const SEP: &str = "-";

lazy_static! {
    static ref pat_linear_branch: Regex = Regex::new(r"^([^\s]+/\w+-\d+)").unwrap();
}

pub fn run() -> Result<(), String> {
    let args = Args::from_args(&env::args().collect())?;
    let storage_file_path = get_git_root_directory()? + "/" + DEFAULT_STORAGE_FILENAME;
    let storage = JsonStorage::new(&storage_file_path)?;
    if args.args.is_empty() {
        list_branches(&storage)?;
        return Ok(());
    } else {
        let first_arg = args.args.get(0).unwrap();
        if first_arg == "--delete" {
            handle_delete()
        } else {
            handle_checkout(first_arg, &args, &storage)
        }
    }
}

fn list_branches(storage: &dyn Storage) -> Result<(), String> {
    let mut branches = storage.list_branch_info()?;
    branches.sort_by_key(|b| -b.last_used.timestamp());
    for b in branches {
        let prefix = b.prefix;
        let title = b.original_title;
        println!("{prefix}\t{title}");
    }
    Ok(())
}

fn handle_checkout(first_arg: &str, args: &Args, storage: &dyn Storage) -> Result<(), String> {
    let now = chrono::Utc::now();
    let branch = if let Some(prefix) = get_linear_prefix(&first_arg) {
        let full_branch_name = match &args.args[..] {
            [] => return Err("missing args".to_string()),
            [branch_name] => branch_name.to_owned(),
            [_branch_name, rest @ ..] => construct_full_branch_name(prefix, rest),
        };

        match storage.get_by_prefix(prefix)? {
            Some(branch) => BranchInfo {
                name: full_branch_name.to_owned(),
                last_used: now,
                ..branch
            },
            // Insert a new branch into the storage.
            None => BranchInfo {
                prefix: prefix.to_owned(),
                name: full_branch_name.to_owned(),
                last_used: now,
                // Use "first_arg" and not the full branch name, because the first arg is the original branch name
                // from Linear, and we want to use that branch as the reference branch.
                original_title: title_from_branch_name(&first_arg),
            },
        }
    } else {
        let prefix = take_latest_prefix(storage)?;
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

fn handle_delete() -> Result<(), String> {
    todo!();
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

/// Turn this "foo/foo-123-zzz-zzz2"
/// into this "FOO-123 zzz zzz2"
fn title_from_branch_name(s: &str) -> String {
    let pat = Regex::new(r"^(\w+)/(\w+-\d+)((-[\w\d]+)*)$").unwrap();
    if let Some(caps) = pat.captures(s) {
        let ticket_id = caps.get(2).unwrap().as_str().to_uppercase();
        let rest = caps.get(3).unwrap().as_str().replace("-", " ");
        format!("{ticket_id}:{rest}")
    } else {
        s.to_owned()
    }
}

fn get_git_root_directory() -> Result<String, AppError> {
    let output = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()?;
    if output.status.success() {
        let path = String::from_utf8(output.stdout).unwrap();
        Ok(path.trim().to_owned())
    } else {
        Err(AppError::new(&String::from_utf8(output.stderr).unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use crate::app::title_from_branch_name;

    use super::get_linear_prefix;

    #[test]
    fn test_get_prefix() {
        assert_eq!(get_linear_prefix("foo"), None);
        assert_eq!(get_linear_prefix("foo/bar"), None);
        assert_eq!(get_linear_prefix("foo/bar-123"), Some("foo/bar-123"));
        assert_eq!(get_linear_prefix("foo/bar-123-quux"), Some("foo/bar-123"));
    }

    #[test]
    fn test_title_from_branch_name() {
        assert_eq!(
            title_from_branch_name("foo/bar-123-aaa-bbb"),
            "BAR-123: aaa bbb"
        );
    }
}
