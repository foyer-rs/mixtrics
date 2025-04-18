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

use std::{borrow::Cow, fmt::Debug};

/// Counter metric operations.
pub trait CounterOps: Send + Sync + 'static + Debug {
    /// Increase record by `val`.
    fn increase(&self, val: u64);
}

/// Gauge metric operations.
pub trait GaugeOps: Send + Sync + 'static + Debug {
    /// Increase record by `val`.
    fn increase(&self, val: u64);
    /// Decrease record by `val`.
    fn decrease(&self, val: u64);
    /// Set the record as a absolute value `val`.
    fn absolute(&self, val: u64);
}

/// Histogram metric operations.
pub trait HistogramOps: Send + Sync + 'static + Debug {
    /// Record a value.
    fn record(&self, val: f64);
}

/// A vector of counters.
pub trait CounterVecOps: Send + Sync + 'static + Debug {
    /// Get a counter within the vector of counters.
    fn counter(&self, labels: &[Cow<'static, str>]) -> BoxedCounter;
}

/// A vector of gauges.
pub trait GaugeVecOps: Send + Sync + 'static + Debug {
    /// Get a gauge within the vector of gauges.
    fn gauge(&self, labels: &[Cow<'static, str>]) -> BoxedGauge;
}

/// A vector of histograms.
pub trait HistogramVecOps: Send + Sync + 'static + Debug {
    /// Get a histogram within the vector of histograms.
    fn histogram(&self, labels: &[Cow<'static, str>]) -> BoxedHistogram;
}

/// Metrics registry.
pub trait RegistryOps: Send + Sync + 'static + Debug {
    /// Register a vector of counters to the registry.
    fn register_counter_vec(
        &self,
        name: Cow<'static, str>,
        desc: Cow<'static, str>,
        label_names: &'static [&'static str],
    ) -> BoxedCounterVec;

    /// Register a vector of gauges to the registry.
    fn register_gauge_vec(
        &self,
        name: Cow<'static, str>,
        desc: Cow<'static, str>,
        label_names: &'static [&'static str],
    ) -> BoxedGaugeVec;

    /// Register a vector of histograms to the registry.
    fn register_histogram_vec(
        &self,
        name: Cow<'static, str>,
        desc: Cow<'static, str>,
        label_names: &'static [&'static str],
    ) -> BoxedHistogramVec;

    /// Register a vector of histograms to the registry.
    fn register_histogram_vec_with_buckets(
        &self,
        name: Cow<'static, str>,
        desc: Cow<'static, str>,
        label_names: &'static [&'static str],
        buckets: Vec<f64>,
    ) -> BoxedHistogramVec;
}

/// Boxed generic counter.
pub type BoxedCounter = Box<dyn CounterOps>;
/// Boxed generic gauge.
pub type BoxedGauge = Box<dyn GaugeOps>;
/// Boxed generic histogram.
pub type BoxedHistogram = Box<dyn HistogramOps>;

/// Boxed generic counter vec.
pub type BoxedCounterVec = Box<dyn CounterVecOps>;
/// Boxed generic gauge vec.
pub type BoxedGaugeVec = Box<dyn GaugeVecOps>;
/// Boxed generic histogram vec.
pub type BoxedHistogramVec = Box<dyn HistogramVecOps>;

/// Boxed generic registry.
pub type BoxedRegistry = Box<dyn RegistryOps>;

/// Helper to create buckets for histogram.
#[derive(Debug)]
pub struct Buckets;

impl Buckets {
    /// Linear bucket distribution.
    ///
    /// e.g.
    ///
    /// ```rust
    /// use mixtrics::metrics::Buckets;
    ///
    /// assert_eq!(Buckets::linear(0.0, 1.0, 5), vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    /// ```
    pub fn linear(start: f64, width: f64, length: usize) -> Vec<f64> {
        std::iter::repeat(())
            .enumerate()
            .map(|(i, _)| start + (width * (i as f64)))
            .take(length)
            .collect()
    }

    /// Exponential bucket distribution.
    ///
    /// e.g.
    ///
    /// ```rust
    /// use mixtrics::metrics::Buckets;
    ///
    /// assert_eq!(Buckets::exponential(1.0, 2.0, 5), vec![1.0, 2.0, 4.0, 8.0, 16.0]);
    /// ```
    pub fn exponential(start: f64, factor: f64, length: usize) -> Vec<f64> {
        std::iter::repeat(())
            .enumerate()
            .map(|(i, _)| start * factor.powf(i as f64))
            .take(length)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_buckets() {
        assert_eq!(Buckets::linear(0.0, 1.0, 5), vec![0.0, 1.0, 2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_exponential_buckets() {
        assert_eq!(Buckets::exponential(1.0, 2.0, 5), vec![1.0, 2.0, 4.0, 8.0, 16.0]);
    }
}
