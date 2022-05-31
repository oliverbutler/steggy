use crate::pixels as pixel_utils;
use crate::util as byte_utils;
use clap::{Parser, Subcommand};
use std::fs;
use std::io::prelude::*;

mod pixels;
mod util;

/// A steganography tool written in Rust
#[derive(Debug, Parser)]
#[clap(name = "steg")]
#[clap(about = "A steganography tool written in Rust", long_about = None)]
struct Cli {
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Encodes Data
  #[clap(arg_required_else_help = true)]
  Encode {
    /// Image to hide data within
    #[clap(short, long)]
    image: String,

    /// Data to hide
    #[clap(short, long)]
    file: String,

    /// Output file
    #[clap(short, long)]
    output: String,
  },
  /// Decodes Data
  #[clap(arg_required_else_help = true)]
  Decode {
    /// Image to hide data within
    #[clap(short, long)]
    image: String,

    /// Output file, or use original file name
    #[clap(short, long)]
    output: Option<String>,
  },
}

fn main() {
  let args = Cli::parse();

  match args.command {
    Commands::Encode {
      image,
      file,
      output,
    } => encode(&image, &file, &output),
    Commands::Decode { image, output } => decode(&image, output),
  }
}

fn encode(image_path: &String, data_path: &String, output_path: &String) {
  let mut img = image::open(&image_path)
    .expect("Error reading image file")
    .to_rgba8();

  let data = byte_utils::get_data_bytes_from_file(&data_path);
  let percent_used = ((data.len() as f64) / (pixel_utils::get_image_capacity(&img) as f64)) * 100.0;

  if percent_used > 99.9 {
    println!("Image is too small to fit the data");
    return;
  }

  println!(
    "Space used in image: {:.1}% Data Size: {:.1}MB",
    percent_used,
    (data.len() as f64) / (1024.0 * 1024.0)
  );

  let file_name_without_initial_slashes = String::from(data_path.split("/").last().unwrap());

  pixel_utils::encode_data(
    &mut img,
    &data,
    &byte_utils::convert_string_to_bytes(&file_name_without_initial_slashes),
  );

  img.save(output_path).expect("Error saving image");
}

fn decode(image_path: &String, output_path: Option<String>) {
  let img = image::open(&image_path)
    .expect("Error reading image file")
    .to_rgba8();
  let file_data = pixel_utils::decode_data(&img);

  let file_name = match output_path {
    Some(output) => output,
    None => file_data.name,
  };

  let mut file = fs::File::create(file_name).expect("Error creating file");
  file.write_all(&file_data.data).expect("Error writing file");
}
