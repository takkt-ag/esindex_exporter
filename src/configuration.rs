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
use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
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
    pub(crate) fn load_yaml(reader: impl std::io::Read) -> Result<Self> {
        serde_yaml::from_reader(reader).context("Failed to parse the configuration file")
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
    ///
    /// ## Regex grouping
    ///
    /// If you also provide a grouping-regex, you can use backreferences to the capturing groups in the regex to create
    /// the group name.
    ///
    /// For example, if you have an indexes like `integration-alb.access-YYYY.MM` and `preprod-alb.access-YYYY.MM` and
    /// you want to create groups dynamically across the environment, you could provide a configuration like this:
    ///
    /// ```yaml
    /// groups:
    /// - name: '${environment}-alb.access'
    ///   index_patterns:
    ///   - '*-alb.access-*'
    ///   grouping_regex: '^(?<environment>[^-]+)-alb\.access'
    /// ```
    ///
    /// If you provide an index pattern where indexes don't match the grouping-regex, the indices that don't match will
    /// be ignored and not show up in the metrics. The `esindex_ungrouped_indexes_total` metric will show the count of
    /// such indexes, with the `group` label set to the name of the group
    pub(crate) name: String,
    /// The index patterns to monitor as part of this group.
    ///
    /// The patterns are combined with a comma and used as the `index` query parameter in the
    /// Elasticsearch `/_cat/indices` API. See the Elasticsearch documentation on
    /// ["Multi-target syntax"][mts] for what all you can provide.
    ///
    /// [mts]: https://www.elastic.co/guide/en/elasticsearch/reference/current/api-conventions.html#api-multi-index
    pub(crate) index_patterns: Vec<String>,
    /// An optional regex whose capturing groups can be used to create the group name dynamically.
    #[serde(
        default,
        deserialize_with = "crate::de::deserialize_optional_string_as_regex"
    )]
    pub(crate) grouping_regex: Option<regex::Regex>,
}
