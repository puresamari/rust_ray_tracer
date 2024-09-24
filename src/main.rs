use scene::scene::Scene;

mod math;
mod ray_tracer;
mod scene;

const EXAMPLE_SCENE_FILE: &str = "example_scenes/lots_of_objects/scene.rrtscene";

fn main() {
    let mut scene = Scene::load_config(EXAMPLE_SCENE_FILE);
    // scene.render();
    scene.render_animation(0, 24);
    scene.save_config(EXAMPLE_SCENE_FILE);
}
