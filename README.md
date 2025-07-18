# git-user

🚀 A no-nonsense CLI to switch and manage Git user profiles on a per-repo basis.

## Why?

If you use different Git identities for work and personal projects, `git-user` makes it easy to switch between them locally — without editing `.git/config` manually or forgetting which identity is active.

## Features

- 🔁 Quickly switch between saved Git profiles
- ➕ Add new profiles via interactive prompts
- 📍 Applies changes only to the current Git repo (not global)

## Installation

### via Homebrew (recommended)

```bash
brew tap forge-weaver/tap
brew install git-user
```

## Usage

```bash
git-user
```

- Pick a profile from the list, or add a new one.
- Instantly updates user.name and user.email for the current repo.

## Inspiration

This project was inspired by [git-user-switch](https://www.npmjs.com/package/git-user-switch) — a great NPM-based CLI tool for switching Git profiles.
git-user is a Rust-based reimplementation with a focus on speed, portability, and no Node.js dependency.
