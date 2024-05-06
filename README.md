# RASH

**Rash** (Rust hash) is a very simple CLI for hashing files.

Allows for sorting and short hashes (note that short hashes will increase chance of collisions).

## Example

```bash
$ rash --short --sort Cargo.toml Cargo.toml Cargo.lock Cargo.toml Cargo.lock README.md`
58958c16  README.md
beb37701  Cargo.lock
beb37701  Cargo.lock
c2395610  Cargo.toml
c2395610  Cargo.toml
c2395610  Cargo.toml
```
