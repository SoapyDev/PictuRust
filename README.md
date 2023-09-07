# PictuRust

## Image manipulator - Written in Rust

### Usage
---------------------------------
Dowload the latest release.

```
./picturust -<flag> <value>
``` 

### Compile from source
----------------------------------

#### Install Nasm

Fedora: 
```
sudo dnf install nasm
```
Windows: 
```
winget install -e --id NASM.NASM
```
NASM will not go into your path by itself. To do so, you can follow this tutorial
https://labs.bilimedtech.com/nasm/windows-install/2.html#download-netwide-assembler-nasm


Homebrew: 
```
brew install nasm
```
--------------------

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

- Controll over multithreading
- Cropping tool
- Verbose
- maybe Preview
- Better visuals on command line
