# Build for release
cargo build -r

./target/release/raytracer render examples/scenes/lots_of_objects/scene.rrtscene animation 0 24

ffmpeg -framerate 24 -i examples/scenes/lots_of_objects/output/frame-%01d.png examples/scenes/lots_of_objects/output/video.mp4
ffmpeg -framerate 24 -i examples/scenes/lots_of_objects/output/frame-%01d.png examples/scenes/lots_of_objects/output/video.gif