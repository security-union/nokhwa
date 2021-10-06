/*
 * Copyright 2021 l1npengtul <l1npengtul@protonmail.com> / The Nokhwa Contributors
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[cfg(all(windows, not(feature = "docs-only")))]
fn main() {
    windows::build!(
        Windows::Win32::Media::MediaFoundation::*,
        Windows::Win32::System::Com::{CoInitializeEx, COINIT, CoUninitialize},
        Windows::Win32::Foundation::{S_OK},
        Windows::Win32::Graphics::DirectShow::*,
    )
}

#[cfg(any(not(windows), feature = "docs-only"))]
fn main() {}
