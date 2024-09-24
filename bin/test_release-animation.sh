# Build for release
cargo build -r

./target/release/raytracer render example_scenes/lots_of_objects/scene.rrtscene animation 0 24

ffmpeg -framerate 24 -i example_scenes/lots_of_objects/output/frame-%01d.png example_scenes/lots_of_objects/output/video.mp4
ffmpeg -framerate 24 -i example_scenes/lots_of_objects/output/frame-%01d.png example_scenes/lots_of_objects/output/video.gif