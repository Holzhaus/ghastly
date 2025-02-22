// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use super::PolicyViolation;
use crate::parse::Workflow;
use ghastly_macros::policy;

#[policy]
pub fn foobar(_workflow: &Workflow) -> Vec<PolicyViolation> {
    vec![]
}
