use gtk::glib;
use gtk::gdk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use three_d::*;
use three_d::context::NativeFramebuffer;
use std::sync::Arc;
use std::cell::RefCell;



struct State {
    context: core::Context,        // Three-d context
    target: RenderTarget<'static>, // RenderTarget associated with the context
    model: Box<dyn Object>         // Any generic renderable Object
}

#[derive(Default)]
pub struct ThreedArea {
    // RefCell needed because all the GObject methods take &self, not &mut self.
    // Option because this can't be initialised until GObject has done its stuff
    // which is after the struct has already been created. As such, the glib
    // object_subclass macro requires that Default is already implemented, so
    // Option is an easy way out via default None.
    state: RefCell<Option<State>>
}

#[glib::object_subclass]
impl ObjectSubclass for ThreedArea {
    const NAME: &'static str = "ThreedArea";
    type Type = super::ThreedArea;
    type ParentType = gtk::GLArea;
}

impl ObjectImpl for ThreedArea {
    fn constructed(&self) {
        // Make sure the parent object is constructed
        self.parent_constructed();

        // Set a tick callback so the area can be redrawn at full FPS (~60) and
        // the scene can be animated.
        self.obj().add_tick_callback(|obj, clock| {
            let time_millis = clock.frame_time() as f32/1000.;

            obj.imp().animate_model(time_millis);
            obj.queue_draw();

            Continue(true)
        });
    }
}

impl WidgetImpl for ThreedArea {
    // Runs when the object initialises. Everything relating to three-d is set
    // up here.
    fn realize(&self) {
        // Make sure the parent GLArea is realized, and attach its buffers so
        // we can get the draw framebuffer ID.
        self.parent_realize();
        self.obj().make_current();
        self.obj().attach_buffers();

        let (ctx, target) = create_target();
        let model = create_triangle_model(&ctx);

        *self.state.borrow_mut() = Some(State {
            context: ctx,
            target,
            model: Box::new(model)
        });
    }
}

impl GLAreaImpl for ThreedArea {
    fn render(&self, _: &gdk::GLContext) -> bool {
        let state = self.state.borrow();
        let state = state.as_ref().unwrap();
        let target = &state.target;
        let model = state.model.as_ref();
        
        // The allocation of our GLArea object is the size and position given
        // to it by GTK's layout process.
        let alloc = self.obj().allocation();
        let (width, height) = (alloc.width() as u32, alloc.height() as u32);
        
        // Recreating this each time so the viewport is always the same size
        // as the area, regardless of resizing.
        let camera = Camera::new_perspective(
            Viewport::new_at_origo(width, height),
            vec3(0.0, 0.0, 2.0),
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            degrees(45.0),
            0.1,
            10.0,
        );
            
        // Clear the (R, G, B, A, depth) values and render the one model with
        // an empty list of lights.
        target.clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1., 1.));
        target.render(&camera, &[model], &[]);
  
        true
    }
}

impl ThreedArea {
    fn animate_model(&self, time: f32) {
        let mut state = self.state.borrow_mut();
        let state = state.as_mut().unwrap();

        state.model.animate(time);
    }
}

fn create_triangle_model(ctx: &core::Context) -> Gm<Mesh, ColorMaterial> {
    // Exactly as in the plain triangle example, create a model of a single
    // triangle with a color gradient between its corners, which will spin
    // when animated.
    let positions = vec![
        vec3(0.5, -0.5, 0.0),  // bottom right
        vec3(-0.5, -0.5, 0.0), // bottom left
        vec3(0.0, 0.5, 0.0)    // top
    ];
    let colors = vec![
        Color::RED,   // bottom right
        Color::GREEN, // bottom left
        Color::BLUE   // top
    ];
    let cpu_mesh = CpuMesh {
        positions: Positions::F32(positions),
        colors: Some(colors),
        ..Default::default()
    };
    let mut model = Gm::new(Mesh::new(&ctx, &cpu_mesh), ColorMaterial::default());

    model.set_animation(|time| Mat4::from_angle_y(radians(time*0.005)));

    model
}

// Must be called when a GL context is current and buffers are bound
fn create_target() -> (core::Context, RenderTarget<'static>) {
    let mut buffer_id = 0;
    
    // Assuming that buffers are currently bound, we can get the draw framebuffer
    // ID which is needed to construct a RenderTarget.
    unsafe {
        gl::GetIntegerv(gl::DRAW_FRAMEBUFFER_BINDING, &mut buffer_id);
    }

    // Create the low-level context using epoxy
    let ctx = unsafe {
        context::Context::from_loader_function(|s| epoxy::get_proc_addr(s))
    };

    // Create the mid-level context from the low-level one, and a RenderTarget
    // from the buffer id. Target size can be (0, 0) because the Camera created
    // in `render` will set the viewport to the GLArea's size.
    let ctx = core::Context::from_gl_context(Arc::new(ctx)).unwrap();
    let buffer = NativeFramebuffer((buffer_id as u32).try_into().unwrap());
    let target = RenderTarget::from_framebuffer(&ctx, 0, 0, buffer);

    (ctx, target)
}
