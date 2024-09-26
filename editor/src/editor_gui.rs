use engine::{hittable::object::HittableObject, scene::scene::Scene};
use iced::{
    widget::{button, column, row, scrollable, text, Column},
    Element, Task,
};
use notify_rust::Notification;
use std::io;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorMode {
    Scene,
    Render,
    Meta,
}

impl EditorMode {
    fn title(&self) -> &str {
        match self {
            EditorMode::Scene => "Scene explorer",
            EditorMode::Render => "Render",
            EditorMode::Meta => "Meta",
        }
    }

    fn button(&self, current: &EditorMode) -> Element<Message> {
        let active_indicator = if self == current {
            text("🔵").width(20)
        } else {
            text(" ").width(20)
        };
        button(row![active_indicator, self.title()])
            .on_press(Message::ModeChanged(self.clone()))
            .into()
    }
}

pub struct EditorGUI {
    scene: Arc<Scene>,
    mode: EditorMode,
    is_loading: bool,
}

#[derive(Debug, Clone)]
pub enum Message {
    ModeChanged(EditorMode),
    OpenSceneFile,
    OpenedSceneFile(Result<Arc<Scene>, Error>),
}

#[derive(Debug, Clone)]
pub enum Error {
    DialogClosed,
    IoError(io::ErrorKind),
}

async fn open_scene_file() -> Result<Arc<Scene>, Error> {
    let picked_file = rfd::AsyncFileDialog::new()
        .add_filter("rrt scene files", &["rrtscene"])
        .set_title("Open a text file...")
        .pick_file()
        .await
        .ok_or(Error::DialogClosed)?;
    let path = picked_file.path().to_str();

    match path {
        Some(p) => {
            let scene = Scene::load_config(p);
            return Ok(Arc::new(scene));
        }
        None => Err(Error::IoError(io::ErrorKind::InvalidData)),
    }
}

impl EditorGUI {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                scene: Arc::new(Scene::create_example_scene(
                    "~/example_scene.rrtscene".to_string(),
                )),
                mode: EditorMode::Scene,
                is_loading: false,
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ModeChanged(mode) => {
                self.mode = mode;
                Task::none()
            }
            Message::OpenSceneFile => {
                if self.is_loading {
                    Task::none()
                } else {
                    self.is_loading = true;

                    Task::perform(open_scene_file(), Message::OpenedSceneFile)
                }
            }
            Message::OpenedSceneFile(Ok(scene)) => {
                self.scene = scene;
                self.is_loading = false;
                self.mode = EditorMode::Scene;
                Notification::new()
                    .summary("Success")
                    .body("Scene opened")
                    .show()
                    .unwrap();
                Task::none()
            }
            Message::OpenedSceneFile(Err(error)) => {
                match error {
                    Error::DialogClosed => {
                        Notification::new()
                            .summary("Error")
                            .body("Dialog closed")
                            .show()
                            .unwrap();
                    }
                    Error::IoError(kind) => {
                        Notification::new()
                            .summary("Error")
                            .body(&kind.to_string())
                            .show()
                            .unwrap();
                    }
                }
                eprintln!("Error opening scene: {:?}", error);
                self.is_loading = false;
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let mode_tabbar: Element<Message> = row![
            EditorMode::Scene.button(&self.mode),
            EditorMode::Render.button(&self.mode),
            EditorMode::Meta.button(&self.mode),
        ]
        .into();

        column![
            mode_tabbar,
            match self.mode {
                EditorMode::Scene => {
                    let mut children: Vec<Element<Message>> = vec![];

                    for object in self.scene.world.objects.iter() {
                        let name = match object {
                            HittableObject::Sphere(_) => format!("Sphere"),
                            HittableObject::List(_) => format!("List"),
                        };
                        children.push(text(name).into());
                    }

                    row![scrollable(Column::from_vec(children).width(200))]
                }
                EditorMode::Render => row![text("Render is WIP")],
                EditorMode::Meta => {
                    row![button("Open different scene").on_press(Message::OpenSceneFile)]
                }
            }
        ]
        .into()
    }
}
