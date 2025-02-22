// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use std::fs::File;
use std::path::Path;

mod error;
mod parse;
mod policies;

pub use error::GhastlyError as Error;
pub use error::GhastlyResult as Result;
pub use policies::{get_policies, Policy, PolicyCheckOutput};

pub fn check_workflow(path: impl AsRef<Path>) -> Result<Vec<PolicyCheckOutput<'static>>> {
    let mut file = File::open(path)?;
    let workflow = parse::parse_workflow(&mut file)?;
    Ok(get_policies()
        .map(|policy| policy.check(&workflow))
        .collect())
}
