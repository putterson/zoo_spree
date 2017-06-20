# Ensure locally built Box2D is picked up
export LIBRARY_PATH="`pwd`/deps_build/Box2D:$LIBRARY_PATH"
mkdir -p deps_build
cd deps_build
cmake ../dependencies/Box2D/Box2D
make "-j$(grep -c ^processor /proc/cpuinfo)"
cd ..
cargo build
