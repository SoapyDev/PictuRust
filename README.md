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

Write only:
- Webp
- Avif



## Looks & Performance

### Looks per resizing Algorithm

|    Algo   |                                                                                    Results                                                                                    |
|-----------|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
|None       | [1024x1024](https://github.com/SoapyDev/PictuRust/blob/main/Assets/Initial.png)                                                                                               |
|Lanczos3   | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/lanczos3_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/lanczos3_big.png)    |
|Gaussian   | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/gaussian_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/gaussian_big.png)    |
|CatmullRom | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/catmullrom_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/catmullrom_big.png)|
|Triangle   | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/triangle_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/triangle_big.png)    |
|Nearest    | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/nearest_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/nearest_big.png)      |
|Thumbnail  | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/thumbnail_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/thumbnail_big.png)  |
|Fill       | [612x612](https://github.com/SoapyDev/PictuRust/blob/main/Assets/fill_small.png), [2048x2048](https://github.com/SoapyDev/PictuRust/blob/main/Assets/fill_big.png)            |


### Weight per conversion format

|                                         Results                                                     | Weight (KiB)| Ratio |
|-----------------------------------------------------------------------------------------------------|-------------|-------|
| [PNG](https://github.com/SoapyDev/PictuRust/blob/main/Assets/Initial.png)                           |    1,357    |   1   |
| [JPEG](https://github.com/SoapyDev/PictuRust/blob/main/Assets/jpeg_format.jpeg)                     |      119    | 0.088 |
| [TIFF](https://github.com/SoapyDev/PictuRust/blob/main/Assets/tiff_format.tiff)                     |    3,146    | 4.269 |
| [WEBP lossless](https://github.com/SoapyDev/PictuRust/blob/main/Assets/webp_lossless.webp)          |      284    | 0.209 |
| [WEBP 70% quality](https://github.com/SoapyDev/PictuRust/blob/main/Assets/webp_loss.webp)           |       60    | 0.044 |
| [Avif 70% quality, speed 7](https://github.com/SoapyDev/PictuRust/blob/main/Assets/avif_quick.avif) |       55.6  | 0.040 |
| [Avif 70% quality, speed 3](https://github.com/SoapyDev/PictuRust/blob/main/Assets/avif_slow.avif)  |       53.2  | 0.039 |


### Performance

#### Software & Material 
The results will depend largely on the machine and the OS. For comparaison here are the specs used for this test suite.

OS: Fedora 38 - 6.4.14-200.fc38.x86_64

CPU: AMD Ryzen 9 5950x (32) @ 4.000GHz

Memory: 64Gb 3600

#### Samples
|        Name      | Number of folder | Number of picture | Picture per folder |    Width    |    Height    | Total Weight (MiB) |
|------------------|------------------|-------------------|--------------------|-------------|--------------|--------------------|
|Single picture    |         0        |         1         |            0       | 2160        | 2160         |         1.30       |
|Single folder     |         1        |      1000         |         1000       | 1080-15,280 | 720-3024     |     1,834.42       |
|Multiple folders  |        10        |      1000         |          100       | 1080-15,280 | 720-3024     |     1,834.42       |


#### Tests suite

|     Name      |                   Flags                   |
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


#### Results
|     Sample     | Lanczos3 (ms) | Gaussian (ms) | Thumbnail (ms) | Fill (ms) | Rotate - Flip (ms) | Convert Easy (ms) | Convert Medium (ms) | Convert Hard (ms) | Convert Extreme (ms) | Insane (ms) | 
|----------------|---------------|---------------|----------------|-----------|--------------------|-------------------|---------------------|-------------------|----------------------|-------------|
|Single picture  |      204      |      197      |       94       |    115    |         167        |        151        |         317         |       1,119       |         2,754        |    10,169   |
|Single folder   |   15,251      |   15,296      |    7,984       |  9,841    |      14,908        |     11,218        |      20,786         |     408,913       |       488,417        |   712,587   |
|Multiple folders|   15,148      |   14,894      |    7,717       |  9,631    |      14,636        |     10,886        |      20,448         |     408,342       |       487,578        |   709,588   |
|Avg Time per MiB|        8.33   |        8.27   |        4.30    |      5.33 |           8.09     |          6.06     |          11.32      |         222,98    |           266.67     |       390.27|


#### Compound effect
Resizing has a strong effect on other flags as its the first manipulation on any given request. Thus, resizing to a smaller size will result in faster times than keeping the original size. On the opposite side, resizing to a bigger size will cause other manipulation to take more time.

#### Space complexity
O(n^x)
Space complexity is directly correlated to requested size (x). Where n is max(image size, requested size) * number of thread and x is max(image size, requested size) / image size.  



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
