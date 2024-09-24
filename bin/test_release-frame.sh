# Build for release
cargo build -r

./target/release/cli render examples/scenes/lots_of_objects/scene.rrtscene frame
