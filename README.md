# ghastly

[![License][license-badge]][license] [![Build Status][build-badge]][build] [![pre-commit.ci status][pre-commit-badge]][pre-commit]

A security-focused linter for Github Actions Workflows, written in Rust 🦀.

## Installation

Just clone the repository and install the crate as usual:

```bash
$ git clone https://github.com/Holzhaus/ghastly.git
$ cd ghastly
$ cargo install --path .
```

Don't forget to make sure that your `$PATH` includes `$HOME/cargo/bin`.

## Usage

To check a workflow file, use the `check` subcommand:

```bash
$ ghastly check .github/workflows/build.yml
.github/workflows/build.yml:14:13:Job 'build' should set 'permissions' field. (permissions_set)
```

All policies are listed using the `list` subcommand:

```bash
$ ghastly list
no_github_expr_in_run
permissions_set
no_all_permissions
...
```

If you need more information on a specific policy, you can use `show`:

```bash
$ ghastly show permissions_set
Every job should set individual permissions for the GITHUB_TOKEN.

Permissions that are unnecessarily broad violate the principle of least privilege. Hence, every
job should specify only the permissions actually needed to perform its tasks by setting the
permissions field.

Note that the permissions field on a job may be omitted if:

1. If the workflow sets the default permissions to none
2. If the workflow sets the permissions field and the there is only one job in the workflow.

...
```

Check the output of the `--help` flag for more information.

## License

This software is [licensed][license] under the terms of the [Mozilla Public License
2.0](https://www.mozilla.org/en-US/MPL/2.0/). Please also have a look at the
[license FAQ](https://www.mozilla.org/en-US/MPL/2.0/FAQ/).


[license]: https://github.com/Holzhaus/ghastly/blob/main/COPYING
[license-badge]: https://img.shields.io/github/license/Holzhaus/ghastly
[build]: https://github.com/Holzhaus/ghastly/actions?query=branch%3Amain
[build-badge]: https://img.shields.io/github/actions/workflow/status/Holzhaus/ghastly/build.yml?branch=main
[pre-commit]: https://results.pre-commit.ci/latest/github/Holzhaus/ghastly/main
[pre-commit-badge]: https://results.pre-commit.ci/badge/github/Holzhaus/ghastly/main.svg
