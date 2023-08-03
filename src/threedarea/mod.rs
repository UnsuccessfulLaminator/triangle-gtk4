mod imp;

use gtk::glib;
use glib::Object;



// This ThreedArea struct is the one that will be publicly visible when using
// this module. This is an entirely separate struct to the ThreedArea in imp.rs
// but it is typical in rust GObject subclassing to name them identically.
glib::wrapper! {
    pub struct ThreedArea(ObjectSubclass<imp::ThreedArea>)
        @extends gtk::GLArea, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl ThreedArea {
    pub fn new() -> Self {
        // Nothing special needs to be done here. All our custom initialisation
        // happens in `constructed` and `realize` in imp.rs
        Object::builder().build()
    }
}
