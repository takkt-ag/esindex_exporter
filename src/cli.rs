// Copyright 2024 TAKKT Industrial & Packaging GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use std::path::PathBuf;

/// Export metrics for groups of Elasticsearch indexes.
#[derive(Debug, Parser)]
#[command(version, about, max_term_width = 100)]
pub(crate) struct Cli {
    /// The YAML configuration file which defines where to find Elasticsearch/OpenSearch, and which
    /// groups of index-patterns to export metrics for.
    #[arg(
        long,
        default_value = "esindex_exporter.yaml",
        env = "ESINDEX_EXPORTER_CONFIG_FILE"
    )]
    pub(crate) config_file: PathBuf,
    /// Only lint the provided configuration file and exit.
    ///
    /// If the configuration is valid, the exit-code will be 0. If the configuration is invalid, the exit-code will be
    /// non-zero.
    #[arg(long, default_value = "false")]
    pub(crate) only_lint_config_file: bool,
    /// Extract the results according to the provided configuration once and print them as JSON, then exit.
    ///
    /// This flag is particularly helpful if you want to test your configuration.
    #[arg(long, default_value = "false")]
    pub(crate) print_once_as_json: bool,
}
