use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::decode;
// Macro attributes
#[wasm_bindgen]
pub fn grayscale(encoded_file: &str){
  // type conversion can be done with into fn
  log(&"Grayscale called".into());

  let base64_to_vector = decode(encoded_file).unwrap();
  log(&"image decoded".into());
}