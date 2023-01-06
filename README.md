# chip8

A CHIP-8 emulator written in Rust.

I wrote this in January of 2022 after getting covid. I had plenty of time and decided to use it to learn a little more about the language.

## Usage
If you want to check it out, clone the repository and use Cargo to build & run the program. It takes 1 argument - the file to load into memory (i.e. the game you'd like to play) It should work on most platforms.

In steps:

1. Clone & open the repository in a terminal
```sh
git clone https://github.com/solarhorizon/chip8 && cd chip8
```

2. Use [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) to run the program (it will need to compile it the first time you do this)
```sh
# tetris
cargo run -- roms/tetris.ch8

# space invaders
cargo run -- roms/spaceinvaders.ch8
```

You can download your own CHIP-8 ROMs online and play them via this emulator as well. There are plenty of them out there, even [right here on GitHub.](https://github.com/kripod/chip8-roms) 
