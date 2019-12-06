# GNU Coreutils using rust
Why?
----

Many GNU, Linux and other utils are pretty awesome, and obviously
some effort has been spent in the past to port them to Windows. However, those projects
are either old, abandoned, hosted on CVS, written in platform-specific C, etc.
Rust provides a good, platform-agnostic way of writing systems utils that are easy
to compile anywhere, and this is as good a way as any to try and learn it.

Requirements
------------

* Rust (`cargo`, `rustc`)

Build Instructions
------------------
```bash
$ git clone https://github.com/satyendra-ops/GNU-Coreutils-using-rust.git
$ cd GNU-Coreutils-using-rust-master
```
### Cargo ###

Building uutils using Cargo is easy because the process is the same as for
every other Rust program:

#### cat command ####
```bash
$ cd cat
$ cargo build --release
$ cargo run [OPTIONS] [FILES...]
```
#### tsort command ####
```bash
$ cd tsort
$ cargo build --release
$ cargo run [FILE]
```
#### whoami command ####
```bash
$ cd whoami
$ cargo build --release
$ cargo run
```
#### id command ####
```bash
$ cd id
$ cargo build --release
$ cargo run [OPTIONS] [USERNAME]
```
