use std::{cell::RefCell, rc::Rc};

#[allow(dead_code)]
pub(crate) struct Sdl {
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub event_pump: sdl2::EventPump,
    pub gl_context: sdl2::video::GLContext,
}

pub(crate) struct ImGui {
    pub imgui: imgui::Context,
    pub platform: imgui_sdl2_support::SdlPlatform,
    pub renderer: imgui_glow_renderer::AutoRenderer,
}

/// `Context` contains the core API handles that control the window,
/// rendering, input, etc.
pub struct Context {
    pub(crate) sdl_context: Sdl,
    pub(crate) imgui_context: ImGui,
    exit_requested: bool,
}

// TODO(brooke.tilley): Clean out all the unwraps in this struct and set up
//                      some sort of error handling system in their place
impl Context {
    /// Create a new context by initializing all the relevant subsystems: SDL2,
    /// OpenGL, imgui, etc. It is undefined behaviour to call this more than once!
    pub fn new() -> Rc<RefCell<Context>> {
        //
        // Initialize SDL2 video subsystem
        //
        let sdl = sdl2::init().unwrap();
        let video_subsystem = sdl.video().unwrap();

        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);

        let window = video_subsystem
            .window("qsheet", 1280, 720)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        //
        // Create an OpenGL context
        //
        let gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&gl_context).unwrap();

        window.subsystem().gl_set_swap_interval(1).unwrap();

        let gl = unsafe {
            glow::Context::from_loader_function(|s| {
                window.subsystem().gl_get_proc_address(s).cast()
            })
        };

        //
        // Create an imgui context
        //
        let mut imgui = imgui::Context::create();

        // Don't clutter with .ini, .log files
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        let platform = imgui_sdl2_support::SdlPlatform::init(&mut imgui);
        let renderer = imgui_glow_renderer::AutoRenderer::initialize(gl, &mut imgui).unwrap();

        let event_pump = sdl.event_pump().unwrap();

        Rc::new(RefCell::new(Context {
            sdl_context: Sdl {
                sdl,
                video_subsystem,
                window,
                event_pump,
                gl_context,
            },
            imgui_context: ImGui {
                imgui,
                platform,
                renderer,
            },
            exit_requested: false,
        }))
    }

    pub fn exit_requested(&self) -> bool {
        self.exit_requested
    }

    pub fn pump_events(&mut self) {
        for event in self.sdl_context.event_pump.poll_iter() {
            self.imgui_context
                .platform
                .handle_event(&mut self.imgui_context.imgui, &event);

            if let sdl2::event::Event::Quit { .. } = event {
                self.exit_requested = true;
            }
        }
    }

    pub fn render_imgui(&mut self) {
        self.imgui_context.platform.prepare_frame(
            &mut self.imgui_context.imgui,
            &self.sdl_context.window,
            &self.sdl_context.event_pump,
        );

        let ui_to_render = self.imgui_context.imgui.new_frame();
        ui_to_render.show_demo_window(&mut true);

        let draw_data = self.imgui_context.imgui.render();

        self.imgui_context.renderer.render(draw_data).unwrap();
    }

    pub fn finish_frame(&mut self) {
        self.sdl_context.window.gl_swap_window();
    }
}
