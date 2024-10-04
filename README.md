## git-linear-branch

A simple helper to speed up creating git new branch names from git branches from [Linear][ref_linear]. The tool remembers the last Linear-like branch prefix for a directory.

[ref_linear]: https://linear.app/

## Installation

```
make release
```

Then make the `git-linear-branch` accessible from `$PATH`.

To plug `git-linear-branch` to zsh use the following `_fzf_comprun`:

```zsh
export FZF_COMPLETION_TRIGGER='`'

_fzf_comprun() {
  local command=$1
  shift

  case "$command" in
    git-linear-branch)
                  git-linear-branch \
                      | fzf \
                        --with-nth 2.. \
                        --bind 'ctrl-d:reload(git-linear-branch --delete-prefix {1} && git-linear-branch)' \
                      | cut -f1

                  ;;
  esac
}
```

## Usage

The tool keeps last 3 prefixes.

List all the prefixes:

```bash
git-linear-branch `<tab>
```

Create a branch with selected prefix:

```bash
git-linear-branch foo/bar-123 hello world
```

Create a branch from a full branch name (copied from Linear)

```bash
git-linear-branch foo/bar-123-hello-world
```

Create branch with the last prefix selected implicitly:

```bash
git-linear-branch hello world
```

Delete prefix from storage:

```bash
git-linear-branch --delete-prefix foo/bar-123
```
