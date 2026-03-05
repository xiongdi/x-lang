
#!/bin/bash

# Build and run hello.x with PATH that doesn't include Git
cd "C:\Users\Administrator\Documents\x-lang"

# Set PATH to Windows system paths and .cargo/bin only
export PATH="/c/Windows/System32:/c/Windows:/c/Users/Administrator/.cargo/bin"

echo "Building x..."
cargo build 2>&1 | head -100

if [ $? -eq 0 ]; then
    echo ""
    echo "Build successful! Running examples/hello.x..."
    cargo run -- run examples/hello.x
fi
