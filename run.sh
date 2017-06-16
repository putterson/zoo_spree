export RUST_LOG=zoo_spree
if ./build.sh ; then
    target/debug/zoo_spree
fi
