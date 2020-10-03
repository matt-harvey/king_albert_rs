# king\_albert\_rs

This is a command-line implementation of the single-player card game
<a href="https://en.wikipedia.org/wiki/King_Albert_(solitaire)">King Albert</a>.

When playing King Albert with ordinary cards, only about 1 in 10 games is actually winnable.
This application, however, will perform a search for a winnable deal first, and will only
ever present you with games that it has been able to determine are winnable.

I wrote this mainly to help me learn Rust.

## How to build and play

You need Rust and Cargo.

Note King Albert has been tested on Mac OSX and Linux, but not on Windows.

Do this from within the project root:

```
cargo build --release
./target/release/king_albert_rs
```

The program will then proceed to search for a winnable game. This may take between several seconds and a few minutes,
depending on how fast your machine is and how many CPU cores are available.

You've won when the only cards you can see are kings.

## License

MIT
