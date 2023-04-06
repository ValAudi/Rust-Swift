set -e
PATH="$PATH:/Users/pamelaaudi/.cargo/bin"

# Navigate to directory
cd /Users/pamelaaudi/Documents/CodeTemplates/Rust-Swift/net_link

# Build the programs
cargo build --target aarch64-apple-ios
cargo build --target x86_64-apple-ios

# Create header file
cbindgen -l C -o target/net_link.h

# creating a fat file
lipo -create target/aarch64-apple-ios/debug/libnet_link.a target/x86_64-apple-ios/debug/libnet_link.a -output target/libnet_link.a

# copy the fatfile into the ios project
cp /Users/pamelaaudi/Documents/CodeTemplates/Rust-Swift/net_link/target/libnet_link.a /Users/pamelaaudi/Documents/CodeTemplates/Rust-Swift/ListInterfaces/net_link/

# copy header file into iOS project
cp /Users/pamelaaudi/Documents/CodeTemplates/Rust-Swift/net_link/target/net_link.h /Users/pamelaaudi/Documents/CodeTemplates/Rust-Swift/ListInterfaces/net_link/
# Type a script or drag a script file from your workspace to insert its path.
