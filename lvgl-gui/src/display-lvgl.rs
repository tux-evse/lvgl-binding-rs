/*
 * Copyright (C) 2015-2022 IoT.bzh Company
 * Author: Fulup Ar Foll <fulup@iot.bzh>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use lvgl::prelude::*;

pub struct DisplayHandle {
    handle: LvglHandle,
    panel: Vec<&'static LvglWidget>,
    ctrlbox: Option<*mut dyn LvglHandler>,
}

impl DisplayHandle {
    pub fn create(x_res: i16, y_res: i16, ratio: u32) -> Self {
        let handle = LvglHandle::new(x_res, y_res, ratio);

        let display = DisplayHandle {
            handle,
            panel: Vec::new(),
            ctrlbox: None,
        };
        display
    }

    pub fn set_theme(
        &mut self,
        primary: LvglColor,
        secondary: LvglColor,
        dark: bool,
        font: &LvglFont,
    ) -> &mut Self {
        self.handle.set_theme(primary, secondary, dark, font);
        self
    }

    pub fn set_callback(&mut self, ctrlbox: Box<dyn LvglHandler>) -> &mut Self {
        self.ctrlbox = Some(Box::leak(ctrlbox));
        self
    }

    pub fn get_panel<'a>(&'a self) -> &'a Vec<&'static LvglWidget> {
        &self.panel
    }

    pub fn get_by_uid(&self, uid: &str) -> Option<&'static LvglWidget> {
        let widget = match self
            .panel
            .binary_search_by(|widget| widget.get_uid().cmp(uid))
        {
            Ok(index) => Some(self.panel[index]),
            Err(_) => None,
        };
        widget
    }

    pub fn draw_panel(&mut self) -> &mut Self {
        self.panel.push(
            LvglLabel::new("Label-1", "Demo Label widget", "Tux EvSe UI", 20, 400)
                .set_size(300, 100)
                .set_color(LvglColor::rvb(0, 0, 0))
                .set_background(LvglColor::rvb(0xFF, 0xFF, 0xFF))
                .set_border(3, LvglColor::rvb(0, 0xff, 0))
                .finalize(),
        );

        self.panel.push(
            LvglIcon::new("Icon-wifi", "Demo Wifi Icon", LvglPixmap::WIFI, 750, 0).finalize(),
        );

        self.panel.push(
            LvglIcon::new(
                "Icon-Battery",
                "Demo Battery Icon",
                LvglPixmap::BATTERY_2,
                700,
                0,
            )
            .set_size(75, 75)
            .finalize(),
        );

        self.panel.push(
            LvglButton::new("Button-A", "Demo Button 1", "Test-1", 100, 100)
                .set_size(180, 100)
                .finalize(),
        );

        self.panel
            .push(LvglButton::new("button-B", "Demo Button 2", "Test-2", 300, 100).finalize());

        self.panel.push(
            LvglImgButton::new(
                "btn_img",
                "Demo Image Button",
                "waiting",
                500,
                400,
                LvglColor::palette(LvglPalette::BLUE_GREY),
                5,
                10,
            )
            .set_size(180, 100)
            .finalize(),
        );

        self.panel.push(
            LvglTextArea::new("Text-Area", "Demo Text area Zone", 0, 750)
                .set_width(800)
                .set_text("display message zone")
                .finalize(),
        );

        self
    }

    pub fn finalize(&mut self) {
        // sort widget by uid and add them to pannel pool
        self.panel.sort_by(|a, b| a.get_uid().cmp(&b.get_uid()));
        for widget in &self.panel {
            widget.set_callback(self.ctrlbox);
        }

        self.handle.start_loop(); // start refresh thread
    }
}
