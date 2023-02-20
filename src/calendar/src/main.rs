mod calendar;

use std::fs;

use calendar::{build_calendar, load_remote_ics};

//use chrono::{Local, TimeZone};

use gtk::gdk::Display;
use gtk::glib::ExitCode;
use gtk::{prelude::*, Button, CssProvider, StyleContext};
use gtk::{Application, ApplicationWindow, Box, Orientation};

const APP_ID: &str = "org.enyo.calendar";

#[no_mangle]
pub extern "C" fn build_ui(app: &Application) {
    let calendar = Box::builder()
        .orientation(Orientation::Vertical)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    build_calendar(&calendar);

    let button = Button::builder().label("Load").build();

    let container = Box::builder().build();
    container.append(&calendar);
    container.append(&button);

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Calendar")
        .child(&container)
        .build();

    button.connect_clicked(move |_| {
        println!("pressed");
        load_remote_ics(&calendar, &fs::read_to_string("token.txt").unwrap());
    });

    win.present();
}

/*fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| {
        let css_provider = CssProvider::new();
        //css_provider.load_from_path("../assets/style.css");
        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
    app.connect_activate(build_ui);

    app.run()
}*/

fn main() -> ExitCode {
    ExitCode::SUCCESS
}