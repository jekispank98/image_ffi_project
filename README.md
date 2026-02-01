# Image Plugin Processor

A Rust-based CLI application for dynamic image processing using external shared libraries (plugins). This architecture allows you to apply various filters and effects without recompiling the core application.

## Features

* **Dynamic Loading**: Loads plugins at runtime (`.dll` on Windows, `.so` on Linux/macOS).
* **Robust Error Handling**: Powered by `thiserror` to provide clear, human-readable error messages without panicking.
* **Safe Resource Management**: Ensures file handles and dynamic libraries are correctly released (RAII).
* **Flexible Configuration**: Pass custom parameters to plugins via external text files.

## Prerequisites

* **Rust** (1.70 or higher)
* **Cargo**
* A C-compatible compiler (if developing plugins in C/C++)

## Installation

1. Clone the repository:
   ```bash
   git clone [https://github.com/your-repo/image-plugin-processor.git](https://github.com/your-repo/image-plugin-processor.git)
   cd image-plugin-processor
2. Build the project:
   cargo build --release
3. Run the application by providing paths to the input image, the plugin, and the parameters file:
   cargo run -- \
  --input path/to/image.png \
  --output path/to/result.png \
  --plugin my_filter \
  --params path/to/params.txt \
  --plugin-path ./target/debug
   
## Arguments
--input: Path to the source image.
--output: Path where the processed image will be saved.
--plugin: The name of the plugin library (exclude the lib prefix and file extension).
--params: Path to a text file containing parameters to be passed to the plugin.
--plugin-path: (Optional) Directory to search for the plugin. Defaults to target/debug.
