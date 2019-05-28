# To the program, put the code in `hello-world.rs` and
# use `rustc` to compile it into a single binary and run it.
$ rustc hello-world.rs
$ ./hello-world
Hello, world!

# Sometimes we'll want to build a project with multiple
# files and not just a single file. Put `hello-world.rs`
# in a directory, `cd` into it and use `cargo init` to
# initialize a project there, then `cargo run` to compile
# and run.
$ cargo init
$ cargo run
Compiling hello-world v0.1.0 (/home/rustbyexample/examples/hello-world)
  Finished dev [unoptimized + debuginfo] target(s) in 12.92s
    Running `target/debug/hello-world`
Hello, world!

# Now that we can build and run basic Rust programs, let's
# learn more about the language.
