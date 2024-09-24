use editor::Editor;
use engine::scene::scene::Scene;

pub mod editor;

const EXAMPLE_SCENE_FILE: &str = "examples/scenes/with_editor/scene.rrtscene";

fn main() {
    let scene = Scene::load_config(EXAMPLE_SCENE_FILE);
    let editor = Editor { scene };

    editor.open();
}
