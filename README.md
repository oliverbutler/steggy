# Steg CLI Tool

Written in Rust, features a simple cli and a client-side webapp. This tool hides data within the least significant bit of an image. Obfuscation techniques are utilized to make the

## Install

```bash
brew tap oliverbutler/steg
brew install steg
```

## Usage

### Encode

```bash
steg encode -f secret.txt -i image.jpg -o out.png
```

### Decode

Decode will output the image inside an encoded image in the same path, optionally allows an output path for the resulting data.

```bash
steg decode -i out.png
```

# Image Data Structure

This is the structure of a `steg` encoded image file.

| Pos  | Length   | Field          |
| ---- | -------- | -------------- |
| 0    | 1 byte   | Flags (unused) |
| 1    | 4 bytes  | Name length    |
| 2    | 4 bytes  | Data length    |
| 6    | 16 bytes | Salt           |
| 22   | X        | Name           |
| 22+x | Y        | Data           |
