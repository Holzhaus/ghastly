// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use super::PolicyViolation;
use crate::parse::{Permissions, Workflow};
use ghastly_macros::policy;
use std::ops::Deref;

#[policy]
/// Check that no Jobs are using `read-all` or `write-all` permissions for the `GITHUB_TOKEN`.
///
/// Permissions that are unnecessarily broad violate the principle of least privilege.
///
/// # Examples
///
/// ## Not OK: Jobs with `read-all` token permission
///
/// Jobs that have `read-all` token permission violates this policy.
///
/// ```yaml
/// name: Job with read-all token permission
/// jobs:
///   foo:
///     runs-on: ubuntu-latest
///     permissions: read-all
///     steps:
///       - run: echo "Too many permissions"
/// ```
///
/// ## Not OK: Jobs with `write-all` token permission
///
/// Jobs that have `write-all` token permission violates this policy.
///
/// ```yaml
/// name: Job with write-all token permission
/// jobs:
///   foo:
///     runs-on: ubuntu-latest
///     permissions: read-all
///     steps:
///       - run: echo "Too many permissions"
/// ```
///
/// ## OK: Jobs without fine-grained token permissions
///
/// Jobs that do not set token permission do not violate this policy.
///
/// ```yaml
/// name: Job with write-all token permission
/// jobs:
///   foo:
///     runs-on: ubuntu-latest
///     permissions:
///         contents: read
///     steps:
///       - run: echo "This is okay"
/// ```
/// ## OK: Jobs with fine-grained token permissions
///
/// Jobs that do not set token permission do not violate this policy.
///
/// ```yaml
/// name: Job with write-all token permission
/// jobs:
///   foo:
///     runs-on: ubuntu-latest
///     permissions:
///         contents: read
///     steps:
///       - run: echo "This is okay"
/// ```
///
/// # References
///
/// - <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#defining-access-for-the-github_token-scopes>
/// - <https://en.wikipedia.org/wiki/Principle_of_least_privilege>
/// ```
pub fn no_all_permissions(workflow: &Workflow) -> Vec<PolicyViolation> {
    workflow
        .jobs
        .iter()
        .filter_map(|(job_name, job)| {
            job.permissions
                .as_ref()
                .and_then(|permissions| match permissions.deref() {
                    Permissions::ReadAll => Some(PolicyViolation::new(
                        permissions.span().to_owned(),
                        format!("Job {} should not use the 'read-all' permission.", job_name),
                    )),
                    Permissions::WriteAll => Some(PolicyViolation::new(
                        permissions.span().to_owned(),
                        format!(
                            "Job {} should not use the 'write-all' permission.",
                            job_name
                        ),
                    )),
                    _ => None,
                })
        })
        .collect()
}
