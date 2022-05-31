use crate::util as byte_utils;
use std::fmt;

type Image = image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>;

pub struct Header {
  pub name_length: u32,
  pub data_length: u32,
}

pub struct FileData {
  pub name: String,
  pub data: Vec<u8>,
}

impl fmt::Debug for Header {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "Header {{ name_length: {}, data_length: {} }}",
      self.name_length, self.data_length
    )
  }
}

fn write_byte_vector_to_image(img: &mut Image, pixel_cursor: &mut u32, bytes: &Vec<u8>) {
  for byte in bytes {
    write_byte_to_image(img, pixel_cursor, &byte)
  }
}

// TODO take into account multiple rows of pixels
fn get_pixel_position(img: &Image, pixel_index: &u32) -> (u32, u32) {
  let y = ((pixel_index / img.width()) as f64).floor() as u32;
  let x = pixel_index % img.width();

  if x > img.width() || y > img.height() {
    panic!("Pixel index out of bounds");
  }

  (x, y)
}

// Write one byte (u8) to two pixels from a start pos
fn write_byte_to_image(img: &mut Image, pixel_cursor: &mut u32, byte: &u8) {
  let mut bits: Vec<u8> = Vec::new();

  for bit_index in 0..8 {
    let bit = (byte >> bit_index) & 1;
    bits.push(bit);
  }

  for i in (0..8).step_by(4) {
    let position = get_pixel_position(&img, pixel_cursor);
    let existing_pixel = img[position];

    let new_pixel = image::Rgba([
      byte_utils::byte_with_x_last_bit(&existing_pixel[0], bits[i]),
      byte_utils::byte_with_x_last_bit(&existing_pixel[1], bits[i + 1]),
      byte_utils::byte_with_x_last_bit(&existing_pixel[2], bits[i + 2]),
      byte_utils::byte_with_x_last_bit(&existing_pixel[3], bits[i + 3]),
    ]);

    *pixel_cursor += 1;

    img.put_pixel(position.0, position.1, new_pixel);
  }
}

fn read_byte_from_image(img: &Image, pixel_cursor: &mut u32) -> u8 {
  let mut byte: u8 = 0;

  for i in (0..8).step_by(4) {
    let position = get_pixel_position(&img, pixel_cursor);
    let existing_pixel = img[position];
    byte |= (byte_utils::get_last_bit_of_byte(&existing_pixel[0])) << i;
    byte |= (byte_utils::get_last_bit_of_byte(&existing_pixel[1])) << (i + 1);
    byte |= (byte_utils::get_last_bit_of_byte(&existing_pixel[2])) << (i + 2);
    byte |= (byte_utils::get_last_bit_of_byte(&existing_pixel[3])) << (i + 3);

    *pixel_cursor += 1;
  }

  byte
}

fn read_bytes_from_image(img: &Image, pixel_cursor: &mut u32, length: &u32) -> Vec<u8> {
  let mut bytes: Vec<u8> = Vec::new();

  for _i in 0..*length {
    let byte = read_byte_from_image(img, pixel_cursor);
    bytes.push(byte);
  }

  bytes
}

pub fn get_image_capacity(img: &Image) -> u32 {
  img.height() * img.width() - 1000 // Remove 1000 for the header
}

pub fn encode_data(img: &mut Image, data: &Vec<u8>, name: &Vec<u8>) {
  println!("Encoding image 🥷");
  let mut pixel_cursor: u32 = 0;

  write_header(img, &data, &name, &mut pixel_cursor);
  println!("Encoded Header ✅");

  write_byte_vector_to_image(img, &mut pixel_cursor, &name);
  write_byte_vector_to_image(img, &mut pixel_cursor, &data);
  println!("Encoded Data ✅");
}

pub fn decode_data(img: &Image) -> FileData {
  println!("Decoding image 🔎");
  let mut pixel_cursor: u32 = 0;

  let header = read_header(img, &mut pixel_cursor);
  println!("Decoded Header ✅");

  let file_name_bytes = read_bytes_from_image(img, &mut pixel_cursor, &header.name_length);
  let data_bytes = read_bytes_from_image(img, &mut pixel_cursor, &header.data_length);
  println!("Decoded Data ✅");

  FileData {
    name: byte_utils::construct_string_from_byte_vector(&file_name_bytes),
    data: data_bytes,
  }
}

fn write_header(img: &mut Image, data: &Vec<u8>, name: &Vec<u8>, pixel_cursor: &mut u32) {
  write_byte_to_image(img, pixel_cursor, &0);
  write_byte_vector_to_image(
    img,
    pixel_cursor,
    &byte_utils::convert_u32_to_bytes(name.len() as u32),
  ); // 4 bytes
  write_byte_vector_to_image(
    img,
    pixel_cursor,
    &byte_utils::convert_u32_to_bytes(data.len() as u32),
  ); // 4 bytes
  write_byte_vector_to_image(img, pixel_cursor, &vec![0; 16]); // 16 bytes
}

fn read_header(img: &Image, pixel_cursor: &mut u32) -> Header {
  let _flags = read_byte_from_image(img, pixel_cursor);
  let name_length_vec = read_bytes_from_image(img, pixel_cursor, &4);
  let data_length_vec = read_bytes_from_image(img, pixel_cursor, &4);
  let _salt = read_bytes_from_image(img, pixel_cursor, &16);

  Header {
    name_length: byte_utils::convert_byte_vector_to_u32(&name_length_vec),
    data_length: byte_utils::convert_byte_vector_to_u32(&data_length_vec),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert_u32_to_bytes() {
    let u32_val = 8123;
    let bytes = byte_utils::convert_u32_to_bytes(u32_val);

    let original_val = byte_utils::convert_byte_vector_to_u32(&bytes);
    assert_eq!(original_val, u32_val);
  }

  #[test]
  fn test_write_and_read_single_pixel() {
    let mut img = image::open(&"test-data/cat.jpeg")
      .expect("error reading image file")
      .to_rgba8();

    for byte in 0..255 {
      // Write a byte to the image
      let mut write_pixel_cursor: u32 = 0;
      write_byte_to_image(&mut img, &mut write_pixel_cursor, &byte);
      assert_eq!(write_pixel_cursor, 2);
      // Read the byte back
      let mut read_pixel_cursor: u32 = 0;
      let read_byte = read_byte_from_image(&img, &mut read_pixel_cursor);
      assert_eq!(read_pixel_cursor, 2);

      assert_eq!(byte, read_byte);
    }
  }

  #[test]
  fn test_get_pixel_position() {
    let img = image::open(&"test-data/cat.jpeg")
      .expect("error reading image file")
      .to_rgba8();

    assert_eq!(get_pixel_position(&img, &0), (0, 0));
    assert_eq!(get_pixel_position(&img, &10), (10, 0));
    assert_eq!(get_pixel_position(&img, &1000), (1000, 0));
    assert_eq!(get_pixel_position(&img, &10000), (1436, 4));
    assert_eq!(get_pixel_position(&img, &100000), (1514, 46));
  }

  #[test]
  fn test_e2e_encode_decode() {
    let img = image::open(&"test-data/cat.jpeg")
      .expect("error reading image file")
      .to_rgba8();

    let data = byte_utils::get_data_bytes_from_file("test-data/data.txt");
    let name = "data.txt".to_string();

    let mut img_copy = img.clone();
    encode_data(
      &mut img_copy,
      &data,
      &byte_utils::convert_string_to_bytes(&name),
    );

    let file_data = decode_data(&img_copy);

    assert_eq!(file_data.name, name);
    assert_eq!(file_data.data, data);
  }
}
