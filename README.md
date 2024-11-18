# esindex_exporter

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
