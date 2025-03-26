<p align="center">
    <img src="https://raw.githubusercontent.com/foyer-rs/mixtrics/main/etc/mixtrics.png" />
</p>

# mixtrics

***mixtrics*** is a rust library that provides a unified abstraction of various metrics systems.

## Motivation

Currently, many libraries in Rust's metrics ecosystem are in a state of rapid development, with frequent breaking changes. As a developer of library that need the metrics feature, a lot of effort is required to maintain dependencies on different versions of components. If only maintaining the latest version of dependencies for metrics, it will couple the functionality of the library with the metrics version, breaking compatibility.

***mixtrics*** supports various versions of multiple metrics libraries through features to alleviate the burden mentioned above.

## Supported Metrics Backends

- Prometheus
    - [`prometheus`](https://crates.io/crates/prometheus): `0.13`
    - [`prometheus-client`](https://crates.io/crates/prometheus-client): `0.22`, `0.23`
- OpenTemeletry Metrics
    - [`opentelemetry`](https://crates.io/crates/opentelemetry): `0.26`, `0.27`, `0.28`, `0.29`

## Roadmap

- [ ] Documents and examples.
- [ ] Other metrics backend supports.
    - [ ] ???
- [ ] Changelog (will start since v0.1.x)