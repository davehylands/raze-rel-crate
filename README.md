This repository is a stripped down repository to show a bug in cargo raze
when relative paths are used.

mylib-rs is a really simple crate with no dependencies.

person-rs is another simple more complictaed crate which
has no dependencies except for mylib.

If you run raze.sh (which sets up the vendoring) then I get the following output:
```
+ cd mylib-rs
+ cargo vendor --versioned-dirs cargo/vendor
To use vendored sources, add this to your .cargo/config for this project:

+ cargo raze
+ cd ../person-rs
+ cargo vendor --versioned-dirs cargo/vendor
To use vendored sources, add this to your .cargo/config for this project:

+ cargo raze
Error: Error during execution of `cargo metadata`: error: failed to get `mylib` as a dependency of package `person v0.1.0 (/private/var/folders/th/d3131qsx7zvb7c_50lb3frb80000gp/T/.tmpza0C5D)`

Caused by:
  failed to load source for dependency `mylib`

Caused by:
  Unable to update /private/var/folders/th/d3131qsx7zvb7c_50lb3frb80000gp/T/mylib-rs

Caused by:
  failed to read `/private/var/folders/th/d3131qsx7zvb7c_50lb3frb80000gp/T/mylib-rs/Cargo.toml`

Caused by:
  No such file or directory (os error 2)
```

You can cd into person-rs and run `cargo test` and it will build both person-rs and mylib-rs and run the tests.
