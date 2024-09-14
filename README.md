git-linear-branch
----------------------

A simple helper to speed up creating git new branch names from git branches from [Linear][ref_linear].

[ref_linear]:https://linear.app/

To plug `git-linear-branch` to zsh use the following `_fzf_comprun`:

```zsh
export FZF_COMPLETION_TRIGGER='`'

_fzf_comprun() {
  local command=$1
  shift

  case "$command" in
    git-linear-branch)
                  git-linear-branch | fzf --with-nth 2.. | cut -f1
                  ;;
  esac
}
```


