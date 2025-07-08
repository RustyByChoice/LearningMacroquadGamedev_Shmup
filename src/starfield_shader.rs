use macroquad::prelude::*;

pub struct StarfieldShader {
    pub direction_modifier : f32,
    render_target : RenderTarget,
    material : Material,
}

impl StarfieldShader {
    pub fn new(fragment_shader :&str, vertex_shader :&str) -> StarfieldShader {
        let render_target = render_target(320, 150);
        render_target.texture.set_filter(FilterMode::Nearest);

        let material = load_material(
            ShaderSource::Glsl { 
                vertex: vertex_shader, 
                fragment: fragment_shader 
            }, MaterialParams { 
                uniforms: vec![
                    UniformDesc::new("iResolution", UniformType::Float2),
                    UniformDesc::new("direction_modifier", UniformType::Float1),
                ],
                ..Default::default()
            }
        ).unwrap();
        
        return StarfieldShader {
            direction_modifier: 0.0,
            render_target,
            material
        }
    }

    pub fn render_starfield(&self, screen_width: f32, screen_height: f32) {
        self.material.set_uniform("iResolution", (screen_width, screen_height));
        self.material.set_uniform("direction_modifier", self.direction_modifier);
        gl_use_material(&self.material);
        draw_texture_ex(&self.render_target.texture, 0., 0., WHITE, DrawTextureParams 
            { 
                dest_size: Some(vec2(screen_width, screen_height)),
                ..Default::default() 
            });
        gl_use_default_material();
    }
}