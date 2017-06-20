extern crate gcc;

use std::env;

fn main() {
    let mut config = gcc::Config::new();
    let config = config
        .cpp(true)
        .file("frontend/lib.cpp");

    config.include("../Box2D/Box2D");

    config.compile("libbox2d_frontend.a");
}
