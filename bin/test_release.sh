# Build for release
cargo build -r

rm image.ppm
./target/release/raytracer >> image.ppm