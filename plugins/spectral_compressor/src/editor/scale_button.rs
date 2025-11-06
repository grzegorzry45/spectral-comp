// Spectral Compressor: an FFT based compressor
// Copyright (C) 2021-2024 Robbert van der Helm
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use nih_plug_vizia::vizia::prelude::*;

/// A button that sets a specific scale factor when clicked.
pub struct ScaleButton {
    scale_factor: f64,
}

impl ScaleButton {
    /// Creates a new button that sets the given scale factor when clicked.
    pub fn new<T>(cx: &mut Context, scale_factor: f64, label: impl Res<T> + Clone) -> Handle<Self>
    where
        T: ToString,
    {
        Self { scale_factor }
            .build(cx, |cx| {
                Label::new(cx, label).hoverable(false);
            })
            // We'll use the same styling as param-button
            .class("scale-button")
    }
}

impl View for ScaleButton {
    fn element(&self) -> Option<&'static str> {
        // Reuse the styling from param-button
        Some("param-button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseDoubleClick(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                cx.set_user_scale_factor(self.scale_factor);
                meta.consume();
            }
            _ => {}
        });
    }
}
