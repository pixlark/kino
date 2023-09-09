use glam::{Vec2, Vec4};
use std::cell::RefCell;
use std::rc::Rc;

use super::context::Context;
use super::gl::{Uniform, UniformValue, UnsafeRectVertex, VertexObject, GL};
use super::shaders;

/// `RectFill` draws a solid-color filled rectangle via OpenGL
struct RectFill {
    program: glow::NativeProgram,
    vertex_object: VertexObject,
    screen_size: Uniform,
    rect_top_left: Uniform,
    rect_bottom_right: Uniform,
    rect_color: Uniform,
}

struct RectFillArgs {
    rect_pos: Vec2,
    rect_size: Vec2,
    rect_color: Vec4,
}

impl RectFill {
    fn setup(gl: &mut GL) -> Self {
        let program = gl.create_shader_program(&shaders::ALL.rect_fill);

        let vertex_object = gl.create_vertex_object(
            &[
                UnsafeRectVertex::new(0.0, 0.0, 0),
                UnsafeRectVertex::new(1.0, 0.0, 1),
                UnsafeRectVertex::new(0.0, -1.0, 2),
                UnsafeRectVertex::new(1.0, -1.0, 3),
            ],
            &[0, 3, 2, 0, 1, 3],
        );

        Self {
            program,
            vertex_object,
            screen_size: Uniform::new(gl, program, "ScreenSize"),
            rect_top_left: Uniform::new(gl, program, "RectTopLeft"),
            rect_bottom_right: Uniform::new(gl, program, "RectBottomRight"),
            rect_color: Uniform::new(gl, program, "RectColor"),
        }
    }
    fn draw(&self, context: &Context, gl: &mut GL, args: &RectFillArgs) {
        let (screen_size_x, screen_size_y) = context.sdl_context.window.size();

        gl.use_program(self.program);

        self.screen_size.set(
            gl,
            &UniformValue::Vec2(Vec2::new(screen_size_x as f32, screen_size_y as f32)),
        );

        self.rect_top_left
            .set(gl, &UniformValue::Vec2(args.rect_pos));

        self.rect_bottom_right
            .set(gl, &UniformValue::Vec2(args.rect_pos + args.rect_size));

        self.rect_color
            .set(gl, &UniformValue::Vec4(args.rect_color));

        gl.draw_from_vertex_array_object(self.vertex_object);
    }
}

/// `Canvas` is a very simple rendering engine that wraps OpenGL.
pub struct Canvas {
    context: Rc<RefCell<Context>>,
    rect_fill: RectFill,
}

impl Canvas {
    pub fn new(context: Rc<RefCell<Context>>) -> Canvas {
        let mut gl = GL::from(&context);

        Canvas {
            context,
            rect_fill: RectFill::setup(&mut gl),
        }
    }

    pub fn clear(&mut self) {
        let mut gl = GL::from(&self.context);

        gl.clear(Vec4::new(0.0, 0.0, 0.0, 0.0));
    }

    pub fn screen_size(&self) -> Vec2 {
        let (w, h) = self.context.borrow().sdl_context.window.size();
        Vec2::new(w as f32, h as f32)
    }

    pub fn draw<R: Renderable>(&mut self, r: &R) {
        r.render(self);
    }

    pub fn rect_fill(&mut self, pos: Vec2, size: Vec2, color: Vec4) {
        self.rect_fill.draw(
            &self.context.borrow(),
            &mut GL::from(&self.context),
            &RectFillArgs {
                rect_pos: pos,
                rect_size: size,
                rect_color: color,
            },
        );
    }

    pub fn rect_outline(&mut self, pos: Vec2, size: Vec2, thickness: f32, color: Vec4) {
        // left
        self.rect_fill(pos, Vec2::new(thickness, size.y), color);
        // top
        self.rect_fill(pos, Vec2::new(size.x, thickness), color);
        // right
        self.rect_fill(
            Vec2::new(pos.x + size.x - thickness, pos.y),
            Vec2::new(thickness, size.y),
            color,
        );
        // bottom
        self.rect_fill(
            Vec2::new(pos.x, pos.y + size.y - thickness),
            Vec2::new(size.x, thickness),
            color,
        );
    }
}

pub trait Renderable {
    fn render(&self, canvas: &mut Canvas);
}
