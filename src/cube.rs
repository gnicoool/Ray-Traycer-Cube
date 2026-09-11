use crate::ray_intersect::{Intersect, Material, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Cube {
    pub center: Vec3,
    pub size: f32,
    pub material: Material,
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Option<Intersect> {
        let half = self.size / 2.0;
        let min = self.center - Vec3::new(half, half, half);
        let max = self.center + Vec3::new(half, half, half);

        let mut t_min = f32::NEG_INFINITY;
        let mut t_max = f32::INFINITY;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);

        for axis in 0..3 {
            let origin = ray_origin[axis];
            let dir = ray_direction[axis];
            let min_a = min[axis];
            let max_a = max[axis];

            if dir.abs() < 1e-8 {
                if origin < min_a || origin > max_a {
                    return None;
                }
                continue;
            }

            let inv_dir = 1.0 / dir;
            let mut t1 = (min_a - origin) * inv_dir;
            let mut t2 = (max_a - origin) * inv_dir;
            let mut sign = -1.0;

            if t1 > t2 {
                std::mem::swap(&mut t1, &mut t2);
                sign = 1.0;
            }

            if t1 > t_min {
                t_min = t1;
                normal = Vec3::new(0.0, 0.0, 0.0);
                normal[axis] = sign;
            }

            if t2 < t_max {
                t_max = t2;
            }

            if t_min > t_max {
                return None;
            }
        }

        let t = if t_min > 0.0 { t_min } else { t_max };

        if t <= 0.0 {
            return None;
        }

        let point = ray_origin + ray_direction * t;

        Some(Intersect {
            point,
            normal,
            distance: t,
            material: self.material,
        })
    }
}
