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

use base64::Engine;
use clap::Parser;
use std::path::PathBuf;

/// Export metrics for groups of Elasticsearch indexes.
#[derive(Debug, Parser)]
#[command(version, about, max_term_width = 100)]
pub(crate) struct Cli {
    /// The YAML configuration file which defines where to find Elasticsearch/OpenSearch, and which
    /// groups of index-patterns to export metrics for.
    ///
    /// The value provided can either be a path to the YAML-file containing the configuration, or the YAML-configuration
    /// itself as base64-encoded content. (The latter, providing the content directly, can be particularly useful if you
    /// are running a pre-built Docker image of the exporter and have no easy way of including a configuration file,
    /// such as on AWS ECS.)
    #[arg(
        long,
        value_parser = content_or_path,
        default_value = "esindex_exporter.yaml",
        env = "ESINDEX_EXPORTER_CONFIG_FILE"
    )]
    pub(crate) config_file: ContentOrPath,
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

#[derive(Debug, Clone)]
pub(crate) enum ContentOrPath {
    Content(Vec<u8>),
    Path(PathBuf),
}

impl ContentOrPath {
    pub(crate) fn reader(&self) -> Result<Box<dyn std::io::Read>, std::io::Error> {
        Ok(match self {
            ContentOrPath::Content(content) => Box::new(std::io::Cursor::new(content.clone())),
            ContentOrPath::Path(path) => Box::new(std::fs::File::open(path)?),
        })
    }
}

fn content_or_path(argument: &str) -> Result<ContentOrPath, String> {
    // If the argument provided is a path that exists and is a file, we don't have to check anything else and can return
    // that right away.
    if std::path::Path::new(argument).is_file() {
        return Ok(ContentOrPath::Path(argument.into()));
    }

    // If the exists-check failed, there are two possibilities: the parameter provided was a path, but one that doesn't
    // exist, or it is base64-encoded content. We'll now try to decode the argument as base64. If that succeeded, we can
    // also return that right away.
    let decode_result = base64::engine::general_purpose::STANDARD.decode(argument.as_bytes());
    if let Ok(decoded) = decode_result {
        return Ok(ContentOrPath::Content(decoded));
    }

    // Since both the path doesn't exist and the base64-decoding failed, we'll return an error that provides details on
    // both.
    Err(format!(
        "path does not exist or is not a file, or content provided is not valid base64 (base64-decode error: {})",
        decode_result.unwrap_err(),
    ))
}
