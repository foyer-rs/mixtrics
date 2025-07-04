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

/// Some phantom metrics components that do nothing.
pub mod noop;

/// Prometheus metrics components.
#[cfg(feature = "prometheus")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus")))]
pub use prometheus_0_14 as prometheus;

/// Prometheus metrics components.
#[cfg(feature = "prometheus_0_13")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus_0_13")))]
pub mod prometheus_0_13;

/// Prometheus metrics components.
#[cfg(feature = "prometheus_0_14")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus_0_14")))]
pub mod prometheus_0_14;

#[cfg(feature = "prometheus-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus-client")))]
pub use prometheus_client_0_23 as prometheus_client;

/// Prometheus metrics components.
#[cfg(feature = "prometheus-client_0_22")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus-client_0_22")))]
pub mod prometheus_client_0_22;

/// Prometheus metrics components.
#[cfg(feature = "prometheus-client_0_23")]
#[cfg_attr(docsrs, doc(cfg(feature = "prometheus-client_0_23")))]
pub mod prometheus_client_0_23;

#[cfg(feature = "opentelemetry")]
#[cfg_attr(docsrs, doc(cfg(feature = "opentelemetry")))]
pub use opentelemetry_0_29 as opentelemetry;

/// OpenTelemetry metrics components.
#[cfg(feature = "opentelemetry_0_26")]
#[cfg_attr(docsrs, doc(cfg(feature = "opentelemetry_0_26")))]
pub mod opentelemetry_0_26;

/// OpenTelemetry metrics components.
#[cfg(feature = "opentelemetry_0_27")]
#[cfg_attr(docsrs, doc(cfg(feature = "opentelemetry_0_27")))]
pub mod opentelemetry_0_27;

/// OpenTelemetry metrics components.
#[cfg(feature = "opentelemetry_0_28")]
#[cfg_attr(docsrs, doc(cfg(feature = "opentelemetry_0_28")))]
pub mod opentelemetry_0_28;

/// OpenTelemetry metrics components.
#[cfg(feature = "opentelemetry_0_29")]
#[cfg_attr(docsrs, doc(cfg(feature = "opentelemetry_0_29")))]
pub mod opentelemetry_0_29;

/// OpenTelemetry metrics components.
#[cfg(feature = "opentelemetry_0_30")]
#[cfg_attr(docsrs, doc(cfg(feature = "opentelemetry_0_30")))]
pub mod opentelemetry_0_30;
