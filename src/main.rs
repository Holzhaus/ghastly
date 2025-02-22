// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
#[command(propagate_version = true)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Check workflow file.
    Check {
        /// Workflow file to check.
        #[arg(value_name = "FILE")]
        path: PathBuf,
    },
}

fn main() -> ghastly::Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Check { path } => ghastly::check_workflow(path),
    }
}
