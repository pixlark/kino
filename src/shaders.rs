use lazy_static::lazy_static;

#[derive(Debug)]
pub struct ShaderImpl {
    pub vertex: String,
    pub fragment: String,
}

macro_rules! shaders {
    ($($shadertype:literal as $name:ident),*,) => {
        pub struct Shaders {
            $(
                pub $name: ShaderImpl,
            )*
        }

        lazy_static! {
            pub static ref ALL: Shaders = Shaders {
                $(
                    $name: ShaderImpl {
                        vertex: String::from(include_str!(concat!("../resources/", $shadertype, "/vertex.glsl"))),
                        fragment: String::from(include_str!(concat!("../resources/", $shadertype, "/fragment.glsl")))
                    },
                )*
            };
        }
    }
}

shaders! {
    "rect-fill" as rect_fill,
}
