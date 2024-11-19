# esindex_exporter

This is a very simple Prometheus exporter, used for gathering metrics about groups of Elasticsearch (or OpenSearch) indices.

The following statistics are gathered:

| Metric                          | Description                                                  |
|---------------------------------|--------------------------------------------------------------|
| `esindex_grouped_indexes_total` | Number of indexes in group                                   |
| `esindex_store_bytes`           | Total stored size of group in bytes                          |
| `esindex_pri_store_bytes`       | Total primary stored size of group in bytes                  |
| `esindex_sec_store_bytes`       | Total secondary (all replicas) stored size of group in bytes |
| `esindex_docs_count_total`      | Total number of documents in group                           |
| `esindex_docs_deleted_total`    | Total number of deleted documents in group                   |

Each metric is labeled with the group name, as the `group` label.

## Example

Given you got a series of indices for your application loadbalancers' access logs,
which are prefixed with the environment name and suffixed with some form of date.

Create a config with a single group:

```yaml
base_url: "https://our.internal.es.url"

groups:
- name: integration-alb.access
  index_patterns:
  - integration-alb.access-*
```

When starting the app with the above configuration saved as `esindex_exporter.yaml` in the CWD:

```shell
$ cargo run
   Compiling esindex_exporter v0.1.0 (.../esindex_exporter)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.67s
     Running `target/debug/esindex_exporter`
[2024-11-18T16:55:33Z INFO  esindex_exporter] Updating metrics as per refresh interval (every 60 seconds)
[2024-11-18T16:55:33Z INFO  prometheus_exporter] exporting metrics to http://127.0.0.1:19100/metrics
[2024-11-18T16:56:33Z INFO  esindex_exporter] Updating metrics as per refresh interval (every 60 seconds)
...
```

This is the metric-output:

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

The `prometheus_exporter_` prefixed metrics are internal to the exporter, which the library we use provides automatically. The `esindex_` metrics are "ours", as documented in the README.

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
