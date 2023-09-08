use std::time::Duration;

use cstr_core::CString;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use lvgl;
use lvgl::style::Style;
use lvgl::widgets::{Arc, Btn, Label};
use lvgl::Event;
use lvgl::{Align, Color, Display, DrawBuffer, LvError, Part, Widget};
//use lvgl::Obj;
use std::cell::RefCell;

pub enum DisplayEvent {
    /// The object has been pressed
    Pressed,

    /// The object is being pressed (sent continuously while pressing)
    Pressing,

    /// The input device is still being pressed but is no longer on the object
    PressLost,

    /// Released before `long_press_time` config time. Not called if dragged.
    ShortClicked,

    /// Called on release if not dragged (regardless to long press)
    Clicked,

    /// Pressing for `long_press_time` config time. Not called if dragged.
    LongPressed,

    /// Called after `long_press_time` config in every `long_press_rep_time` ms. Not
    /// called if dragged.
    LongPressedRepeat,

    /// Called in every case when the object has been released even if it was dragged. Not called
    /// if slid from the object while pressing and released outside of the object. In this
    /// case, `Event<_>::PressLost` is sent.
    Released,

    /// Called when an underlying value is changed e.g. position of a `Slider`.
    ValueChanged,

    /// Called on focus
    Focused,

    // screen simulator closed
    Quit,
}

pub enum DisplayWidget<'a> {
    Btn(Btn<'a>, Event<()>),
    Arc(Arc<'a>, Event<Arc<'a>>),
    Label(Label<'a>, Event<Label<'a>>),
    Display(DisplayEvent),
}

pub trait DisplayCtrl {
    fn callback(&self, widget: DisplayWidget);
}

pub struct DisplaySize {
    pub h: u32,
    pub v: u32,
}

pub struct DisplayConfig {
    pub title: &'static str,
    pub size: Option<DisplaySize>,
    pub bg_color: Option<(u8, u8, u8)>,
    pub fn_color: Option<(u8, u8, u8)>,
    pub control: Box<dyn DisplayCtrl>,
}

pub(crate) struct DisplayHandle<'a> {
    pub(self) screen: SimulatorDisplay<Rgb565>,
    pub(self) window: Window,
    pub(self) control: Box<dyn DisplayCtrl>,
    display: RefCell<Display>,
    //buffer: DrawBuffer,  // comprendre comment sortir la conseption de l'ecran du refresh
    pub(self) arc: Arc<'a>,
    //pub(self) button: Btn<'a>,
}

impl DisplayHandle<'_> {
    pub fn new(config: DisplayConfig) -> Result<&'static mut Self, LvError> {
        println!("new display handle");

        let bg_color = if let Some(value) = config.bg_color {
            value
        } else {
            (255, 255, 255)
        };

        let fn_color = if let Some(value) = config.fn_color {
            value
        } else {
            (0, 0, 0)
        };

        const HOR_RES: u32 = 240;
        const VER_RES: u32 = 240;

        let mut sim_display: SimulatorDisplay<Rgb565> =
            SimulatorDisplay::new(Size::new(HOR_RES, VER_RES));
        let output_settings = OutputSettingsBuilder::new().scale(1).build();
        let mut window = Window::new("PineTime", &output_settings);

        // LVGL will render the graphics here first, and seed the rendered image to the
        // display. The buffer size can be set freely.
        let buffer = DrawBuffer::<{ (HOR_RES * VER_RES) as usize }>::default();

        // Register your display update callback with LVGL. The closure you pass here will be called
        // whenever LVGL has updates to be painted to the display.
        let display = Display::register(buffer, HOR_RES, VER_RES, |refresh| {
            sim_display.draw_iter(refresh.as_pixels()).unwrap();
        })?;

        let mut screen = display.get_scr_act()?;

        let mut screen_style = Style::default();
        screen_style.set_bg_color(Color::from_rgb(bg_color));
        screen_style.set_radius(0);
        screen.add_style(Part::Main, &mut screen_style);

        // Create the arc object
        let mut arc = Arc::create(&mut screen)?;
        arc.set_size(150, 150);
        arc.set_align(Align::Center, 0, 10);
        arc.set_start_angle(135)?;
        arc.set_end_angle(135)?;

        let mut loading_lbl = Label::create(&mut screen)?;
        loading_lbl.set_text(CString::new("Loading...").unwrap().as_c_str());
        loading_lbl.set_align(Align::OutTopMid, 0, 0);
        //loading_lbl.set_label_align(LabelAlign::Center)?;

        let mut loading_style = Style::default();
        loading_style.set_text_color(Color::from_rgb(fn_color));
        loading_lbl.add_style(Part::Main, &mut loading_style);

        // Create the button
        // let mut button = Btn::create(&mut screen)?;
        // button.set_align(Align::LeftMid, 30, 0);
        // button.set_size(180, 80);
        // let mut btn_lbl = Label::create(&mut button)?;
        // btn_lbl.set_text(CString::new("Click me!").unwrap().as_c_str())?;

        // button.on_event(|btn, event| {
        //     println!("Btn received event: {:?}", event);
        //     config.control.callback(DisplayWidget::Btn(btn, event));
        // })?;

        let handle = Box::new(DisplayHandle {
            control: config.control,
            screen: sim_display,
            window,
            display: RefCell::new(display),
            //buffer,
            arc,
            //button,
        });
        let handle = (Box::leak(handle));

        lvgl::tick_inc(Duration::new(0, 10 * 1000000));
        lvgl::task_handler();
        handle.window.update(&handle.screen);

        Ok(handle)
    }

    // pub fn set_angle(&mut self, value: u16) -> Result<(), LvError> {
    //     self.arc.set_end_angle(value)?;
    //     self.window.update(&self.screen);

    //     Ok(())
    // }

    // this function is called every tic ms to update lvgl timer and pending tasks
    pub fn tick_update(&self, tick: Duration) {
        // update internal lvgl tick timer
        lvgl::tick_inc(tick);

        // exec any lvgl pending task/action
        lvgl::task_handler();

        // for event in self.window.events() {
        //     match event {
        //         SimulatorEvent::Quit => self
        //             .control
        //             .callback(DisplayWidget::Display(DisplayEvent::Quit)),
        //         _ => {}
        //     }
        // }
    }
}
