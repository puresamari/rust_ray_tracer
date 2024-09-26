use engine::scene::scene::{RenderType, Scene};
use std::{env, fmt::Error};

enum Command {
    Render(String, RenderType),
    Editor(String),
}

impl Command {
    fn from_args(args: Vec<String>) -> Result<Command, Error> {
        match args[1].as_str() {
            "render" => {
                if args.len() < 4 {
                    panic!("Please provide a scene file and a render type.");
                }
                let scene_file = args[2].clone();
                let render_type: RenderType = match args[3].as_str() {
                    "frame" => {
                        let frame = if args.len() == 5 {
                            args[4].parse::<u32>().unwrap()
                        } else {
                            0
                        };
                        RenderType::SingleFrame(frame)
                    }
                    "animation" => {
                        if args.len() != 6 {
                            panic!("Please provide a start frame and the number of frames.");
                        }
                        let start_frame = args[4].parse::<u32>().unwrap();
                        let frames = args[5].parse::<u32>().unwrap();
                        RenderType::Animation(start_frame, frames)
                    }
                    _ => {
                        panic!("Please provide a valid render type.");
                    }
                };
                return Ok(Command::Render(scene_file, render_type));
            }
            "editor" => {
                if args.len() != 3 {
                    panic!("Please provide a scene file.");
                }

                let scene_file = args[2].clone();

                return Ok(Command::Editor(scene_file));
            }
            _ => panic!("Please provide a valid command."),
        }
    }
}

fn print_usage() {
    println!("Usage:");
    println!("  raytracer render <scene_file> frame");
    println!("  raytracer render <scene_file> frame <frame>");
    println!("  raytracer render <scene_file> animation <start_frame> <frames>");

    println!("  raytracer editor");
}

fn main() {
    if env::args().len() < 2 {
        print_usage();
        return;
    }
    match Command::from_args(env::args().collect()) {
        Ok(command) => match command {
            Command::Render(scene_file, render_type) => {
                let mut scene = Scene::load_config(&scene_file);
                scene.render(render_type);
                scene.save_config(&scene_file);
            }
            Command::Editor(_) => {
                print!("EDITOR IS WIP");
            }
        },
        _ => print_usage(),
    }
}
