use crate::{
    aabb::AABB,
    bvh::BVH,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    prelude::{HittableType, MODELS_DIR, MaterialType, UV},
    shape::Triangle,
    vector::{Point3, Vec3},
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
    pub fn new(file_name: impl AsRef<Path>, material: MaterialType) -> Result<Self> {
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

        for model in models {
            let mesh = model.mesh;

            for chunk in mesh.indices.chunks_exact(3) {
                let i = chunk[0] as usize;
                let j = chunk[1] as usize;
                let k = chunk[2] as usize;

                let p0 = Self::vertex(&mesh.positions, i);
                let p1 = Self::vertex(&mesh.positions, j);
                let p2 = Self::vertex(&mesh.positions, k);

                // we assume normals are provided in obj file
                let n0 = Self::vertex(&mesh.normals, i);
                let n1 = Self::vertex(&mesh.normals, j);
                let n2 = Self::vertex(&mesh.normals, k);

                // we assume uv are provided in obj file
                let uv0 = Self::get_uv(&mesh.texcoords, i);
                let uv1 = Self::get_uv(&mesh.texcoords, j);
                let uv2 = Self::get_uv(&mesh.texcoords, k);

                let triangle = Triangle::new_with_normals_uvs(
                    p0,
                    p1,
                    p2,
                    material.clone(),
                    n0,
                    n1,
                    n2,
                    uv0,
                    uv1,
                    uv2,
                );

                triangles.add(Arc::new(triangle));
            }
        }

        Ok(Self {
            root: Arc::new(BVH::bvh_node(&mut triangles, &mut rng)),
        })
    }

    fn vertex(vector: &[f32], index: usize) -> Vec3 {
        let base = index * 3;

        Point3::new(
            vector[base] as f64,
            vector[base + 1] as f64,
            vector[base + 2] as f64,
        )
    }

    fn get_uv(textcoords: &[f32], index: usize) -> UV {
        let base = index * 2;

        let u = textcoords[base] as f64;
        let v = textcoords[base + 1] as f64;

        UV { u, v }
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
