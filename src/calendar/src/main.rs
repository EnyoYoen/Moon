use chrono::{Local, TimeZone};

use gtk::gdk::Display;
use gtk::glib::{ExitCode, self};
use gtk::subclass::prelude::*;
use gtk::{prelude::*, Grid, CssProvider, StyleContext, ScrolledWindow};
use gtk::{Application, ApplicationWindow, Box, Button, Label, Orientation};


const APP_ID: &str = "org.enyo.calendar";
const DAYS: [&str; 8]  = ["", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];


fn build_calendar(cal: &Grid) {
    for i in 0..8 {
        cal.attach(&Label::builder().label(DAYS[i]).build(), i as i32, 0, 1, 1);
    }

    let content = ScrolledWindow::builder()
        .build();

    cal.attach(&content, 0, 1, 8, 24);
}

fn build_ui(app: &Application) {
    let calendar = Grid::builder()
        .row_spacing(5)
        .column_spacing(10)
        .margin_bottom(10)
        .margin_top(10)
        .margin_start(10)
        .margin_end(10)
        .build();
    build_calendar(&calendar);

    let container = Box::builder()
        .build();
    container.append(&calendar);

    let win = ApplicationWindow::builder()
        .application(app)
        .title("Calendar")
        .child(&container)
        .build();

    win.present();
}

fn main() -> ExitCode {
    let app = Application::builder()
        .application_id(APP_ID)
        .build();
    
    app.connect_startup(|_| {
        let css_provider = CssProvider::new();
        //css_provider.load_from_path("src/style.css");
        StyleContext::add_provider_for_display(
            &Display::default().expect("Could not connect to a display."),
            &css_provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    });
    app.connect_activate(build_ui);

    app.run()
}






















/*

fn main() {
    let app = Application::builder()
        .application_id("com.example.calendar")
        .build();

    app.connect_activate(|app| {
        let window = ApplicationWindow::builder()
            .application(app)
            .title("Calendar")
            .default_width(400)
            .default_height(400)
            .build();

        let box_container = Box::builder()
            .orientation(Orientation::Vertical)
            .spacing(10)
            .build();

        let label = Label::builder()
            .label("Choose a date")
            .build();

        let calendar = Calendar::builder()
            .build();

        let button = Button::builder()
            .label("Get selected date")
            .build();

        let selected_date_label = Label::new(None);

        box_container.append(&label);
        box_container.append(&calendar);
        box_container.append(&button);
        box_container.append(&selected_date_label);

        button.connect_clicked(move |_| {
            let date = calendar.date();
            let (year, month, day) = date.ymd();
            let selected_date = Local.with_ymd_and_hms(year as i32, month as u32, day as u32, 0, 0, 0).unwrap();
            selected_date_label.set_text(selected_date.to_string().as_str());
        });

        window.set_child(Some(&box_container));
        window.show();
    });

    app.run();
}*/