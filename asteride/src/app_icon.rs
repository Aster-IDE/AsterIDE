/*
 * This file is part of AsterIDE.
 *
 * Copyright (c) 2026 playfairs
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     https://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! App icon handler module

#![allow(unused_imports)]

use image::GenericImageView;
use std::sync::LazyLock;

/// Returns the resource to be consumed by the app.
///
/// # Platform-specific
/// No-op for macOS. The way this is done on macOS is more elegant, read
/// from the app bundle's `Info.plist` (`CFBundleIconFile`).
#[cfg(not(target_os = "macos"))]
pub static APP_ICON: LazyLock<Option<iced::window::Icon>> = LazyLock::new(|| {
    let image = image::load_from_memory(include_bytes!(
        "../../assets/appIcon/asteride-macOS-Default-1024x1024@1x.png"
    ))
    .expect("embedded icon should be valid image data");
    let (width, height) = image.dimensions();
    let rgba = image.into_rgba8().into_raw();

    Some(
        iced::window::icon::from_rgba(rgba, width, height)
            .expect("unhandled exception from callee `from_rgba`"),
    )
});

#[cfg(target_os = "macos")]
pub static APP_ICON: Option<iced::window::Icon> = None;
