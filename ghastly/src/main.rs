// Copyright (c) 2025 Jan Holthuis <jan.holthuis@rub.de>
//
// This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy
// of the MPL was not distributed with this file, You can obtain one at
// http://mozilla.org/MPL/2.0/.
//
// SPDX-License-Identifier: MPL-2.0

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use termimad::{Alignment, MadSkin};

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
    /// List policies.
    List,
    /// Show information about a policy.
    Show {
        /// Policy Name
        name: String,
    },
}

fn main() -> ghastly::Result<()> {
    let args = Args::parse();

    match &args.command {
        Commands::Check { path } => {
            let output = ghastly::check_workflow(path)?;
            let mut policy_violations: Vec<_> = output
                .iter()
                .flat_map(|policy_output| {
                    policy_output
                        .violations()
                        .iter()
                        .map(|violation| (policy_output.policy(), violation))
                })
                .collect();
            policy_violations.sort_by_key(|(_policy, violation)| {
                violation
                    .source()
                    .start()
                    .map(|marker| (marker.line(), marker.column()))
                    .unwrap_or_default()
            });
            policy_violations
                .into_iter()
                .for_each(|(policy, violation)| {
                    let line = violation
                        .source()
                        .start()
                        .map(|marker| marker.line())
                        .unwrap_or_default();
                    let column = violation
                        .source()
                        .start()
                        .map(|marker| marker.column())
                        .unwrap_or_default();
                    println!(
                        "{path}:{line}:{column}:{message} ({policy_name})",
                        path = path.display(),
                        message = violation.message(),
                        policy_name = policy.name
                    );
                });
            Ok(())
        }
        Commands::List => {
            ghastly::get_policies().for_each(|policy| {
                println!("{}", policy.name);
            });
            Ok(())
        }
        Commands::Show { name } => {
            if let Some(policy) = ghastly::get_policies().find(|policy| policy.name == name) {
                if let Some(doc) = policy.doc {
                    let mut skin = MadSkin::default();
                    skin.headers.iter_mut().for_each(|line_style| {
                        line_style.align = Alignment::Left;
                    });

                    skin.print_text(doc);
                } else {
                    eprintln!("Policy {} has not documentation", policy.name);
                }
            } else {
                eprintln!("Policy {} not found", name);
            };
            Ok(())
        }
    }
}
