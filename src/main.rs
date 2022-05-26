use std::env;
use std::fmt;

type Image = image::ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>;

// Pos  Length     Field
// 0    1 byte   : Flags (unused)
// 1    4 byte   : Name length
// 2    4 byte   : Length of data in bytes
// 6    16 bytes : Salt for encryption  (unused)
// 22   X bytes  : Name
// X+22 Y bytes  : Data

struct Header {
  name_length: u32,
  data_length: u32,
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

// Byte cursor is incremented for every byte written to the file

fn write_byte_vector_to_image(img: &mut Image, pixel_cursor: &mut u32, bytes: &Vec<u8>) {
  for byte in bytes {
    write_byte_to_image(img, pixel_cursor, &byte)
  }
}

// TODO take into account multiple rows of pixels
fn get_pixel_position(_img: &Image, start_pos: &u32) -> (u32, u32) {
  let x = *start_pos;
  let y = 0 as u32;

  (x, y)
}

fn byte_with_x_last_bit(byte: &u8, x: u8) -> u8 {
  if x == 1 {
    byte | 1
  } else {
    byte & !1
  }
}

fn get_last_bit_of_byte(byte: &u8) -> u8 {
  byte & 1
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
      byte_with_x_last_bit(&existing_pixel[0], bits[i]),
      byte_with_x_last_bit(&existing_pixel[1], bits[i + 1]),
      byte_with_x_last_bit(&existing_pixel[2], bits[i + 2]),
      byte_with_x_last_bit(&existing_pixel[3], bits[i + 3]),
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
    byte |= (get_last_bit_of_byte(&existing_pixel[0])) << i;
    byte |= (get_last_bit_of_byte(&existing_pixel[1])) << (i + 1);
    byte |= (get_last_bit_of_byte(&existing_pixel[2])) << (i + 2);
    byte |= (get_last_bit_of_byte(&existing_pixel[3])) << (i + 3);

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

fn construct_string_from_byte_vector(bytes: &Vec<u8>) -> String {
  String::from_utf8(bytes.to_vec()).unwrap()
}

fn convert_string_to_bytes(s: &String) -> Vec<u8> {
  s.clone().into_bytes()
}

pub fn convert_u32_to_bytes(x: u32) -> Vec<u8> {
  x.to_be_bytes().to_vec()
}

fn convert_byte_vector_to_u32(bytes: &Vec<u8>) -> u32 {
  ((bytes[0] as u32) << 24)
    | ((bytes[1] as u32) << 16)
    | ((bytes[2] as u32) << 8)
    | (bytes[3] as u32)
}

fn write_header(img: &mut Image, data: &Vec<u8>, name: &Vec<u8>, pixel_cursor: &mut u32) {
  write_byte_to_image(img, pixel_cursor, &0);
  write_byte_vector_to_image(img, pixel_cursor, &convert_u32_to_bytes(name.len() as u32)); // 4 bytes
  write_byte_vector_to_image(img, pixel_cursor, &convert_u32_to_bytes(data.len() as u32)); // 4 bytes
  write_byte_vector_to_image(img, pixel_cursor, &vec![0; 16]); // 16 bytes
}

fn read_header(img: &Image, pixel_cursor: &mut u32) -> Header {
  let _flags = read_byte_from_image(img, pixel_cursor);
  let name_length_vec = read_bytes_from_image(img, pixel_cursor, &4);
  let data_length_vec = read_bytes_from_image(img, pixel_cursor, &4);
  let _salt = read_bytes_from_image(img, pixel_cursor, &16);

  Header {
    name_length: convert_byte_vector_to_u32(&name_length_vec),
    data_length: convert_byte_vector_to_u32(&data_length_vec),
  }
}

fn encode(img: &mut Image, data: &String, file_name: &String) {
  let mut pixel_cursor: u32 = 0;

  let data_bytes = convert_string_to_bytes(data);
  let name_bytes = convert_string_to_bytes(file_name);

  write_header(img, &data_bytes, &name_bytes, &mut pixel_cursor);

  write_byte_vector_to_image(img, &mut pixel_cursor, &name_bytes);
  write_byte_vector_to_image(img, &mut pixel_cursor, &data_bytes);
}

fn decode(img: &Image) {
  let mut pixel_cursor: u32 = 0;

  let header = read_header(img, &mut pixel_cursor);

  let file_name_bytes = read_bytes_from_image(img, &mut pixel_cursor, &header.name_length);
  let file_name = construct_string_from_byte_vector(&file_name_bytes);

  let data_bytes = read_bytes_from_image(img, &mut pixel_cursor, &header.data_length);
  let data = construct_string_from_byte_vector(&data_bytes);

  println!("{}", data);
  println!("{}", file_name);
}

fn main() {
  let args: Vec<String> = env::args().collect();

  let image_file = &args[1];
  let target_file = &args[2];

  println!("Image F***");

  let mut img = image::open(&image_file)
    .expect("error reading image file")
    .to_rgba8();

  let data = String::from("Hello World!");

  encode(&mut img, &data, &target_file);

  img.save("out.png").expect("error saving image");

  let out_img = image::open("out.png")
    .expect("error reading image file")
    .to_rgba8();

  decode(&out_img);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_convert_u32_to_bytes() {
    let u32_val = 8123;
    let bytes = convert_u32_to_bytes(u32_val);

    let original_val = convert_byte_vector_to_u32(&bytes);
    assert_eq!(original_val, u32_val);
  }

  #[test]
  fn test_write_and_read_single_pixel() {
    let mut img = image::open(&"test-cat.jpeg")
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
}
