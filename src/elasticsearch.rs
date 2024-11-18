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
use serde::Deserialize;

pub(crate) struct CatIndices {
    pub(crate) cat_index_results: Vec<CatIndexResult>,
    pub(crate) docs_count_sum: u64,
    pub(crate) docs_deleted_sum: u64,
    pub(crate) store_size_sum: u64,
    pub(crate) pri_store_size_sum: u64,
    pub(crate) sec_store_size_sum: u64,
}

impl CatIndices {
    fn new(cat_index_results: Vec<CatIndexResult>) -> Self {
        let mut docs_count_sum = 0;
        let mut docs_deleted_sum = 0;
        let mut store_size_sum = 0;
        let mut pri_store_size_sum = 0;
        let mut sec_store_size_sum = 0;

        for cat_index_result in &cat_index_results {
            docs_count_sum += cat_index_result.docs_count;
            docs_deleted_sum += cat_index_result.docs_deleted;
            store_size_sum += cat_index_result.store_size;
            pri_store_size_sum += cat_index_result.pri_store_size;
            sec_store_size_sum += cat_index_result.store_size - cat_index_result.pri_store_size;
        }

        Self {
            cat_index_results,
            docs_count_sum,
            docs_deleted_sum,
            store_size_sum,
            pri_store_size_sum,
            sec_store_size_sum,
        }
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct CatIndexResult {
    #[serde(
        rename = "docs.count",
        deserialize_with = "crate::de::deserialize_string_as_number"
    )]
    pub(crate) docs_count: u64,
    #[serde(
        rename = "docs.deleted",
        deserialize_with = "crate::de::deserialize_string_as_number"
    )]
    pub(crate) docs_deleted: u64,
    #[serde(
        rename = "store.size",
        deserialize_with = "crate::de::deserialize_string_as_number"
    )]
    pub(crate) store_size: u64,
    #[serde(
        rename = "pri.store.size",
        deserialize_with = "crate::de::deserialize_string_as_number"
    )]
    pub(crate) pri_store_size: u64,
}

pub(crate) fn cat_indices<S: AsRef<str>>(
    base_url: S,
    index_patterns: &[String],
) -> Result<CatIndices> {
    let mut url = reqwest::Url::parse(base_url.as_ref())?;
    url = url.join("_cat/indices/")?;
    url = url.join(&index_patterns.join(","))?;
    let client = reqwest::blocking::Client::new();
    Ok(CatIndices::new(
        client
            .get(url)
            .query(&[("format", "json"), ("bytes", "b")])
            .send()?
            .json()?,
    ))
}
