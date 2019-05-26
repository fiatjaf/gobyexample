## Rust by Example

Content and build toolchain for [Rust by Example](https://rustbyexample.alhur.es),
a site that teaches Rust via annotated example programs.

This is a fork of [Go by Example](https://gobyexample.com/) that
aims to be the most useful guide for beginner Rust programmers as
that is for Go programmers.

The [official "Rust by Example"](https://doc.rust-lang.org/stable/rust-by-example/)
site is more like a tutorial/full manual than a quick reference of
useful constructs, so there's this one here.


### Overview

The Rust by Example site is built by extracting code and
comments from source files in `examples` and rendering
them via the `templates` into a static `public`
directory. The programs implementing this build process
are in `tools`, along with some vendor'd dependencies
in `vendor`.

The built `public` directory can be served by any
static content system. The production site uses S3 and
CloudFront, for example.


### Building

To build the site you'll need Go and Python installed. Run:

```console
$ go get github.com/russross/blackfriday
$ tools/build
$ open public/index.html
```

To build continuously in a loop:

```console
$ tools/build-loop
```

### Publishing

The site will be built in the `docs/` subdirectory. GitHub pages supports
hosting that but you can also publish elsewhere.

### License

This work is a fork of [Go by Example]() by Mark McGranaghan and licensed under a
[Creative Commons Attribution 3.0 Unported License](http://creativecommons.org/licenses/by/3.0/).
