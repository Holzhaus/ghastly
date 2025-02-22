// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use marked_yaml::Spanned;
use serde::Deserialize;
use serde_either::StringOrStruct;
use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::io::Read;
use std::str::FromStr;

pub type Map<T> = BTreeMap<String, Spanned<T>>;
pub type StringMap = Map<String>;

/// A GitHub Actions workflow.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Workflow {
    /// The name of the workflow.
    ///
    /// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#name>
    pub name: Option<Spanned<String>>,
    /// The name for workflow runs generated from the workflow.
    ///
    /// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#run-name>
    pub run_name: Option<Spanned<String>>,
    ///// Defines which events can cause the workflow to run.
    /////
    ///// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#on>
    //on: Spanned<WorkflowTrigger>,
    /// Sets the default permissions granted to the `GITHUB_TOKEN`.
    ///
    /// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#permissions>
    pub permissions: Option<Spanned<Permissions>>,
    /// Environment variables that are available to the steps of all jobs in the workflow.
    ///
    /// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#env>
    pub env: Option<Spanned<StringMap>>,
    ///// Default settings that will apply to all jobs in the workflow.
    /////
    ///// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#defaults>
    //pub defaults: Option<Spanned<Node>>,
    ///// Sets a concurrency group that ensures that only a single job or workflow using the same group will run at a time.
    /////
    ///// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#concurrency>
    //pub concurrency: Option<Spanned<Node>>,
    /// Defines jobs that are part of this workflow.
    ///
    /// Documentation: <https://docs.github.com/en/actions/writing-workflows/workflow-syntax-for-github-actions#jobs>
    pub jobs: Spanned<Map<Job>>,
}

/// Token Permission Settings
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(try_from = "StringOrStruct<PermissionEvent>")]
#[serde(rename_all = "kebab-case")]
pub enum Permissions {
    /// `read-all` token permissions
    ReadAll,
    /// `write-all` token permissions
    WriteAll,
    /// Fine-grained token permissions
    Event(PermissionEvent),
}

impl TryFrom<StringOrStruct<PermissionEvent>> for Permissions {
    type Error = String;

    fn try_from(value: StringOrStruct<PermissionEvent>) -> Result<Self, Self::Error> {
        match value {
            StringOrStruct::String(s) => Permissions::from_str(&s),
            StringOrStruct::Struct(s) => Ok(Permissions::Event(s)),
        }
    }
}

impl FromStr for Permissions {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "read-all" => Ok(Permissions::ReadAll),
            "write-all" => Ok(Permissions::WriteAll),
            other => Err(format!("unknown global permission {:?}", other)),
        }
    }
}

/// Fine-Grained Token Permissions
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct PermissionEvent {
    /// Work with GitHub Actions.
    pub actions: Option<PermissionLevel>,
    /// Work with artifact attestations.
    pub attestations: Option<PermissionLevel>,
    /// Work with check runs and check suites.
    pub checks: Option<PermissionLevel>,
    /// Work with check runs and check suites.
    pub contents: PermissionLevel,
    /// Work with deployments.
    pub deployments: Option<PermissionLevel>,
    /// Work with GitHub Discussions.
    pub discussions: Option<PermissionLevel>,
    /// Fetch an OpenID Connect (OIDC) token.
    pub id_token: Option<PermissionLevel>,
    /// Work with issues.
    pub issues: Option<PermissionLevel>,
    /// Work with GitHub Packages.
    pub packages: Option<PermissionLevel>,
    /// Work with GitHub Pages.
    pub pages: Option<PermissionLevel>,
    /// Work with pull requests.
    pub pull_requests: Option<PermissionLevel>,
    /// Work with GitHub projects (classic).
    pub repository_projects: Option<PermissionLevel>,
    /// Work with GitHub code scanning and Dependabot alerts.
    pub security_events: Option<PermissionLevel>,
    /// Work with commit statuses.
    pub statuses: Option<PermissionLevel>,
}

/// Work with commit statuses.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum PermissionLevel {
    /// Read permission.
    Read,
    /// Write permission (includes read permission).
    Write,
    /// No permission.
    None,
}

/// A job in a GitHub workflow.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Job {
    /// Token permissions for the job.
    pub permissions: Option<Spanned<Permissions>>,
    /// Defines the type of machine to run the job on.
    pub runs_on: Spanned<String>,
    /// Override the default shell settings in the runner's operating system and sets it as the
    /// job's default.
    pub shell: Option<Spanned<String>>,
    /// A sequence of tasks that are run for this job.
    pub steps: Option<Spanned<Vec<Step>>>,
}

/// A task that is run as part of Job.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct Step {
    /// A unique identifier for the step.
    pub id: Option<Spanned<String>>,
    /// A conditional to prevent a step from running unless a condition is met.
    #[serde(rename = "if")]
    pub condition: Option<Spanned<String>>,
    /// A name for your step to display on GitHub.
    pub name: Option<Spanned<String>>,
    /// Selects an action to run as part of a step in the job.
    pub uses: Option<Spanned<String>>,
    /// Runs command-line programs using the operating system's shell.
    pub run: Option<Spanned<String>>,
    /// Specifies the working directory of where to run the command.
    pub working_directory: Option<Spanned<String>>,
    /// Overrides the default shell settings in the runner's operating system and the job's
    /// default.
    pub shell: Option<Spanned<String>>,
    /// A map of the input parameters defined by the action.
    pub with: Option<Spanned<StringMap>>,
    /// Sets variables for steps to use in the runner environment.
    pub env: Option<Spanned<StringMap>>,
    //pub continue_on_error: Option<Spanned<ContinueOnError>>,
    //pub timeout_minutes: Option<Spanned<Timeout>>,
}

/// Parse a workflow from the given reader.
pub fn parse_workflow<R>(reader: &mut R) -> crate::Result<Workflow>
where
    R: Read,
{
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    let workflow: Workflow = marked_yaml::from_yaml(0, &buffer)?;
    Ok(workflow)
}
