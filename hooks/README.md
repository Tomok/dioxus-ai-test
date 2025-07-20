# Git Hooks

This directory contains Git hooks that enforce code quality standards for the project.

## Available Hooks

### pre-commit

The pre-commit hook ensures code quality by running:
- `cargo fmt --check` - Validates Rust code formatting
- `cargo clippy -- -D warnings` - Checks for clippy warnings

This hook enforces the mandatory code quality checks specified in `CLAUDE.md`.

## Setup

To use these hooks in your local repository, run the following command from the project root:

```bash
git config core.hooksPath hooks
```

This configures Git to use the hooks in this directory instead of the default `.git/hooks/` directory.

## Verification

To test that the pre-commit hook is working, try making a commit. The hook will run automatically and prevent the commit if code quality checks fail.

You can also manually run the hook to test it:

```bash
./hooks/pre-commit
```