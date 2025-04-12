// Copyright 2025 mixtrics Project Authors
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

use std::{borrow::Cow, fs::File, io::Write, sync::Arc, time::Instant};

use mixtrics::{
    metrics::{BoxedCounter, BoxedHistogram, BoxedRegistry},
    registry::prometheus_0_14::PrometheusMetricsRegistry,
};

#[allow(dead_code)]
struct DiskMetrics {
    write_ops: BoxedCounter,
    write_duration: BoxedHistogram,
    read_ops: BoxedCounter,
    read_duration: BoxedHistogram,
    sync_ops: BoxedCounter,
    sync_duration: BoxedHistogram,
}

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

pub struct App {
    file: File,
    metrics: Arc<DiskMetrics>,
}

impl App {
    // Require user to pass the metrics registry.
    pub fn new(registry: &BoxedRegistry) -> Self {
        let metrics = Arc::new(DiskMetrics::new("my_app", registry));
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

fn main() {
    let registry = Box::new(PrometheusMetricsRegistry::new(prometheus_0_14::Registry::new())) as _;
    let mut app = App::new(&registry);
    app.write(b"Hello, world!");
}
