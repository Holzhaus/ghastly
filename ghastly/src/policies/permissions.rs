// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use super::PolicyViolation;
use crate::parser::workflow::{PermissionLevel, Permissions, Workflow};
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

#[policy]
/// Check that every job sets individual permissions for the `GITHUB_TOKEN`.
///
/// Permissions that are unnecessarily broad violate the principle of least privilege. Hence, every
/// job should specify only the permissions actually needed to perform its tasks by setting the
/// `permissions` field.
///
/// Note that the `permissions` field on a job may be omitted if:
///
/// 1. If the workflow sets the default permissions to `none`
/// 2. If the workflow sets the permissions field and the there is only one job in the workflow.
///
/// # Examples
///
/// ## Not OK: Neither Workflow not Job specifies `permissions` Field
///
/// ```yaml
/// on: [push]
/// jobs:
///   job-without-permissions:
///     runs-on: ubuntu-latest
///     steps:
///       - run: echo "Too many permissions"
/// ```
///
/// ## Not OK: Jobs that do not set `permissions` in a Multi-Job Workflow
///
/// If a workflow with multiple jobs specifies permissions on the workflow level, but not on the
/// job level, this policy is violated because all jobs in the same workflow will get the same
/// permissions even though some may not need it.
///
/// ```yaml
/// on: [push]
/// permissions:
///     content: write
/// jobs:
///   job-that-writes-content:
///     runs-on: ubuntu-latest
///     steps:
///       - run: echo "This job needs 'content: write' permission"
///   job-that-does-not-write-content:
///     runs-on: ubuntu-latest
///     steps:
///       - run: echo "This job has 'content: write' permission, but doesn't need it"
/// ```
///
/// ## OK: Job sets `permissions` Field
///
/// Jobs that set token permissions do not violate this policy.
///
/// ```yaml
/// on: [push]
/// jobs:
///   job-with-permissions:
///     runs-on: ubuntu-latest
///     permissions:
///         contents: read
///     steps:
///       - run: echo "This is okay"
/// ```
///
/// ## OK: Single-Job Workflow with `permissions` field
///
/// In a workflow with only a single job, the `permission` field may be set on the workflow level
/// instead.
///
/// ```yaml
/// on: [push]
/// permissions:
///     contents: read
/// jobs:
///   foo:
///     runs-on: ubuntu-latest
///     steps:
///       - run: echo "This is okay"
/// ```
///
/// # References
///
/// - <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#defining-access-for-the-github_token-scopes>
/// - <https://en.wikipedia.org/wiki/Principle_of_least_privilege>
/// ```
pub fn permissions_set(workflow: &Workflow) -> Vec<PolicyViolation> {
    // If the workflow sets the default permissions to `none`, the job's `permissions` field may be
    // omitted.
    if workflow.permissions.as_ref().is_some_and(|permissions| {
        if let Permissions::Event(permission_event) = permissions.deref() {
            permission_event
                .iter()
                .all(|(_name, permission)| permission == PermissionLevel::None)
        } else {
            false
        }
    }) {
        return vec![];
    }

    // If the workflow specifies permissions and the workflow has only one job, we don't care if
    // they job also specifies permissions or not.
    if workflow.permissions.is_some() && workflow.jobs.len() <= 1 {
        return vec![];
    }

    // Otherwise, every job should specify permissions separately.
    workflow
        .jobs
        .iter()
        .filter_map(|(job_name, job)| {
            if job.permissions.is_none() {
                Some(PolicyViolation::new(
                    job.span().to_owned(),
                    format!("Job '{}' should set 'permissions' field.", job_name),
                ))
            } else {
                None
            }
        })
        .collect()
}
