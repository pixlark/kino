use glam::{Vec2, Vec3, Vec4};
use glow::HasContext;
use std::cell::RefCell;
use std::rc::Rc;

use super::context::Context;
use super::shaders;

#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct UnsafeRectVertex {
    gl_x: f32,
    gl_y: f32,
    index: u8,
}

impl UnsafeRectVertex {
    pub fn new(gl_x: f32, gl_y: f32, index: u8) -> UnsafeRectVertex {
        UnsafeRectVertex { gl_x, gl_y, index }
    }
}

#[derive(Copy, Clone)]
pub struct VertexObject {
    array_object: glow::NativeVertexArray,
    element_count: i32,
}

pub struct GL(Rc<glow::Context>);

impl std::ops::Deref for GL {
    type Target = glow::Context;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl std::convert::From<&Rc<RefCell<Context>>> for GL {
    fn from(context: &Rc<RefCell<Context>>) -> Self {
        let gl_context_ref = context.borrow();
        let gl_context = gl_context_ref.imgui_context.renderer.gl_context();
        GL(Rc::clone(gl_context))
    }
}

/// `GL` is a wrapper that adds helper functions to `glow::Context` which
/// encompass common OpenGL tasks.
///
/// Much of this adapted from:
///   <https://github.com/grovesNL/glow/blob/main/examples/howto/src/main.rs>
impl GL {
    pub fn create_shader_program(
        &mut self,
        shader_impl: &shaders::ShaderImpl,
    ) -> glow::NativeProgram {
        unsafe {
            let program = self.create_program().unwrap();

            let shader_sources = [
                (glow::VERTEX_SHADER, &shader_impl.vertex),
                (glow::FRAGMENT_SHADER, &shader_impl.fragment),
            ];

            let mut shaders = Vec::new();

            for (shader_type, shader_source) in &shader_sources {
                let shader = self.create_shader(*shader_type).unwrap();

                self.shader_source(shader, shader_source);
                self.compile_shader(shader);

                assert!(
                    self.get_shader_compile_status(shader),
                    "{}",
                    self.get_shader_info_log(shader)
                );

                self.attach_shader(program, shader);
                shaders.push(shader);
            }

            self.link_program(program);

            assert!(
                self.get_program_link_status(program),
                "{}",
                self.get_program_info_log(program)
            );

            for shader in shaders {
                self.delete_shader(shader);
            }

            program
        }
    }

    pub fn create_vertex_object(
        &mut self,
        vertices: &[UnsafeRectVertex],
        elements: &[u32],
    ) -> VertexObject {
        unsafe {
            // Generate vertex/element data
            let vertices_u8: Vec<u8> = vertices
                .iter()
                .flat_map(|rv| {
                    core::slice::from_raw_parts::<u8>(
                        (rv as *const UnsafeRectVertex).cast::<u8>(),
                        std::mem::size_of::<UnsafeRectVertex>(),
                    )
                    .iter()
                    .copied()
                })
                .collect();

            let elements_u8: &[u8] = core::slice::from_raw_parts(
                elements.as_ptr().cast(),
                std::mem::size_of_val(elements),
            );

            // Create VBO/EBO/VAO
            let vertex_buffer_object = self.create_buffer().unwrap();
            let element_buffer_object = self.create_buffer().unwrap();
            let vertex_array_object = self.create_vertex_array().unwrap();

            // Bind VAO first
            self.bind_vertex_array(Some(vertex_array_object));

            // Bind/Load VBO/EBO data
            self.bind_buffer(glow::ARRAY_BUFFER, Some(vertex_buffer_object));
            self.buffer_data_u8_slice(glow::ARRAY_BUFFER, &vertices_u8, glow::STATIC_DRAW);

            self.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(element_buffer_object));
            self.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, elements_u8, glow::STATIC_DRAW);

            // Mark vertex attributes
            self.vertex_attrib_pointer_f32(
                0,
                2,
                glow::FLOAT,
                false,
                i32::try_from(core::mem::size_of::<UnsafeRectVertex>()).unwrap(),
                0,
            );
            self.enable_vertex_attrib_array(0);

            self.vertex_attrib_pointer_i32(
                1,
                1,
                glow::UNSIGNED_BYTE,
                i32::try_from(core::mem::size_of::<UnsafeRectVertex>()).unwrap(),
                i32::try_from(core::mem::size_of::<f32>() * 2).unwrap(),
            );
            self.enable_vertex_attrib_array(1);

            // Unbind VBO, then VAO, and only _then_ the EBO
            self.bind_buffer(glow::ARRAY_BUFFER, None);
            self.bind_vertex_array(None);
            self.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);

            VertexObject {
                array_object: vertex_array_object,
                element_count: i32::try_from(elements.len()).unwrap(),
            }
        }
    }

    pub fn clear(&mut self, color: Vec4) {
        unsafe {
            self.clear_color(color.x, color.y, color.z, color.w);
            self.0.clear(glow::COLOR_BUFFER_BIT);
        }
    }

    pub fn draw_from_vertex_array_object(&mut self, vertex_object: VertexObject) {
        unsafe {
            self.bind_vertex_array(Some(vertex_object.array_object));
            self.draw_elements(
                glow::TRIANGLES,
                vertex_object.element_count,
                glow::UNSIGNED_INT,
                0,
            );
        }
    }

    pub fn use_program(&mut self, program: glow::NativeProgram) {
        unsafe {
            self.0.use_program(Some(program));
        }
    }
}

#[allow(dead_code)]
pub enum UniformValue {
    Scalar(f32),
    Vec2(Vec2),
    Vec3(Vec3),
    Vec4(Vec4),
}

pub struct Uniform {
    /// `location` is an `Option` in case GLSL decides to optimize out a uniform variable
    location: Option<glow::NativeUniformLocation>,
}

impl Uniform {
    pub fn new(gl: &GL, program: glow::NativeProgram, name: &str) -> Self {
        Uniform {
            location: unsafe { gl.get_uniform_location(program, name) },
        }
    }
    pub fn set(&self, gl: &GL, value: &UniformValue) {
        match *value {
            UniformValue::Scalar(f) => unsafe { gl.uniform_1_f32(self.location.as_ref(), f) },
            UniformValue::Vec2(Vec2 { x, y }) => unsafe {
                gl.uniform_2_f32(self.location.as_ref(), x, y);
            },
            UniformValue::Vec3(Vec3 { x, y, z }) => unsafe {
                gl.uniform_3_f32(self.location.as_ref(), x, y, z);
            },
            UniformValue::Vec4(vec4) => unsafe {
                gl.uniform_4_f32(self.location.as_ref(), vec4.x, vec4.y, vec4.z, vec4.w);
            },
        }
    }
}
