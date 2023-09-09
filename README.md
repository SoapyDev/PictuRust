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



## Performance & Looks

### Material 
The results will depend largely on the machine and the OS. For comparaison here are the specs used for this test suite.

OS: Fedora 38 - 6.4.14-200.fc38.x86_64

CPU: AMD Ryzen 9 5950x (32) @ 4.000GHz

Memory: 64Gb 3600

### Components

#### Samples
|      Sample      | Number of folder | Number of picture | Picture per folder | Width (px)  | Height (px)  | Total Weight (MiB) |
|------------------|------------------|-------------------|--------------------|-------------|--------------|--------------------|
|Single picture    |         0        |         1         |            0       | 2160        | 2160         |         1.30       |
|Single folder     |         1        |      1000         |         1000       | 1080-15,280 | 720-3024     |     1,834.42       |
|Multiple folders  |        10        |      1000         |          100       | 1080-15,280 | 720-3024     |     1,834.42       |


#### Test suite
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
#### Comparaison of time processing per Test [ms]
|     Sample     | Lanczos3      | Gaussian      | Thumbnail      | Fill      | Rotate - Flip      | Convert Easy      | Convert Medium      | Convert Hard      | Convert Extreme      | Insane      | 
|----------------|---------------|---------------|----------------|-----------|--------------------|-------------------|---------------------|-------------------|----------------------|-------------|
|Single picture  |      204      |      197      |       94       |    115    |         167        |        151        |         317         |       1,119       |         2,754        |    10,169   |
|Single folder   |   15,251      |   15,296      |    7,984       |  9,841    |      14,908        |     11,218        |      20,786         |     408,913       |       488,417        |   712,587   |
|Multiple folders|   15,148      |   14,894      |    7,717       |  9,631    |      14,636        |     10,886        |      20,448         |     408,342       |       487,578        |   709,588   |
|Avg Time per MiB|        8.33   |        8.27   |        4.30    |      5.33 |           8.09     |          6.06     |          11.32      |         222.98    |           266.67     |       390.27|


Resizing has a strong effect on other flags as its the first manipulation on any given request. Thus, resizing to a smaller size will result in faster times than keeping the original size. On the opposite side, resizing to a bigger size will cause other manipulation to take more time. 
Recursivity (multiple folders) perform better across the board as it's splitting the path finding job accross multiple threads.

Space complexity is O(n^x). Where n is the number of simultaneously treated image and x is the max(image area, requested area) / image area. 


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

#### Weight per formatting
|                                           Formatting                                                | Weight (KiB)| Ratio |
|-----------------------------------------------------------------------------------------------------|-------------|-------|
| [PNG](https://github.com/SoapyDev/PictuRust/blob/main/Assets/Initial.png)                           |    1,357    |   1   |
| [JPEG](https://github.com/SoapyDev/PictuRust/blob/main/Assets/jpeg_format.jpeg)                     |      119    | 0.088 |
| [TIFF](https://github.com/SoapyDev/PictuRust/blob/main/Assets/tiff_format.tiff)                     |    3,146    | 4.269 |
| [WEBP lossless](https://github.com/SoapyDev/PictuRust/blob/main/Assets/webp_lossless.webp)          |      284    | 0.209 |
| [WEBP 70% quality](https://github.com/SoapyDev/PictuRust/blob/main/Assets/webp_loss.webp)           |       60    | 0.044 |
| [Avif 70% quality, speed 7](https://github.com/SoapyDev/PictuRust/blob/main/Assets/avif_quick.avif) |       55.6  | 0.040 |
| [Avif 70% quality, speed 3](https://github.com/SoapyDev/PictuRust/blob/main/Assets/avif_slow.avif)  |       53.2  | 0.039 |

Most gain can be achieved by converting to webp or avif. Playing with the speed does improve the compression, but only by a small margin.


## Future plan
### Support to be added 

- Webp (Read)
- Avif (Read)

### Features

- Controll over multithreading
- Cropping tool
- Verbose

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
|Print              |--print           |-p         |False         |Sixel, Kitty, Iterm, Blocks                       |Indicate to the program if it should display the result image in the terminal when its saved                       |
