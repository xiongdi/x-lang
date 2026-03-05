
#!/bin/bash
# Restore the git changes we made
cd "C:\Users\Administrator\Documents\x-lang"
git checkout HEAD -- Cargo.toml
echo "Restored Cargo.toml"
