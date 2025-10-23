# Contributing to MDTex

Thanks for your interest in contributing! Please follow the below guidelines to ensure the project remains organized and maintainable.

---

# Table of Contents

1. [Getting Started](#getting-started)
2. [Branching Structure](#branching-structure)
3. [Making Changes](#making-changes)
4. [Commit Guidelines](#commit-guidelines)
5. [Pull Request Process](#pull-request-process)

---

## Getting Started

1. Install Rust from [rustup.rs](https://rustup.rs/) :
1. Clone the repository locally:
   ```bash
   git clone https://github.com/your-username/fa25-team079
   ```

---

## Branching Structure

| Branch Name                           | Function                                                                                                                                                                                                 |
| ------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `main`                                | This is the production branch. It must always compile.                                                                                                                                                   |
| `develop`                             | Active development branch. Feature branches are branched and merged back into this branch.                                                                                                               |
| `feat/*` or `feat/*-<issue number>`   | Feature branch. Should be named descriptively. Merge back into develop upon completion.                                                                                                                  |
| `chore/*` or `chore/*-<issue number>` | Chore branch. Semantically identical to a feature branch, except that these branches should be used to do chores, such as updating READMEs, changing deps, etc                                           |
| `release-v<release-number>`           | A release branch. Release branches should be branched off of `develop` once enough features to constitute a complete release are ready. This branch will be tested for bugs, and then merged with `main` |
| `hotfix/*-<issue number>`             | A hotfix branch. Branched off `main`, and merged into both `main` and `develop`. When possible, no branches should make it into `main`.                                                                  |

## Branch Lifecycle

### `main` Branch

This branch is permanent. It is the production branch and is the version of the product ready for user consumption.
Every commit here that results from a merge from a `release` branch should be tagged `v<version number>`

### `develop` Branch

This branch is permanent. This is the main development branch.

### Feature/Chore Branches

These branches are branched off of `develop`. They are then worked on. Once ready, a pull request from the branch is made onto `develop`.
Once approved and merged into `develop`, the branch must be deleted.

### Release Branches

Once enough features to constitute a release are merged into `develop` (and a release is desired), a `release` branch is branched off of `develop`.
For example a release branch from version 1.0 would be named `release-v1.0`.
This branch will be prepared for final release, with final fixes, chores, and other tasks being done on it.
Once done, the branch is merged (via pull request) into both `main` and `develop` (the logic here being so that `develop` benefits from any last minute fixes in the release branch).
Once merged, the branch must be deleted.

### Hotfix Branches

In general, all code pushed to `develop` should try to be bug-free. Any bugs that somehow end up in `develop` should in theory be caught before they reach `main`
when the `release` branch for the corresponding release is worked on. If a bug _somehow_ reaches `main` a `hotfix` branch is branched off of `main`.
The bug is then fixed on this branch. Once the bug is fixed, the bug is merged via pull request into `main` and `develop`. The branch is then deleted.

# Making Changes

For a feature branch a workflow would look like this:

1. Create a feature branch off `develop`
2. Make your changes with clear, concise commits
3. Run tests locally
4. Push your branch and submit a pull request to `develop`

# Commit Guidelines

- Keep commits all lowercase and concise, but informative
- Avoid vague messages such as `update` or `changes`, instead specifying what capabilities you've added or changed.

# Pull Request Process

1. Submit your PR to `develop`
2. Each PR will be approved by at least two reviewers prior to merging
