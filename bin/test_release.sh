# Build for release
cargo build -r

./target/release/raytracer

ffmpeg -framerate 24 -i example_scenes/lots_of_objects/output/frame-%01d.png example_scenes/lots_of_objects/output/video.mp4
ffmpeg -framerate 24 -i example_scenes/lots_of_objects/output/frame-%01d.png example_scenes/lots_of_objects/output/video.gif