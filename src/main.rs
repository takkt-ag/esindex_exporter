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

use anyhow::Result;
use clap::Parser;
use log::{
    info,
    warn,
};
use prometheus_exporter::prometheus::register_gauge_vec;

mod cli;
mod configuration;
mod de;
mod elasticsearch;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let cli = cli::Cli::parse();
    let configuration = configuration::Configuration::load_from_yaml_file(cli.config_file)?;
    if cli.only_lint_config_file {
        info!("Configuration file is valid");
        return Ok(());
    }

    let grouped_indexes_total = register_gauge_vec!(
        "esindex_grouped_indexes_total",
        "Number of indexes in group",
        &["group"],
    )?;
    let store_bytes = register_gauge_vec!(
        "esindex_store_bytes",
        "Total stored size of group in bytes",
        &["group"],
    )?;
    let pri_store_bytes = register_gauge_vec!(
        "esindex_pri_store_bytes",
        "Total primary stored size of group in bytes",
        &["group"],
    )?;
    let sec_store_bytes = register_gauge_vec!(
        "esindex_sec_store_bytes",
        "Total secondary (all replicas) stored size of group in bytes",
        &["group"],
    )?;
    let docs_count_total = register_gauge_vec!(
        "esindex_docs_count_total",
        "Total number of documents in group",
        &["group"],
    )?;
    let docs_deleted_total = register_gauge_vec!(
        "esindex_docs_deleted_total",
        "Total number of deleted documents in group",
        &["group"],
    )?;

    let ungrouped_indexes_total = register_gauge_vec!(
        "esindex_ungrouped_indexes_total",
        "Number of indexes that could not be made part of the group it was requested to be part of",
        &["group"],
    )?;

    let exporter = prometheus_exporter::start(configuration.bind_addr)?;
    let refresh_interval =
        std::time::Duration::from_secs(configuration.refresh_interval_in_seconds);

    let mut first_run = true;
    loop {
        let _guard = if first_run {
            None
        } else {
            Some(exporter.wait_duration(refresh_interval))
        };
        info!(
            "Updating metrics as per refresh interval (every {} seconds)",
            configuration.refresh_interval_in_seconds
        );

        for group in &configuration.groups {
            let (index_groups, ungrouped) =
                elasticsearch::cat_indices(&configuration.base_url, &group.index_patterns)?
                    .group(&group.name, group.grouping_regex.as_ref());

            ungrouped_indexes_total
                .with_label_values(&[&group.name])
                .set(ungrouped.len() as f64);
            if !ungrouped.is_empty() {
                warn!(
                    "The following indexes could not be grouped into '{}': {}",
                    group.name,
                    ungrouped
                        .iter()
                        .map(|i| &*i.index)
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }

            for (group_name, cat_indices) in index_groups {
                grouped_indexes_total
                    .with_label_values(&[&group_name])
                    .set(cat_indices.cat_index_results.len() as f64);
                store_bytes
                    .with_label_values(&[&group_name])
                    .set(cat_indices.store_size_sum as f64);
                pri_store_bytes
                    .with_label_values(&[&group_name])
                    .set(cat_indices.pri_store_size_sum as f64);
                sec_store_bytes
                    .with_label_values(&[&group_name])
                    .set(cat_indices.sec_store_size_sum as f64);
                docs_count_total
                    .with_label_values(&[&group_name])
                    .set(cat_indices.docs_count_sum as f64);
                docs_deleted_total
                    .with_label_values(&[&group_name])
                    .set(cat_indices.docs_deleted_sum as f64);
            }
        }

        first_run = false;
    }
}
