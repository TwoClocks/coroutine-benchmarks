# Things you need in your path:
#   - Rust tool chain (cargo, rustc, etc.)
#   - kotlinc version 1.8.20
#   - java version 19 or better
#   - zig version 0.10.1

# Tihs assumes your running from the root of the project
#!/usr/bin/env sh

cargo build --release

cd kotlin/
echo "$PWD"
./build.sh
cd ../cpp
echo "$PWD"
./build.sh
cd ../zig
echo "$PWD"
zig build