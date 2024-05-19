kleinian
========

A program that draws fractal limit sets of Kleinian groups.

Running the program
===================

There is a command line program as well as a web interface.
To see the options for the command line program:
```sh
cd kleinian-cli
cargo run --release -- --help
```
To build the web interface:
```sh
cd kleinian-web
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen ../target/wasm32-unknown-unknown/release/kleinian_web.wasm --out-dir pkg --target web --no-typescript
```
Then the web interface can be tested with any web server.  For example:
```
cd pkg
python3 -m http.server
```

Further reading
===============
Much of the mathematics behind this program is explained in *Indra's Pearls*
by Mumford, Series, and Wright.


License
=======
Dual licensed under the [MIT License](LICENSE-MIT) and the
[Apache License, Version 2.0](LICENSE-APACHE).
