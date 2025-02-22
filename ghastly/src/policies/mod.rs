// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use crate::parse::Workflow;
use marked_yaml::Span;

mod permissions;

pub type PolicyCheckFn = fn(workflow: &Workflow) -> Vec<PolicyViolation>;

#[derive(Debug, Clone)]
pub struct Policy {
    pub name: &'static str,
    check_fn: PolicyCheckFn,
}

impl Policy {
    pub const fn new(name: &'static str, check_fn: PolicyCheckFn) -> Self {
        Self { name, check_fn }
    }

    #[inline]
    pub fn check<'a>(&'a self, workflow: &Workflow) -> PolicyCheckOutput<'a> {
        let violations = (self.check_fn)(workflow);
        PolicyCheckOutput::new(self, violations)
    }
}

#[derive(Debug, Clone)]
pub struct PolicyViolation {
    source: Span,
    message: String,
}

impl PolicyViolation {
    pub const fn new(source: Span, message: String) -> Self {
        Self { source, message }
    }

    #[inline]
    pub const fn source(&self) -> &Span {
        &self.source
    }

    #[inline]
    pub fn message(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, Clone)]
pub struct PolicyCheckOutput<'a> {
    policy: &'a Policy,
    violations: Vec<PolicyViolation>,
}

impl<'a> PolicyCheckOutput<'a> {
    pub const fn new(
        policy: &'a Policy,
        violations: Vec<PolicyViolation>,
    ) -> PolicyCheckOutput<'a> {
        Self { policy, violations }
    }

    #[inline]
    pub const fn policy(&self) -> &Policy {
        self.policy
    }

    #[inline]
    pub fn violations(&self) -> &[PolicyViolation] {
        &self.violations
    }

    #[inline]
    pub fn into_violations(self) -> Vec<PolicyViolation> {
        self.violations
    }
}

inventory::collect!(Policy);

pub fn get_policies() -> impl Iterator<Item = &'static Policy> {
    inventory::iter::<Policy>.into_iter()
}
