# PictuRust

Image manipulator made in Rust.

## Usage
Dowload the latest release and run.

```
 ./picturust -<flag> <value>
``` 




## Compile from source

This project require Nasm.

### Install Nasm

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

### Build from source

```
cargo run --release -- -i /path/to/image -o /path/to/desired/output -<Flag> <Value>
```




## About this project

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



## Looks & Performance

### Looks per resizing Algo

|  Algo  | Size   | Results |
|--------|--------|---------|
|  None  | 1024x1024  | [https://github.com/SoapyDev/PictuRust/Assets/Initial.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/Initial.png)     |
|Lanczos3|612x612   | [https://github.com/SoapyDev/PictuRust/Assets/lanczos3_small.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/lanczos3_small.png)      |
|Lanczos3|2048x2048     | [https://github.com/SoapyDev/PictuRust/Assets/lanczos3_big.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/lanczos3_big.png)           |
|Gaussian|612x612   | [https://github.com/SoapyDev/PictuRust/Assets/gaussian_small.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/gaussian_small.png)          |
|Gaussian|2048x2048     | [https://github.com/SoapyDev/PictuRust/Assets/gaussian_big.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/gaussian_big.png)          |
|CatmullRom|612x612 | [https://github.com/SoapyDev/PictuRust/Assets/catmullrom_small.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/catmullrom_small.png)          |
|CatmullRom|2048x2048   | [https://github.com/SoapyDev/PictuRust/Assets/catmullrom_big.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/catmullrom_big.png)          |
|Triangle|612x612   | [https://github.com/SoapyDev/PictuRust/Assets/triangle_small.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/triangle_small.png)          |
|Triangle|2048x2048     | [https://github.com/SoapyDev/PictuRust/Assets/triangle_big.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/triangle_big.png)          |
|Nearest|612x612    | [https://github.com/SoapyDev/PictuRust/Assets/nearest_small.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/nearest_small.png)         |
|Nearest|2048x2048      | [https://github.com/SoapyDev/PictuRust/Assets/nearest_big.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/nearest_big.png)         |
|Thumbnail|612x612  | [https://github.com/SoapyDev/PictuRust/Assets/thumbnail_small.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/thumbnail_small.png)          |
|Thumbnail|2048x2048    | [https://github.com/SoapyDev/PictuRust/Assets/thumbnail_big.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/thumbnail_big.png)         |
|Fill|612x612       | [https://github.com/SoapyDev/PictuRust/Assets/fill_small.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/fill_small.png)          |
|Fill|2048x2048  | [https://github.com/SoapyDev/PictuRust/Assets/fill_big.png](https://github.com/SoapyDev/PictuRust/blob/main/Assets/fill_big.png)          |

### Weight pes conversion format

|   Format  | Weight      | Ratio | Results |
|-----------|-------------|-------|---------|
|  PNG      |   1,357KiB  |   1   |         |
| Jpeg      |   119KiB    | 0.088 |         |
| Tiff      |   3,146KiB  | 4.269 |         |
|Webp(100%) |   284KiB    | 0.209 |         |
|Webp(70%)  |   60KiB     | 0.044 |         |
|Avif(70%,7)|   55.6KiB   | 0.040 |         |
|Avif(70%,3)|   53.2KiB   | 0.039 |         |


## Future plan
### Support to be added 

- Webp (Read)
- Avif (Read)

### Features

- Controll over multithreading
- Cropping tool
- Verbose
- maybe Preview
- Better visuals on command line
