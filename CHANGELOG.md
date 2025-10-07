---
title: Changelog
description: Changelog for mixtrics.
authors: mrcroxx
date: 2025-04-12T12:00:00+08:00
---

# Changelog

<!-- truncate -->

## v0.2.2

- Support  [`opentelemetry`](https://crates.io/crates/opentelemetry): `0.31`.

## v0.2.1

- Support [`prometheus-client`](https://crates.io/crates/prometheus-client) `0.24`.
- Disable unnecessary features for metrics backends. The features can be enabled on the user side if needed.

## v0.2.0

Support [`opentelemetry`](https://crates.io/crates/opentelemetry) `0.30`.

## v0.1.0

Supports the following metrics backends:

- Prometheus
    - [`prometheus`](https://crates.io/crates/prometheus): `0.13`, `0.14`
    - [`prometheus-client`](https://crates.io/crates/prometheus-client): `0.22`, `0.23`
- OpenTemeletry Metrics
    - [`opentelemetry`](https://crates.io/crates/opentelemetry): `0.26`, `0.27`, `0.28`, `0.29`