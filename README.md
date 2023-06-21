<h1 align="center">gross</h1>

## 🗒 About

This is a (simple) bundled collection of JSON generators that should be consumed by [Eww](https://github.com/elkowar/eww).
It aims to replace my [shell scripts](https://github.com/fufexan/dotfiles/tree/e1e554fdddc2600635f6a9b9f3eb95b9a876d4c0/home/programs/eww/scripts),
which have become too complex to be written easily in bash.

## 🗃️  Contents

Currently, the program has these commands functional:
- music - general info about a song
- music-time - time info about a song

## ⚒ Building

Most of the following instructions will assume you have this repository cloned.

### 📦 Cargo

You will need several dependencies installed on your system. Cargo will print an error about missing packages in case I forget anything
```
dbus openssl pkg-config
```

When that's done, you can compile with
```bash
cargo build --release
```

Success! Now you can run the binary from `target/release/gross`.

### ❄ Nix

For this next command, you don't need to have the repo cloned. It will build and run `gross` directly.
```bash
nix run github:fufexan/gross
```

If you wish to hack on this program, you can use its `devShell`. In the cloned repo, run the command `nix develop`.
You now have everything needed to develop and build `gross`.
