## git-branch-from-linear

```
git-branch-from-linear push bla/bla-123-bla
```

```
git-branch-from-linear list
```

```
git-branch-from-linear get-branch-name bla/bla-123-bla description of a branch
```

```
git-branch-from-linear checkout-new bla/bla-123-bla description of a branch
```


# UX

- `git-newbranch <tab>`
	- list all the available prefixes
	- there are only top 2 prefixes
	- prefixes are ordered by the Recent Usage
- `git-newbranch prefix some branch name here`
	- combine prefix and the other parts with `-`
	- update Recent Usage of the prefix
	- run external `git checkout -b <the branch name>`
