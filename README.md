I live-coded this renderer at RevolverConf 2018.1: <https://www.youtube.com/watch?v=LuUb9Hrl-LQ>

Development
===========

Dependencies:

    rustup target add wasm32-unknown-unknown --toolchain nightly
    cargo install --git https://github.com/alexcrichton/wasm-gc
    cargo install basic-http-server

Compiling:

    make

Running:

    basic-http-server

Open <http://localhost:4000/> in your browser.

