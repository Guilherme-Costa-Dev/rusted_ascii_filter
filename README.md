# rusted_ascii_filter
CLI-based image to ASCII generator written in Rust.

## What it does
You can think of it as a filter, as the final product is another image. It converts the original image to ASCII, then generates an image with the ASCII characters.

## Building and installing from source
**Prerequisites:** [Rust and Cargo installed](https://rustup.rs/) on your system.
```
git clone https://github.com/Guilherme-Costa-Dev/rusted_ascii_filter.git
cd rusted_ascii_filter
cargo install --path .
```

## Usage
Just call the binary, if you have it installed globally, with the name of the file
```
rusted_ascii_filter image.png
```

### Options
* -w, --width: The width of the output in ASCII characters (Default: 1280).
* -c, --contrast : Contrast adjustment applied before conversion (Default: 10.0).
* -f, --font-size : Font size used to render the final ASCII image (Default: 24.0).

### Example

```
rusted_ascii_filter ducati.jpg -w 1000 -f 12

```

<img width="5488" height="3084" alt="ducati_ascii" src="https://github.com/user-attachments/assets/7d5d89e1-263a-4b99-93bc-0289992c2777" />

```
rusted_ascii_filter ducati.jpg -w 600

```

<img width="6586" height="3696" alt="ducati_ascii" src="https://github.com/user-attachments/assets/90671f52-c7e6-4bc6-b099-e3494bf91a48" />

Original image:
<img width="3840" height="2160" alt="wp4373670-ducati-panigale-v4r-wallpapers" src="https://github.com/user-attachments/assets/c0f4a6f8-2174-4d7e-af64-a56ea99eb54d" />

