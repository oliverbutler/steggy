use std::fs;
use std::io::prelude::*;

pub fn byte_with_x_last_bit(byte: &u8, x: u8) -> u8 {
  if x == 1 {
    byte | 1
  } else {
    byte & !1
  }
}

pub fn get_last_bit_of_byte(byte: &u8) -> u8 {
  byte & 1
}

pub fn construct_string_from_byte_vector(bytes: &Vec<u8>) -> String {
  String::from_utf8(bytes.to_vec()).unwrap()
}

pub fn convert_string_to_bytes(s: &String) -> Vec<u8> {
  s.clone().into_bytes()
}

pub fn convert_u32_to_bytes(x: u32) -> Vec<u8> {
  x.to_be_bytes().to_vec()
}

pub fn convert_byte_vector_to_u32(bytes: &Vec<u8>) -> u32 {
  ((bytes[0] as u32) << 24)
    | ((bytes[1] as u32) << 16)
    | ((bytes[2] as u32) << 8)
    | (bytes[3] as u32)
}

pub fn get_data_bytes_from_file(file_path: &str) -> Vec<u8> {
  let mut file = fs::File::open(file_path).unwrap();
  let mut data = Vec::new();
  file.read_to_end(&mut data).unwrap();
  data
}
