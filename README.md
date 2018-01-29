# King Albert

This is a command-line implementation of the single-player card game
<a href="https://en.wikipedia.org/wiki/King_Albert_(solitaire)">King Albert</a>.

I wrote this mainly to help me learn Rust. It is quite unpolished.

## How to build and play

You need Rust and Cargo.

Do this from within the project root:

```
cargo build --release
./target/release/king_albert
```

You've won when the only cards you can see are kings.

Note King Albert is often not winnable. Just CTRL-C to given up and try again :)

## License

MIT
