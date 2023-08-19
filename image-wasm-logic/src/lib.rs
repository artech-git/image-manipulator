// use std::path::PathBuf;
// use bytes::Bytes;

use wasm_bindgen::prelude::*;

use image::imageops::{resize, rotate90, rotate180, rotate270};
use image::{open, DynamicImage, ImageBuffer};
// use image::imageops::colorops::grayscale;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name)); 
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct ImageOps {
    location: String, 
    image: image::DynamicImage, 

}

#[wasm_bindgen] 
impl ImageOps {

    pub fn new(location: String) -> Self {
        
        let image = open(location.clone()).unwrap();
        
        Self {
            location: location,
            image: image,
        }
    }

    pub fn scale(&self, new_width: u32, new_height: u32) -> Vec<u8> {

        // Open the input image
        let img = &self.image;

        // Rescale the image
        let scaled_img = resize(img, new_width, new_height, image::imageops::FilterType::Lanczos3);
        
        // Save the scaled image
        scaled_img.to_vec()    
    }
    
    // add more params for handling manipulation
    pub fn gray_scale(&self,) -> Vec<u8> {
        
        let img = &self.image; 

        let grayscale_img = image::imageops::grayscale(img);

        // return the byte 
        grayscale_img.to_vec()
    }

    pub fn rotate(&self, rotation: u32) -> Vec<u8> {
        let img = &self.image; 

        let rotated_img = match rotation {
            90 => {
                rotate90(img)
            },
            180 => {
                rotate180(img)
            },
            270 => {
                rotate270(img)
            }   
            _ => {
                rotate90(img)
            }
        };

        // return bytes
        rotated_img.to_vec()
    }

    // pass the filter type in the pararm 
    pub fn noise_reduction(&self) -> Vec<u8> {

        let img = &self.image; 

        // process the denoised image 
        let denoised_img = noise_reduction_filter(img);

        // return the byte
        denoised_img.to_vec()
    }

    pub fn histo_equilizer(&self, ) -> Vec<u8> {
        let img = &self.image; 

        let equalized_img = histogram_equalization(img);

        equalized_img.to_vec()
    }

}


fn noise_reduction_filter(image: &image::DynamicImage) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    
    //define custome kernel type 
    let kernel: [f32; 9] = [
        1., 2., 1.,
        2., 4., 2.,
        1., 2., 1.,
    ];

    // filter the image type 
    image::imageops::filter3x3(image, &kernel)
}

fn histogram_equalization(image:  &DynamicImage) -> ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    // Convert the image to grayscale
    let gray_image = image.to_luma8();

    // Compute the histogram of the grayscale image
    let mut histogram = [0u32; 256];
    for pixel in gray_image.pixels() {
        let intensity = pixel[0] as usize;
        histogram[intensity] += 1;
    }

    // Compute the cumulative histogram
    let mut cumulative_histogram = [0u32; 256];
    let mut sum = 0u32;
    for (intensity, cum_hist) in histogram.iter().zip(cumulative_histogram.iter_mut()) {
        sum += *intensity;
        *cum_hist = sum;
    }

    // Normalize the cumulative histogram
    let max_value = cumulative_histogram[255];
    let normalized_histogram: Vec<u8> = cumulative_histogram
        .iter()
        .map(|&cum_hist| ((cum_hist as f64 / max_value as f64) * 255.0) as u8)
        .collect();

    // Apply histogram equalization to the original image
    let equalized_image = image::ImageBuffer::from_fn(gray_image.width(), gray_image.height(), |x, y| {
        let intensity = gray_image.get_pixel(x, y)[0] as usize;
        let new_intensity = normalized_histogram[intensity];
        image::Rgba([new_intensity, new_intensity, new_intensity, 255])
    });

    equalized_image
}


#[cfg(test)]
mod tests {

    use crate::ImageOps;

    const IMG_LOC: &str = "./sample.jpg"; 

    #[test]
    fn test_image_scaling() {

        let img_ops = ImageOps::new( IMG_LOC.to_string());

        let scaled_bytes = img_ops.scale(800, 600);

        let res = std::fs::write("./sample-test", format!("{:?}", scaled_bytes)).unwrap();
    }



}