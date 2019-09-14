# king\_albert\_rs

This is a command-line implementation of the single-player card game
<a href="https://en.wikipedia.org/wiki/King_Albert_(solitaire)">King Albert</a>.

I wrote this mainly to help me learn Rust.

## How to build and play

You need Rust and Cargo.

Note King Albert has been tested on Mac OSX and Linux, but not on Windows.

Do this from within the project root:

```
cargo build --release
./target/release/king_albert_rs
```

You've won when the only cards you can see are kings.

Note King Albert is often not winnable. Just CTRL-C to give up and try again :)

## License

MIT
