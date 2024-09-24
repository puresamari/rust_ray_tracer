use crate::ray_tracer::{
    camera::{Camera, CameraConfig},
    hittable_list::HittableList,
};

pub struct Scene {
    pub world: HittableList,
    camera: Camera,
}

impl Scene {
    pub fn new(world: HittableList, config: CameraConfig) -> Self {
        Self {
            world,
            camera: Camera::new_with_config(config),
        }
    }

    pub fn render(&mut self) {
        self.camera.render(&self.world);
    }
}
