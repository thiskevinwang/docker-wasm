use image::{GenericImageView, Pixel};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
use std::str;
use std::time::Instant;
use wasm_bindgen::prelude::*;
use wasmedge_tensorflow_interface;

#[wasm_bindgen]
pub fn infer(image_data: &[u8]) -> Vec<u8> {
  // 1. Load the input image
  let mut img = image::load_from_memory(image_data).unwrap();

  // 2. Convert the image into a vector f32 values representing the RGB values for each pixel
  let mut flat_img: Vec<f32> = Vec::new();
  for (_x, _y, rgb) in img.pixels() {
    flat_img.push(rgb[2] as f32);
    flat_img.push(rgb[1] as f32);
    flat_img.push(rgb[0] as f32);
  }

  // 3. Load the frozen saved tensorflow model into a byte array. The model is trained to detect faces in the input image.
  let model_data: &[u8] = include_bytes!("mtcnn.pb");

  // 4. Execute the model with the image as input, and retrieves the model output
  // 4.1 The model type of a Tensorflow frozen model. SSVM could also handle Tensorflow Lite (TFLite) models.
  let mut session = wasmedge_tensorflow_interface::Session::new(
    model_data,
    wasmedge_tensorflow_interface::ModelType::TensorFlow,
  );
  // 4.2 Multiple input tensors for image data and parameters. Each input tensor has a name, data, and shape.
  // 4.3 Multiple output tensors for the results. Each output tensor has a name.
  session
    .add_input("min_size", &[20.0f32], &[])
    .add_input("thresholds", &[0.6f32, 0.7f32, 0.7f32], &[3])
    .add_input("factor", &[0.709f32], &[])
    .add_input(
      "input",
      &flat_img,
      &[img.height().into(), img.width().into(), 3],
    )
    .add_output("box")
    .add_output("prob")
    .run();
  // 4.4 Retrieve the data vector associated with the named output tensor.
  let res_vec: Vec<f32> = session.get_output("box");

  // 5. Create arrays of boxes for detected faces
  let mut box_vec: Vec<[f32; 4]> = Vec::new();
  // ... ...

  // 6. Create and return a new image with the boxes overlay on the detected faces
  let mut buf = Vec::new();
  img
    .write_to(&mut buf, image::ImageOutputFormat::Png)
    .expect("Unable to write");
  return buf;
}
