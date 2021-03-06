// Copyright (c) Aptos
// SPDX-License-Identifier: Apache-2.0

use crate::common::{types::CliResult, utils::to_common_result};
use clap::Subcommand;

pub mod create;
pub mod list;

/// CLI tool for interacting with accounts
///
#[derive(Debug, Subcommand)]
pub enum AccountTool {
    Create(create::CreateAccount),
    List(list::ListResources),
}

impl AccountTool {
    pub async fn execute(self) -> CliResult {
        match self {
            AccountTool::Create(tool) => to_common_result(tool.execute().await),
            AccountTool::List(tool) => to_common_result(tool.execute().await),
        }
    }
}
