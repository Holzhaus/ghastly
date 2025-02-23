// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use super::PolicyViolation;
use crate::parser::{
    expression::{tokenize, TokenKind},
    workflow::Workflow,
};
use ghastly_macros::policy;

#[policy]
/// No step should be using a GitHub Actions Expression in the `run` field.
///
/// Instead, the expression should be assigned to an environment variable which is used in the
/// script. The reason for this is that the result of the expression is substituted as-is, which
/// can lead to quoting issues and might be exploitable by an attacker (i.e., script injection).
///
/// # Examples
///
/// ## Not OK: Job uses Expression in `run` field
///
/// The job below uses the `echo` command to print the pull request title. Because the GitHub
/// expression is used directly inside the `run` field, an attacker could use the PR title to inject
/// custom commands.
///
/// For example, a PR with the title `a"; ls "$GITHUB_WORKSPACE"; echo "b` would result that the
/// the code in the `run` field would be set to `echo "a"; ls "$GITHUB_WORKSPACE"; echo "b"`
///
/// ```yaml
/// on: [pull_request]
/// jobs:
///   job-with-expression-in-run:
///     runs-on: ubuntu-latest
///     steps:
///       - run: echo "${{ github.event.pull_request.title }}"
/// ```
///
/// ## OK: Job uses Expression via `env` field
///
/// Jobs that have `write-all` token permission violates this policy.
///
/// ```yaml
/// on: [pull_request]
/// jobs:
///   job-with-expression-in-env:
///     runs-on: ubuntu-latest
///     steps:
///       - run: echo "${PULL_REQUEST_TITLE}"
///         env:
///           PULL_REQUEST_TITLE: ${{ github.event.pull_request.title }}
/// ```
///
/// # References
///
/// - <https://docs.github.com/de/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions#understanding-the-risk-of-script-injections>
/// - <https://docs.github.com/de/actions/security-for-github-actions/security-guides/security-hardening-for-github-actions#good-practices-for-mitigating-script-injection-attacks>
pub fn no_github_expr_in_run(workflow: &Workflow) -> Vec<PolicyViolation> {
    workflow
        .jobs
        .iter()
        .flat_map(|(job_name, job)| {
            job.steps.iter().flat_map(move |steps| steps.iter().enumerate().filter_map(move |(step_index, step)| step.run.as_ref().and_then(|run| {
                if tokenize(run).any(|token| token.kind() == TokenKind::Expression) {
                    Some(PolicyViolation::new(
                        run.span().to_owned(),
                        format!("Step {} of job {} should not directly include GitHub expression in the 'run' field.", step_index + 1, job_name),
                    ))
                } else {
                    None
                }
            })))
        }).collect()
}
