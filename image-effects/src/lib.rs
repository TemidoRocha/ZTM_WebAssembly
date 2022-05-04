use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{encode, decode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;


// Macro attributes
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
  // there are 2 types of strings:
  // if we are borrowing a string the type &str
  // if ew have the ownership the type should be String

  // type conversion can be done with into fn
  log(&"Grayscale called".into());

  let base64_to_vector = decode(encoded_file).unwrap();
  log(&"image decoded".into());

  let mut img = load_from_memory(&base64_to_vector).unwrap();
  log(&"image loaded".into());
  
  img = img.grayscale();
  log(&"grayscale effect applyed".into());
  
  let mut buffer = vec![];
  img.write_to(&mut buffer, Png).unwrap();
  log(&"New img writen".into());

  let encoded_img = encode(&buffer);
  let data_url = format!(
    "data:image/png;base64,{}",
    encoded_img
  );
  
  data_url
}