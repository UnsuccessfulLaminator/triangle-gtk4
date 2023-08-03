mod threedarea;

use gtk::prelude::*;
use gtk::glib;
use threedarea::ThreedArea;
use std::ptr;
use shared_library::dynamic_library::DynamicLibrary;



fn main() -> glib::ExitCode {
    epoxy::load_with(|s| unsafe {
        DynamicLibrary::open(None).unwrap().symbol(s).unwrap_or(ptr::null_mut())
    });

    gl::load_with(epoxy::get_proc_addr);

    let app = gtk::Application::builder().application_id("three.d.triangle").build();

    app.connect_activate(on_activate);
    app.run()
}

fn on_activate(app: &gtk::Application) {
    let area = ThreedArea::new();
    let window = gtk::ApplicationWindow::builder()
        .application(app)
        .title("Camtest")
        .child(&area)
        .build();

    window.present();
}
