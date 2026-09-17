# PictuRust

Image manipulator made with Rust. 

Single picture or folders full of them, gain the ability to resize, convert, rotate or flip them from the command line. 

## Usage
Dowload the latest release and run.

```
 ./picturust -<flag> <value>
```
For help
```
./picturust -h
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
cargo build --release
```


## About this project

This was mainly a pet project to learn Rust and dip a toe into multithreading, file manipulation and CLI tools. 
It also happens to be efficient, quick and usefull.
I hope you enjoy it!


Currently this project support read and write of:
- Jpeg
- Png
- Tiff
- Webp
- Avif



## Performance & Looks

### Test suite
|     Test      |                   Flags                   |
|---------------|-------------------------------------------|
|Lanczos3       | -w 1200                                   |
|Gaussian       | -w 1200 -f Gaussian                       |
|Thumbnail      | -w 1200 -t Thumbnail                      |
|Fill           | -w 1200 -t Fill                           |
|Rotate - Flip  | -r 180 -s -v                              |
|Convert Easy   | -F Jpeg                                   |
|Convert Medium | -F Webp -Q 70.0                           |
|Convert Hard   | -F Avif -S 7 -Q 70.0                      |
|Convert Extreme| -F Avif -S 3 -Q 70.0                      |
|Insane         | -w 1200 -r 180 -s -v -F Avif -S 1 -Q 70.0 |

### Results

#### Timing and weight per test (auto-generated)

Ran against a single picture (`Assets/Initial.png`); results depend heavily on the
machine, so regenerate on yours with:
```
cargo bench --bench pipeline
cargo run --example update_readme_bench
```

<!-- BENCH:START -->
OS: CachyOS (x86_64)

CPU: AMD Ryzen 9 5950X 16-Core Processor (32 threads)

Memory: 62.7 GiB

Sample: full decode-resize-encode pipeline on `Assets/Initial.png` (single picture, no folder walking).

| Test | Mean time (ms) | Output weight (KiB) | Avg time per MiB (ms) |
|------|----------------|----------------------|-----------------------|
| Lanczos3 | 69.75 | 1806.8 | 39.53 |
| Gaussian | 68.81 | 1608.1 | 43.82 |
| Thumbnail | 34.27 | 1404.5 | 24.98 |
| Fill | 69.75 | 1557.9 | 45.85 |
| Rotate - Flip | 18.83 | 1404.5 | 13.73 |
| Convert Easy | 27.94 | 119.3 | 239.79 |
| Convert Medium | 16.25 | 1112.2 | 14.96 |
| Convert Hard | 250.43 | 59.3 | 4326.71 |
| Convert Extreme | 252.92 | 59.3 | 4369.83 |
| Insane | 349.73 | 69.0 | 5191.92 |
<!-- BENCH:END -->

Resizing has a strong effect on the other flags since it is the first
manipulation applied. Converting to a heavier compression scheme (Avif in
particular) costs the most time per MiB of output.

#### Looks per resizing Algorithm
| Algorithm |                                                                                    Size                                                                                       |
|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|None       | [1024x1024](https://github.com/SoapyDev/PictuRust/blob/main/Assets/Initial.png)                                                                                               |
|Lanczos3   | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/lanczos3_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/lanczos3_big.png)    |
|Gaussian   | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/gaussian_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/gaussian_big.png)    |
|CatmullRom | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/catmullrom_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/catmullrom_big.png)|
|Triangle   | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/triangle_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/triangle_big.png)    |
|Nearest    | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/nearest_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/nearest_big.png)      |
|Thumbnail  | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/thumbnail_small.png)                                                                                         |
|Fill       | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/fill_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/fill_big.png)            |

Multiple sizes were used to test the looks as some algorythm perform best under certain condition. For example Thumbnail is the quickest & the smallest file at 612x612, but cannot scale above the initial size of the image. Also nearest is usually the worst when resizing for a smaller form, but in the case of 2048x2048 the size is almost half of Lanczos3 with no visible difference.

## Future plan

### Features

- Controll over multithreading
- Cropping tool
- Verbose
- Allow usage of GPU

## Flags

|   Name            |      Long        |   Short   |   Defaults   |             Options                              |                                                     Descripton                                                    |
|-------------------|------------------|-----------|--------------|--------------------------------------------------|-------------------------------------------------------------------------------------------------------------------|
|Input              |--input_dir       |-i         |No default    |No options                                        |Directory, or file used as the source                                                                              |
|Output             |--outpub_dir      |-o         |No default    |No options                                        |Directory used as the target for the program                                                                       |
|Recursive          |--recursive       |-R         |False         |No options                                        |Allow the program to go into any subfolder to search for more picture                                              |
|Width              |--width           |-w         |Calculated    |No options                                        |The width in pixel to resize the image to. If none is given, it will keep the ratio                                |
|Height             |--height          |-H         |Calculated    |No options                                        |The height in pixel to resize the image to. If none is given, it will keep the ratio                               |
|Resizing           |--resize_type     |-t         |Exact         |Eact, Thumbnail, Fill                             |Define the method used to resize the picture                                                                       |
|Filter             |--filter          |-f         |Lanczos3      |Lanczos3, Gaussian, CatmullRom, Triangle, Nearest |The height in pixel to resize the picture to. If none is given, it will keep the ratio                             |
|Format             |--format          |-F         |None          |None, Jpeg, Png, Tiff, Webp, Avif                 |The format of the new picture. None set it to the actual format of the picture                                     |
|Rotation           |--rotation        |-r         |None          |None, 90, 180, 270                                |The rotation to be applied to the picture. None set it to the actual rotation                                      |
|Flip Horinzontally |--flip_horizontal |-s         |False         |No options                                        |If the picture should be flipped on the X axis                                                                     |
|Flip Vertically    |--flip_vertical   |-v         |False         |No options                                        |If the picture should be flipped on the Y axis                                                                     |
|Threads            |--threads         |-T         |Max available |1 to the number of threads available on this machine|The number of threads to use for processing                                                                     |
