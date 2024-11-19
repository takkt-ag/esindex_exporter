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

use anyhow::{
    Context,
    Result,
};
use serde::Deserialize;
use std::{
    net::{
        IpAddr,
        Ipv4Addr,
        SocketAddr,
    },
    path::Path,
};

/// The configuration of the exporter.
#[derive(Debug, Deserialize)]
pub(crate) struct Configuration {
    /// The address to bind the exporter to.
    #[serde(default = "Configuration::default_bind_addr")]
    pub(crate) bind_addr: SocketAddr,
    /// The interval, in seconds, in which the metrics for the groups of indexes should be
    /// refreshed.
    #[serde(default = "Configuration::default_refresh_interval_in_seconds")]
    pub(crate) refresh_interval_in_seconds: u64,
    /// The base URL of the Elasticsearch (or OpenSearch) cluster.
    pub(crate) base_url: String,
    /// The groups of indexes to monitor.
    pub(crate) groups: Vec<Group>,
}

impl Configuration {
    /// Load the configuration from a YAML-file.
    pub(crate) fn load_from_yaml_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        serde_yaml::from_reader(std::fs::File::open(path)?)
            .context("Failed to parse the configuration file")
    }

    fn default_bind_addr() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 19100)
    }

    fn default_refresh_interval_in_seconds() -> u64 {
        60
    }
}

/// A group of indexes to monitor.
#[derive(Debug, Deserialize)]
pub(crate) struct Group {
    /// The name of the group which will be present as the `group` label in the metrics.
    pub(crate) name: String,
    /// The index patterns to monitor as part of this group.
    ///
    /// The patterns are combined with a comma and used as the `index` query parameter in the
    /// Elasticsearch `/_cat/indices` API. See the Elasticsearch documentation on
    /// ["Multi-target syntax"][mts] for what all you can provide.
    ///
    /// [mts]: https://www.elastic.co/guide/en/elasticsearch/reference/current/api-conventions.html#api-multi-index
    pub(crate) index_patterns: Vec<String>,
}
