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
 *
 */

use crate::binding::*;
use libafb::prelude::*;
use std::time::Duration;

use crate::display::*;

pub struct DisplayContext {
    event: &'static AfbEvent,
}

struct TimerData {
    pub(self) display: &'static DisplayHandle <'static>,
    pub(self) tic: Duration,
}

AfbTimerRegister!(TimerCtrl, timer_callback, TimerData);
fn timer_callback(_timer: &AfbTimer, _decount: u32, userdata: &mut TimerData) {

    // update display tic and waiting task.
    userdata.display.tick_update(userdata.tic);
}

impl DisplayCtrl for DisplayContext {
    fn callback(&self, widget: DisplayWidget) {
        match widget {
            DisplayWidget::Btn(_widget, event) => {
                println!("button event={:?}", event)
            }

            DisplayWidget::Arc(_widget, _event) => {
                println!("arc event=")
            }

            DisplayWidget::Label(_widget, _event) => {
                println!("label event=")
            }

            DisplayWidget::Display(_widget) => {
                println!("display quit event")
            }
        }
    }
}

pub fn register(api: &mut AfbApi, config: &ApiUserData) -> Result<(), AfbError> {

    // create event and store it within callback context
    let event = AfbEvent::new(config.uid);

    let display_conf = DisplayConfig {
        title: "touch screen simulator",
        size: None,
        bg_color: None,
        fn_color: None,
        control: Box::new(DisplayContext {event: event}),
    };

    let display= match DisplayHandle::new(display_conf) {
        Err(error) => return Err(AfbError::new ("lvgl-display-fail", "fail to implement lvgl display simulator")),
        Ok(value) => value,
    };

    api.add_event(event);


    let result= match AfbTimer::new("lvgl_tic")
        .set_period(config.tic)
        .set_decount(0)
        .set_callback(Box::new(TimerData {
            display: display,
            tic: Duration::new(0, config.tic*1000000),
        }))
        .start()
    {
        Err(error) => {
            afb_log_msg!(Critical, api.get_apiv4(), &error);
            Err(error)
        }
        Ok(_timer) => {Ok(())}
    };
    result

}
