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

pub use opentelemetry_0_26;
pub use opentelemetry_0_27;
pub use opentelemetry_0_28;
pub use opentelemetry_0_29;
pub use prometheus_0_13;
pub use prometheus_0_14;
pub use prometheus_client_0_22;
pub use prometheus_client_0_23;

#[macro_export]
macro_rules! test {
    ($f:expr) => {
        #[test]
        fn test_prometheus_0_13() {
            use $crate::{
                metrics::BoxedRegistry, registry::prometheus_0_13::PrometheusMetricsRegistry,
                test_utils::prometheus_0_13::Registry,
            };
            let registry: BoxedRegistry = Box::new(PrometheusMetricsRegistry::new(Registry::new()));
            ($f)(&registry);
        }

        #[test]
        fn test_prometheus_0_14() {
            use $crate::{
                metrics::BoxedRegistry, registry::prometheus_0_14::PrometheusMetricsRegistry,
                test_utils::prometheus_0_14::Registry,
            };
            let registry: BoxedRegistry = Box::new(PrometheusMetricsRegistry::new(Registry::new()));
            ($f)(&registry);
        }

        #[test]
        fn test_prometheus_client_0_22() {
            use std::sync::Arc;

            use parking_lot::Mutex;
            use $crate::{
                metrics::BoxedRegistry, registry::prometheus_client_0_22::PrometheusClientMetricsRegistry,
                test_utils::prometheus_client_0_22::registry::Registry,
            };
            let registry: BoxedRegistry = Box::new(PrometheusClientMetricsRegistry::new(Arc::new(Mutex::new(
                Registry::default(),
            ))));
            ($f)(&registry);
        }

        #[test]
        fn test_prometheus_client_0_23() {
            use std::sync::Arc;

            use parking_lot::Mutex;
            use $crate::{
                metrics::BoxedRegistry, registry::prometheus_client_0_23::PrometheusClientMetricsRegistry,
                test_utils::prometheus_client_0_23::registry::Registry,
            };
            let registry: BoxedRegistry = Box::new(PrometheusClientMetricsRegistry::new(Arc::new(Mutex::new(
                Registry::default(),
            ))));
            ($f)(&registry);
        }

        #[test]
        fn test_opentelemetry_0_26() {
            use $crate::{
                metrics::BoxedRegistry, registry::opentelemetry_0_26::OpenTelemetryMetricsRegistry,
                test_utils::opentelemetry_0_26::global::meter,
            };
            let registry: BoxedRegistry = Box::new(OpenTelemetryMetricsRegistry::new(meter("test")));
            ($f)(&registry);
        }

        #[test]
        fn test_opentelemetry_0_27() {
            use $crate::{
                metrics::BoxedRegistry, registry::opentelemetry_0_27::OpenTelemetryMetricsRegistry,
                test_utils::opentelemetry_0_27::global::meter,
            };
            let registry: BoxedRegistry = Box::new(OpenTelemetryMetricsRegistry::new(meter("test")));
            ($f)(&registry);
        }

        #[test]
        fn test_opentelemetry_0_28() {
            use $crate::{
                metrics::BoxedRegistry, registry::opentelemetry_0_28::OpenTelemetryMetricsRegistry,
                test_utils::opentelemetry_0_28::global::meter,
            };
            let registry: BoxedRegistry = Box::new(OpenTelemetryMetricsRegistry::new(meter("test")));
            ($f)(&registry);
        }

        #[test]
        fn test_opentelemetry_0_29() {
            use $crate::{
                metrics::BoxedRegistry, registry::opentelemetry_0_29::OpenTelemetryMetricsRegistry,
                test_utils::opentelemetry_0_29::global::meter,
            };
            let registry: BoxedRegistry = Box::new(OpenTelemetryMetricsRegistry::new(meter("test")));
            ($f)(&registry);
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::metrics::{BoxedCounter, BoxedGauge, BoxedHistogram, BoxedRegistry};

    #[allow(dead_code)]
    struct Model {
        c: BoxedCounter,
        g: BoxedGauge,
        h: BoxedHistogram,
    }

    impl Model {
        fn new(registry: &BoxedRegistry) -> Self {
            let cv = registry.register_counter_vec("counter".into(), "counter".into(), &["k1"]);
            let gv = registry.register_gauge_vec("gauge".into(), "gauge".into(), &["k1"]);
            let hv = registry.register_histogram_vec("histogram".into(), "histogram".into(), &["k1"]);

            let c = cv.counter(&["v1".into()]);
            let g = gv.gauge(&["v1".into()]);
            let h = hv.histogram(&["v1".into()]);

            Self { c, g, h }
        }
    }

    test! {Model::new}
}
