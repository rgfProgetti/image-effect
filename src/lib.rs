use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _,engine::general_purpose};
use image::load_from_memory;
use image::ImageOutputFormat::Png;
use std::io::Cursor;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
   log(&"Grayscal called".into());

   let base64_to_vector = general_purpose::STANDARD
    .decode(encoded_file).unwrap();
   log(&"Image decoded".into());

   let mut img = load_from_memory(&base64_to_vector).unwrap();
   log(&"Image loaded".into());

   img = img.grayscale();
   log(&"Grayscale effect applied".into());

    let mut buffer  = Cursor::new(vec![]);
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New image written".into());

    let encoded_img = general_purpose::STANDARD.encode(&buffer.into_inner());
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    data_url

}
