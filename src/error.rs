// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use thiserror::Error;

/// Enumerates errors returned by this library.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GhastlyError {
    /// Represents an IO error.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    /// Represents a YAML error.
    #[error(transparent)]
    YamlError(#[from] marked_yaml::FromYamlError),
}

pub type GhastlyResult<T> = Result<T, GhastlyError>;
