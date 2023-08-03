mod imp;

use gtk::glib;
use glib::Object;



glib::wrapper! {
    pub struct ThreedArea(ObjectSubclass<imp::ThreedArea>)
        @extends gtk::GLArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ThreedArea {
    pub fn new() -> Self {
        Object::builder().build()
    }
}
