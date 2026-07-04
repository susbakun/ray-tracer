use crate::{
    aabb::AABB,
    bvh::BVH,
    color::Color,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    material::Metal,
    prelude::{HittableType, MODELS_DIR},
    triangle::Triangle,
    vector::Point3,
};
use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tobj::LoadOptions;

pub struct Model {
    root: HittableType,
}

impl Model {
    pub fn new(file_name: impl AsRef<Path>) -> Result<Self> {
        let file_dir = PathBuf::from(MODELS_DIR).join(file_name);

        let mut triangles = HittableList::new();

        let mut rng = rand::rng();

        let options = LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        };

        // ignore material property for now
        let (models, _) = tobj::load_obj(file_dir, &options)?;

        let mat = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

        for model in models {
            let mesh = model.mesh;

            for chunk in mesh.indices.chunks_exact(3) {
                let i = chunk[0] as usize;
                let j = chunk[1] as usize;
                let k = chunk[2] as usize;

                let p0 = Self::vertex(&mesh.positions, i);
                let p1 = Self::vertex(&mesh.positions, j);
                let p2 = Self::vertex(&mesh.positions, k);

                let triangle = Triangle::new(p0, p1, p2, mat.clone());

                triangles.add(Arc::new(triangle));
            }
        }

        Ok(Self {
            root: Arc::new(BVH::bvh_node(&mut triangles, &mut rng)),
        })
    }

    fn vertex(positions: &Vec<f32>, index: usize) -> Point3 {
        let base = index * 3;

        Point3::new(
            positions[base] as f64,
            positions[base + 1] as f64,
            positions[base + 2] as f64,
        )
    }
}

impl Hittable for Model {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        self.root.hit(ray, ray_t, rec)
    }

    fn bounding_box(&self) -> &AABB {
        self.root.bounding_box()
    }
}
