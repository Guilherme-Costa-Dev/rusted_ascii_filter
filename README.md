# rusted_ascii_filter
CLI-based image to ASCII generator written in Rust.

## What it does
You can think of it as a filter, as the final product is another image. It converts the original image to ASCII, then generates an image with the ASCII characters.

## Building it from source
To compile it, you must have a file `font.ttf` in the `/src` directory. It's recommended to use a monospaced font.

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
rusted_ascii_filter image.jpg -w 800 -c 40.0 -f 18.0
```
