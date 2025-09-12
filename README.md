<p align="center">
    <img src="https://raw.githubusercontent.com/foyer-rs/mixtrics/main/etc/mixtrics.png" />
</p>

# mixtrics

***mixtrics*** is a lightweight unified metrics library in Rust for various metrics system.

## Motivation

Currently, many libraries in Rust's metrics ecosystem are in a state of rapid development, with frequent breaking changes. As a developer of library that need the metrics feature, a lot of effort is required to maintain dependencies on different versions of components. If only maintaining the latest version of dependencies for metrics, it will couple the functionality of the library with the metrics version, breaking compatibility.

***mixtrics*** supports various versions of multiple metrics libraries through features to alleviate the burden mentioned above.

## Features

- Various metrics system support. Even various versions of various metrics system support.
- Lightweight library with almost no additional dependencies.
- Implementation with minimal overhead.

## Supported Metrics Backends

- Prometheus
    - [`prometheus`](https://crates.io/crates/prometheus): `0.13`, `0.14`
    - [`prometheus-client`](https://crates.io/crates/prometheus-client): `0.22`, `0.23`, `0.24`
- OpenTemeletry Metrics
    - [`opentelemetry`](https://crates.io/crates/opentelemetry): `0.26`, `0.27`, `0.28`, `0.29`, `0.30`

***mixtrics*** will continuously track the libraries and their version of different metrics system. If there is a library or a specific version is not support yet, please feel free to open an issue or a PR. 🙌

## Example

***mixtrics*** is convenient for both lib developers and users.

### As a lib developer

Assuming you have many clients that rely on different metrics systems, you want to implement a solution that provides each client with the compatibility they need for their specific metrics system.

With ***mixtrics***, you can easily solve the problem.

1. Add ***mixtrics*** to your lib dependencies, no need to enable any features.

```toml
mixtrics = "0.1"
```

2. Define the metrics.

```rust
struct DiskMetrics {
    write_ops: BoxedCounter,
    write_duration: BoxedHistogram,
    read_ops: BoxedCounter,
    read_duration: BoxedHistogram,
    sync_ops: BoxedCounter,
    sync_duration: BoxedHistogram,
}
```

3. Add a `new()` function to setup metrics.

```rust
impl DiskMetrics {
    fn new(app: impl Into<Cow<'static, str>>, registry: &BoxedRegistry) -> Self {
        let app = app.into();

        // 1. Register metric vectors.

        let op_total = registry.register_counter_vec("disk_op_total".into(), "disk ops".into(), &["app", "op"]);
        let op_duration =
            registry.register_histogram_vec("disk_op_duration".into(), "disk op duration".into(), &["app", "op"]);
        // You can also specify buckets for the histogram.
        let slow_op_duration = registry.register_histogram_vec_with_buckets(
            "disk_slow_op_duration".into(),
            "disk slow op duration".into(),
            &["app", "op"],
            vec![0.001, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0],
        );

        // 2. Create metrics from metrics vectors.

        let write_ops = op_total.counter(&[app.clone(), "write".into()]);
        let write_duration = op_duration.histogram(&[app.clone(), "write".into()]);
        let read_ops = op_total.counter(&[app.clone(), "read".into()]);
        let read_duration = op_duration.histogram(&[app.clone(), "read".into()]);
        let sync_ops = op_total.counter(&[app.clone(), "sync".into()]);
        let sync_duration = slow_op_duration.histogram(&[app.clone(), "sync".into()]);

        Self {
            write_ops,
            write_duration,
            read_ops,
            read_duration,
            sync_ops,
            sync_duration,
        }
    }

    // You can create a noop metrics for testing.
    #[cfg(test)]
    fn noop() -> Self {
        let registry: BoxedRegistry = Box::new(mixtrics::registry::noop::NoopMetricsRegistry);
        Self::new("test", &registry)
    }
}
```

4. Record metrics in your app.

```rust
pub struct App {
    file: File,
    metrics: Arc<DiskMetrics>,
}

impl App {
    // Require user to pass the metrics registry.
    pub fn new(registry: &BoxedRegistry) -> Self {
        let metrics = Arc::new(DiskMetrics::new("my_app", &registry));
        let file = tempfile::tempfile().unwrap();

        Self { file, metrics }
    }

    pub fn write(&mut self, data: &[u8]) {
        let start = Instant::now();
        self.file.write_all(data).unwrap();
        let elapsed = start.elapsed();
        self.metrics.write_ops.increase(1);
        self.metrics.write_duration.record(elapsed.as_secs_f64());
    }
}
```

ALL DONE! No need to consider the implementation of different metrics systems.

### As a user

Assuming you are a user using a lib that uses ***mixtrics***.

1. Add ***mixtrics*** to your lib dependencies with your metrics system feature enabled.

```toml
mixtrics = { version = "0.1", features = ["prometheus_0_14"] }
```

2. Setup the app with the registry of your metrics system.

```rust
fn main() {
    let registry = Box::new(PrometheusMetricsRegistry::new(prometheus_0_14::Registry::new())) as _;
    let mut app = App::new(&registry);
    app.write(b"Hello, world!");
}
```

ALL DONE!

> For crate `prometheus`, you need to export the metrics by yourself. Please refer to the `prometheus` [docs.rs](https://docs.rs/prometheus/latest/prometheus/)

### Advanced

As a lib developer, if you don't want your users to add ***mixtrics*** and set the registry on the user's side. You can warp it in your lib with `#[cfg(feature = "xxx")]` to control which metrics system to use and compile.

## Star History

[![Star History Chart](https://api.star-history.com/svg?repos=foyer-rs/mixtrics&type=Date)](https://www.star-history.com/#foyer-rs/mixtrics&Date)