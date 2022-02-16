# Syncing Rustlings

- [Git Subtree](https://www.atlassian.com/git/tutorials/git-subtree)

```bash
git remote add -f rustlings https://github.com/rust-lang/rustlings.git

git subtree add --prefix rustling_exercises rustlings main --squash

git fetch rustlings main
git subtree pull --prefix rustling_exercises rustlings main --squash
```
