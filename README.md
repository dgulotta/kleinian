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
To start the web interface:
```sh
cd kleinian-web
npm install webpack-dev-server
npm run serve
```

Further reading
===============
Much of the mathematics behind this program is explained in *Indra's Pearls*
by Mumford, Series, and Wright.


License
=======
Dual licensed under the [MIT License](LICENSE-MIT) and the
[Apache License, Version 2.0](LICENSE-APACHE).
