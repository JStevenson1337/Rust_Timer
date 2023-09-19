use glib::clone;
use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
fn main() -> glib::ExitCode {
    let application = Application::builder().application_id("com.Navistar.GTKRustTimer").build();

application.connect_activate(|app| {
    let window = ApplicationWindow::builder()
        .application(app)
        .title("Navistar Rust Timer")
        .default_width(350)
        .default_height(70)
        .build();

    let button = Button::with_label("Click me!");
    button.connect_clicked(|_| {
        eprintln!("Clicked!");
    });
    window.set_child(Some(&button));

    window.present();
});

application.run()

}
