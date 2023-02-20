use std::fs;
use std::io::Cursor;
use std::path::Path;

use gtk::pango::{AttrList, AttrSize};
use gtk::{prelude::*, Box, Label, Orientation, ScrolledWindow};
use icalendar::{self, parser, Calendar};

const DAYS: [&str; 8] = [
    "",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

fn set_font_size(label: &Label, size: i32) -> &Label {
    let attr = label.attributes();
    match attr {
        Some(a) => {
            let size = AttrSize::new(size * 1000);
            a.insert(size);
            label.set_attributes(Option::Some(&a))
        }
        None => {
            let a = AttrList::new();
            let size = AttrSize::new(size * 1000);
            a.insert(size);
            label.set_attributes(Option::Some(&a))
        }
    }
    label
}

pub fn build_calendar(cal: &Box) {
    let row_height = 30;
    let column_width = 80;
    let header_font_size = 13;
    let content_font_size = 12;

    let header = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();

    for i in 0..8 {
        header.append(set_font_size(
            &Label::builder()
                .label(DAYS[i])
                .width_request(column_width)
                .build(),
            header_font_size,
        ));
    }

    let content = ScrolledWindow::builder().height_request(300).build();

    let event_space = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(10)
        .build();

    let hours = Box::builder().orientation(Orientation::Vertical).build();

    for i in 1..=24 {
        hours.append(set_font_size(
            &Label::builder()
                .label(i.to_string())
                .width_request(column_width)
                .height_request(row_height)
                .build(),
            content_font_size,
        ))
    }

    event_space.append(&hours);
    for _i in 0..7 {
        event_space.append(
            &Label::builder()
                .label("test")
                .width_request(column_width)
                .height_request(row_height)
                .build(),
        );
    }

    content.set_child(Some(&event_space));

    cal.append(&header);
    cal.append(&content);
}

async fn fetch_url(
    url: &String,
    file_name: String,
) -> Result<String, std::boxed::Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let mut file = std::fs::File::create(&file_name)?;
    let mut content = Cursor::new(response.bytes().await?);
    std::io::copy(&mut content, &mut file)?;
    Ok(file_name)
}

fn display_ics(calendar_widget: &Box, cal: Calendar) {
    println!("test");
}

pub fn load_ics(calendar_widget: &Box, path: &str) {
    display_ics(
        calendar_widget,
        match fs::read_to_string(path) {
            Ok(string) => match parser::read_calendar(&string) {
                Ok(cal) => cal.into(),
                Err(_e) => Calendar::new(),
            },
            Err(_e) => Calendar::new(),
        },
    );
}

pub async fn load_remote_ics(calendar_widget: &Box, url: &String) {
    println!("{url}");
    let file_name = match Path::new(&url).file_name() {
        Some(name) => match name.to_str() {
            Some(name) => name,
            None => "file.ics",
        },
        None => "file.ics",
    };
    match fetch_url(url, String::from(file_name)).await {
        Ok(string) => load_ics(calendar_widget, &string),
        Err(_) => {}
    };
}
