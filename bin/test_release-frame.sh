# Build for release
cargo build -r

./target/release/raytracer render example_scenes/lots_of_objects/scene.rrtscene frame
