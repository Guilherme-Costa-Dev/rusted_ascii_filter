# rusted_ascii_filter
CLI-based image to ASCII generator written in Rust.

## What it does
You can think of it as a filter, as the final product is another image. It converts the original image to ASCII, then generates an image with the ASCII characters.

## Usage
Just call the binary, if you have it installed globally, with the name of the file
```
rusted_ascii_filter image.png
```
### Options
* -w, --width: The width of the output in ASCII characters (Default: 1280).
* -c, --contrast : Contrast adjustment applied before conversion (Default: 30.0).
* -f, --font-size : Font size used to render the final ASCII image (Default: 24.0).
### Example

```
rusted_ascii_filter ducati.jpg -w 1280 -f 12

```

<img width="7025" height="3948" alt="wp4373670-ducati-panigale-v4r-wallpapers_ascii" src="https://github.com/user-attachments/assets/b211f9d4-4eec-44c9-bdd6-9d3ffc2c1316" />

```
rusted_ascii_filter ducati.jpg -w 600 -c 50

```

<img width="6586" height="3696" alt="wp4373670-ducati-panigale-v4r-wallpapers_ascii" src="https://github.com/user-attachments/assets/4f7a7b74-a3d7-4a39-9b1b-85966d7be96e" />


Original image:
<img width="3840" height="2160" alt="wp4373670-ducati-panigale-v4r-wallpapers" src="https://github.com/user-attachments/assets/c0f4a6f8-2174-4d7e-af64-a56ea99eb54d" />

