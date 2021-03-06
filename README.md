![BARCODERS](/media/logo.jpg?raw=true "BARCODERS")

**Barcoders** is a barcode-encoding library for the Rust programming language.

Barcoders allows you to encode valid data for a chosen symbology into a ```Vec<u8>``` representation of the underlying binary structure. From here, you can take advantage one of builtin generators (for exporting to GIF, PNG, etc) or build your own.

**Please note, Barcoders is under active development. Initial release is expected late November, 2015**

## Currently Supported

The ultimate goal of Barcoders is to provide encoding support for all major (and many not-so-major) symbologies.

### Symbologies

* EAN-13
  * UPC-A
  * JAN
  * Bookland
* EAN-8
* EAN Supplementals
  * EAN-2
  * EAN-5
* Code39
* More coming!

### Generators

* ASCII
* PNG
* GIF
* More coming! (PostScript, SVG, etc)

## Examples

### ASCII generation
```rust
extern crate barcoders;

use barcoders::sym::ean13::*;
use barcoders::generators::ascii::*;

let barcode = EAN13::new("750103131130".to_string()).unwrap();
let encoded: Vec<u8> = ean13.encode();
let ascii = ASCII::new().generate(&encoded);

assert_eq!(ascii.unwrap(),
"
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
# # ##   # #  ###  ##  # #  ### #### # ##  ## # # #    # ##  ## ##  ## #    # ###  # ### #  # #
".trim().to_string());
```

### Image generation
```rust
extern crate barcoders;

use barcoders::sym::code39::*;
use barcoders::generators::image::*;
use std::fs::File;
use std::path::Path;

let barcode = Code39::new("1ISTHELONELIESTNUMBER".to_string()).unwrap();
let png = Image::PNG{height: 80, xdim: 1};

// The `encode` method returns a Vec<u8> of the binary representation of the
// generated barcode. This is useful if you want to add your own generator.
let encoded: Vec<u8> = barcode.encode();

// Image generators save the file to the given path and return a u32 indicating
// the number of bytes written to disk.
let mut path = File::create(&Path::new("my_barcode.png")).unwrap();
let bytes = png.generate(&encoded, &mut path).unwrap();
```

![Code 39: 1ISTHELONELIESTNUMBER](/media/code39_1istheloneliestnumber.png?raw=true "Code 39: 1ISTHELONELIESTNUMBER")

## Tests

Note, some of the tests (intentionally) leave behind image files in ./target/debug that should be visually
inspected for correctness.

```
$ cargo test
```
