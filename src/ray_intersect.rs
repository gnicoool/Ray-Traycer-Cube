use crate::color::Color;
use crate::texture::Texture;
use nalgebra_glm::Vec3;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Material {
    pub diffuse: Color,
    pub specular: f32,
    pub albedo: [f32; 2],
    pub texture: Option<Rc<Texture>>,
}

impl Material {
    pub fn new(diffuse: Color, specular: f32, albedo: [f32; 2]) -> Self {
        Material {
            diffuse,
            specular,
            albedo,
            texture: None,
        }
    }

    pub fn with_texture(specular: f32, albedo: [f32; 2], texture: Rc<Texture>) -> Self {
        Material {
            diffuse: Color::new(255, 255, 255),
            specular,
            albedo,
            texture: Some(texture),
        }
    }

    fn texture_pixel(&self, tex: &Texture, u: f32, v: f32) -> (usize, usize) {
        let x = ((u.clamp(0.0, 1.0)) * (tex.width as f32 - 1.0)).round() as usize;
        let y = ((1.0 - v.clamp(0.0, 1.0)) * (tex.height as f32 - 1.0)).round() as usize;
        (x, y)
    }

    pub fn diffuse_at(&self, u: f32, v: f32) -> Color {
        match &self.texture {
            Some(tex) => {
                let (x, y) = self.texture_pixel(tex, u, v);
                Color::from_hex(tex.get_pixel(x, y))
            }
            None => self.diffuse,
        }
    }

    // Opacidad en (u, v)
    pub fn alpha_at(&self, u: f32, v: f32) -> u8 {
        match &self.texture {
            Some(tex) => {
                let (x, y) = self.texture_pixel(tex, u, v);
                tex.get_alpha(x, y)
            }
            None => 255,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Intersect {
    pub point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub u: f32,
    pub v: f32,
    pub material: Material,
}

pub trait RayIntersect {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect>;
}
