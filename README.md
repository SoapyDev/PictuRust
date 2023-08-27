# PictuRust

## Image manipulator - Written in Rust
-----------------
### Usage

#### Install Nasm

Fedora: 
```
sudo dnf install nasm
```
Ubuntu: 
```
sudo apt-get install nasm
```
Winget: 
```
winget install -e --id NASM.NASM
```
Homebrew: 
```
brew install nasm
```
#### Build from source

```
cargo run --release -- -i /path/to/image -o /path/to/desired/output -w width
```
-------------------
### About this project

This was mainly a pet project to learn Rust and dip a toe into multithreading, file manipulation and CLI tools. 
It also happens to be efficient, quick and usefull.
I hope you enjoy it!


Currently this project support read and write of:
- Jpeg
- Png
- Tiff

Write only:
- Webp
- Avif

---------------------
### Future plan
#### Support to be added 

- Webp (Read)
- Avif (Read)

#### Features

- Cropping tool
- Verbose
- maybe Preview
