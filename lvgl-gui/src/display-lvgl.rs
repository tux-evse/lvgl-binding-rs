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
            LvglLabel::new("Label-1", "Tux EvSe UI", 50, 400)
                .set_info("Demo Label widget")
                .set_size(300, 100)
                .set_color(LvglColor::rvb(0, 0, 0))
                .set_background(LvglColor::rvb(0xFF, 0xFF, 0xFF))
                .set_border(3, LvglColor::rvb(0, 0xff, 0))
                .finalize(),
        );

        self.panel.push(
            LvglIcon::new("Icon-wifi", LvglPixmap::WIFI, 1000, 0)
                .set_info("Demo Wifi Icon")
                .finalize(),
        );

        self.panel.push(
            LvglIcon::new("Icon-Nfc", LvglPixmap::SD_CARD, 950, 0)
                .set_color(LvglColor::rvb(255, 0, 0))
                .finalize(),
        );

        self.panel
            .push(LvglIcon::new("Icon-Battery", LvglPixmap::BATTERY_2, 900, 0).finalize());

        self.panel.push(
            LvglLed::new("Led-Red", 850, 5)
                .set_color(LvglColor::palette(LvglPalette::RED))
                .set_size(10, 10)
                .set_on(true)
                .finalize(),
        );

        self.panel.push(
            LvglLed::new("Led-Green", 800, 5)
                .set_color(LvglColor::rvb(0, 255, 0))
                .set_brightness(255)
                .set_size(10, 10)
                .set_on(true)
                .finalize(),
        );

        let points = [
            LvglPoint { x: 5, y: 5 },
            LvglPoint { x: 70, y: 70 },
            LvglPoint { x: 120, y: 10 },
            LvglPoint { x: 180, y: 60 },
            LvglPoint { x: 240, y: 10 },
        ];
        self.panel.push(
            LvglLine::new("Line", 400, 100)
                .set_color(LvglColor::palette(LvglPalette::RED))
                .set_width(8)
                .set_rounded(true)
                .set_points(Box::new(points))
                .finalize(),
        );

        self.panel.push(
            LvglButton::new("Button-A", "Test-1", 100, 200)
                .set_info("Demo Button 1")
                .set_size(180, 100)
                .finalize(),
        );

        self.panel
            .push(LvglButton::new("Button-B", "Test-2", 300, 200).finalize());

        self.panel
            .push(LvglArc::new("Arc", 10, 270, 800, 200).finalize());

        self.panel.push(
            LvglMeter::new(
                "Meter",
                4,
                -10,
                LvglColor::palette(LvglPalette::INDIGO),
                800,
                400,
            )
            .set_size(200,200)
            .set_tic(
                3,
                10,
                41,
                10,
                8,
                LvglColor::palette(LvglPalette::BLUE_GREY),
                LvglColor::palette(LvglPalette::GREY),
            )
            .set_zone(0,20,4,LvglColor::palette(LvglPalette::RED))
            .set_zone(80,100,4,LvglColor::palette(LvglPalette::GREEN))
            .set_value(50)
            .finalize(),
        );

        self.panel.push(
            LvglTextArea::new("Text-Area", 0, 550)
                .set_info("Demo Text area Zone")
                .set_width(600)
                .set_text("display message zone")
                .finalize(),
        );

        self
    }

    pub fn finalize(&mut self) {
        // sort widget by uid and add them to pannel pool
        self.panel.sort_by(|a, b| a.get_uid().cmp(&b.get_uid()));
        for widget in &self.panel {
            println!("widget uid={}", widget.get_uid());
            widget.set_callback(self.ctrlbox);
        }
        // start lvgl main loop thread
        self.handle.start_loop();
    }
}
