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

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod utils;

/// Core metric traits and helpers shared by all backends.
pub mod metrics;
/// Registry adapters for the supported metrics backends.
pub mod registry;

#[cfg(feature = "test-utils")]
#[cfg_attr(docsrs, doc(cfg(feature = "test-utils")))]
/// Utilities that simplify testing instrumentation built with mixtrics.
pub mod test_utils;
