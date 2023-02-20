use gtk::glib::ExitCode;
use gtk::{Application, prelude::*};

const APP_ID: &str = "org.enyo.moon";

fn build_ui(app: &Application) {
    unsafe {
        let lib = libloading::Library::new("apps/libcalendar.so").unwrap();
        let calendar: libloading::Symbol<unsafe extern fn(&Application) -> u32> = lib.get(b"build_ui").unwrap();
        calendar(app);
    }
}

fn main() -> ExitCode {
    let app = Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
