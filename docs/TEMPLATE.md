# CONFIGURE

For now, 

before start the server, you should link the `./src/templates` to `/tmp/www`.

**Or change** the value of `WWW_PATH` in `main.rs` and link the ./src/templates to new value.

Steps:

```
$ ln -s $(pwd)/src/templates /tmp/www

```
Then:

```
$ cargo build
$ cargo run
```

Or:

```
$ cargo build --release
$ ./target/release/lol
```

