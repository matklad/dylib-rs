# Rust dynamic libraries example

This is an example of reloading dynamic libraries with the help of [libloading]
crate. It implements a simple repl, which can say hello and goodbye. You can
dynamically extend the repl with plugins.

To run the repl itself, execute `cargo run`. Available commands are `greet`, `exit`, `reload`.

To compile example plugins:

```
$ cd libs
$ rustc --crate-type dylib hi.rs
$ rustc --crate-type dylib bye.rs
```

To load plugin in the repl, use `reload` command. To unload a plugin, remove
`libs/lib{hi, bye}.so` and use `reload` command.


[libloading]: https://github.com/nagisa/rust_libloading
