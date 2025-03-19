# zipwin

zipwin is a CLI tool to create ZIP file that can be extracted on Windows without encoding issues.

## Installation

:construction: **This project is under development.** :construction:

Currently, you need to build from source:

```sh
# After clone this repository and change directory
cargo build --release
cd target/release
chmod +x zipwin
mv zipwin /usr/local/bin/
```

## Usage

### Basic usage

To create a ZIP file from a directory:

```sh
zipwin <directory>
```

By default, the output file will be named `<directory_name>.zip`.

### Specify an output file name

```sh
zipwin <target_directory> <output_file.zip>
```
