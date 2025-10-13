# Contributing to MDTex

Thanks for your interest in contributing! Please follow the below guidelines to ensure the project remains organized and maintainable.

---

## Table of Contents
1. [Getting Started](#getting-started)
2. [Branching Structure](#branching-structure)
3. [Making Changes](#making-changes)
4. [Commit Guidelines](#commit-guidelines)
5. [Pull Request Process](#pull-request-process)

---

## Getting Started

1. Fork the repository
2. Clone your fork locally:
    ```bash
    git clone https://github.com/your-username/fa25-team079
    ```
3. Add the upstream repository:
    ```bash
    git remote add upstream https://github.com/CS222-UIUC/fa25-team079.git
    ```
4. Install Rust and dependencies:
    ```bash
    rustup install stable
    cargo build
    ```

---

## Branching Structure

`main`: Production branch. Will be merged from `develop` after thorough testing.  
`develop`: Active development branch. Feature branches branch off of `develop`.  
`feat-*`: Feature branch. Should be named descriptively. Merge back into develop upon completion.  

---

## Making Changes

1. Create a feature branch off develop
2. Make your changes with clear, concise commits
3. Run tests locally
4. Push your branch and submit a pull request to `develop`

---

## Commit Guidelines

- Keep commits all lowercase and concise, but informative
- Avoid vague messages such as `update` or `changes`

---

## Pull Request Process

1. Submit your PR to `develop`
2. Each PR will be approved by at least two reviewers prior to merging
