# esindex_exporter

This is a very simple Prometheus exporter, used for gathering metrics about groups of Elasticsearch (or OpenSearch) indices.

The following statistics are gathered:

| Metric                            | Description                                                                               |
|-----------------------------------|-------------------------------------------------------------------------------------------|
| `esindex_grouped_indexes_total`   | Number of indexes in group                                                                |
| `esindex_store_bytes`             | Total stored size of group in bytes                                                       |
| `esindex_pri_store_bytes`         | Total primary stored size of group in bytes                                               |
| `esindex_sec_store_bytes`         | Total secondary (all replicas) stored size of group in bytes                              |
| `esindex_docs_count_total`        | Total number of documents in group                                                        |
| `esindex_docs_deleted_total`      | Total number of deleted documents in group                                                |
| `esindex_ungrouped_indexes_total` | Number of indexes that could not be made part of the group it was requested to be part of |

Each metric is labeled with the group name, as the `group` label.

## Building

To build the project, you need to have Rust installed.
You can install Rust by following the instructions at <https://www.rust-lang.org/tools/install>.

Once you have Rust installed, you can build the project by running the following command:

```sh
cargo build --release
```

The built binary will be available at `target/release/esindex_exporter`.

## Example

Given you got a series of indices for your application loadbalancers' access logs,
which are prefixed with the environment name and suffixed with some form of date (`integration-alb.access-2024.11.19`).
You might now be interested in how much space these kinds of indices take up, or how many documents are in them.

Create a config with a single group:

```yaml
base_url: "https://our.internal.es.url"

groups:
- name: integration-alb.access
  index_patterns:
  - integration-alb.access-*
```

When starting the app with the above configuration saved as `esindex_exporter.yaml` in the same directory:

```shell
$ esindex_exporter
[2024-11-18T16:55:33Z INFO  esindex_exporter] Updating metrics as per refresh interval (every 60 seconds)
[2024-11-18T16:55:33Z INFO  prometheus_exporter] exporting metrics to http://127.0.0.1:19100/metrics
[2024-11-18T16:56:33Z INFO  esindex_exporter] Updating metrics as per refresh interval (every 60 seconds)
...
```

Following is the metric-output you'll see if you navigate to <http://127.0.0.1:19100/metrics>:

```plaintext
# HELP esindex_docs_count_total Total number of documents in group
# TYPE esindex_docs_count_total gauge
esindex_docs_count_total{group="integration-alb.access"} 17402137
# HELP esindex_docs_deleted_total Total number of deleted documents in group
# TYPE esindex_docs_deleted_total gauge
esindex_docs_deleted_total{group="integration-alb.access"} 0
# HELP esindex_grouped_indexes_total Number of indexes in group
# TYPE esindex_grouped_indexes_total gauge
esindex_grouped_indexes_total{group="integration-alb.access"} 2
# HELP esindex_pri_store_bytes Total primary stored size of group in bytes
# TYPE esindex_pri_store_bytes gauge
esindex_pri_store_bytes{group="integration-alb.access"} 8434832247
# HELP esindex_sec_store_bytes Total secondary (all replicas) stored size of group in bytes
# TYPE esindex_sec_store_bytes gauge
esindex_sec_store_bytes{group="integration-alb.access"} 2874463118
# HELP esindex_store_bytes Total stored size of group in bytes
# TYPE esindex_store_bytes gauge
esindex_store_bytes{group="integration-alb.access"} 11309295365
# HELP prometheus_exporter_request_duration_seconds The HTTP request latencies in seconds.
# TYPE prometheus_exporter_request_duration_seconds histogram
prometheus_exporter_request_duration_seconds_bucket{le="0.005"} 1
prometheus_exporter_request_duration_seconds_bucket{le="0.01"} 1
prometheus_exporter_request_duration_seconds_bucket{le="0.025"} 1
prometheus_exporter_request_duration_seconds_bucket{le="0.05"} 1
prometheus_exporter_request_duration_seconds_bucket{le="0.1"} 1
prometheus_exporter_request_duration_seconds_bucket{le="0.25"} 1
prometheus_exporter_request_duration_seconds_bucket{le="0.5"} 1
prometheus_exporter_request_duration_seconds_bucket{le="1"} 1
prometheus_exporter_request_duration_seconds_bucket{le="2.5"} 1
prometheus_exporter_request_duration_seconds_bucket{le="5"} 1
prometheus_exporter_request_duration_seconds_bucket{le="10"} 1
prometheus_exporter_request_duration_seconds_bucket{le="+Inf"} 1
prometheus_exporter_request_duration_seconds_sum 0.000002504
prometheus_exporter_request_duration_seconds_count 1
# HELP prometheus_exporter_requests_total Number of HTTP requests received.
# TYPE prometheus_exporter_requests_total counter
prometheus_exporter_requests_total 1
```

All metrics prefixed `esindex_` are the metrics gathered for the index-groups you created.

Additionally, the exporter itself also keeps track of some internal metrics (i.e. how well does the exporter perform), which are prefixed with `prometheus_exporter_`.

If you want to test your configuration without having to navigate to the metric-endpoint, you can provide the `--print-once-as-json` flag on startup.
This changes the exporter's behavior to not run a server, but instead print the metrics of each group as a JSON-object to stdout:

```shell
$ esindex_exporter --print-once-as-json
{"group_name":"integration-alb.access","grouped_indexes_total":2,"store_bytes":11309295365,"pri_store_bytes":8434832247,"sec_store_bytes":2874463118,"docs_count_total":17402137,"docs_deleted_total":0}
...
[2025-01-08T15:32:09Z INFO  esindex_exporter] Printed metrics once as JSON, exiting
```

You can then also use this for further processing, e.g. getting the top 5 groups by stored bytes, formatted in columns:

```shell
$ esindex_exporter --print-once-as-json | jq -s 'sort_by(.store_bytes) | reverse | .[:5] | [.[]| with_entries(.key |= ascii_downcase) ] | (.[0] | keys_unsorted | @tsv), (.[]  | map(.) | @tsv)' | column -t
group_name                       grouped_indexes_total  store_bytes  pri_store_bytes  sec_store_bytes  docs_count_total  docs_deleted_total
production-alb.access            43                     1230722      1209647          21075            2077388733        0
preprod-alb.access               43                     226973       223434           3538             560098066         0
production-some.application      44                     156548       153886           2661             532177108         0
services-docker.container        1                      71073        35544            35529            80021704          0
integration-alb.access           43                     54157        53097            1059             71242321          68
```

## License

esindex_exporter is licensed under the Apache License, Version 2.0, (see [LICENSE](LICENSE) or <https://www.apache.org/licenses/LICENSE-2.0>).

esindex_exporter internally makes use of various open-source projects.
You can find a full list of these projects and their licenses in [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md).

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in esindex_exporter by you, as defined in the Apache-2.0 license, shall be licensed under the Apache License, Version 2.0, without any additional terms or conditions.

We require code submitted to be formatted with Rust's default rustfmt formatter (CI will automatically verified if your code is formatted correctly).
We are using unstable rustfmt formatting rules, which requires running the formatter with a nightly toolchain, which you can do as follows:

```sh
$ rustup toolchain install nightly
$ cargo +nightly fmt
```

(Building and running esindex_exporter itself can and should happen with the stable toolchain.)

Additionally, we are also checking whether there are any clippy warnings in your code.
You can run clippy locally with:

```sh
$ cargo clippy --workspace --lib --bins --tests --all-targets -- -Dwarnings
```

There can be occasions where newer versions of clippy warn about code you haven't touched.
In such cases we'll try to get those warnings resolved before merging your changes, or work together with you to get them resolved in your merge request.
