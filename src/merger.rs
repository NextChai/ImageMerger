use image::Pixel;
use num_traits::Zero;

use crate::image::Image;

/// NOTE FOR FUTURE: Canvas dimensions will dynamically resize based on if you hit an index that is out of bounds. This means a new canvas will only
/// contain space for num_images_per_row and resize later. To combat this, the canvas can be resized according to the expected number of images. This is a memory
/// management feature

pub struct Merger<P: Pixel> {
    canvas: image::ImageBuffer<P, Vec<P::Subpixel>>, // The canvas that gets written to.
    image_dimensions: (u32, u32), // The dimensions of the images being pasted (images must be a uniform size)
    num_images: u32,              // The number of images that have been pasted to the canvas
    num_images_per_row: u32,      // The number of pages per row.
    last_pasted_index: u32, // The index of the last pasted image, starts at -1 if not images have been pasted.
    total_rows: u32,        // The total number of rows currently on the canvas.
}

impl<P: Pixel> Merger<P> {
    pub fn pasted_images_len(&self) -> u32 {
        self.num_images
    }

    // fn paste<T, F>(&mut self, to: &mut T, from: &F, x: u32, y: u32) -> ()
    // where
    //     T: image::GenericImage<Pixel = P>,
    //     F: image::GenericImageView<Pixel = P>,
    // {
    //     // TODO: Implement faster algorithm for pasting
    //     image::imageops::overlay(to, from, x.into(), y.into())
    // }

    fn grow_canvas(&mut self) -> () {
        self.total_rows += 1;

        let new_canvas_dimensions = (self.canvas.width(), self.canvas.height() * self.total_rows);

        // Create a new container with the capacity of the new canvas
        let mut new_container: Vec<P::Subpixel> = Vec::with_capacity(
            (<P as Pixel>::CHANNEL_COUNT as usize)
                * (new_canvas_dimensions.0 * new_canvas_dimensions.1) as usize,
        );

        // Push the old container contents into the new one and fill the rest with zeroes
        // Unfortunatley we must hold two containers in memory at once.
        // TODO: Look into a way to do this without holding two containers in memory at once.
        self.canvas.as_raw().iter().for_each(|pixel| {
            new_container.push(*pixel);
        });
        new_container.resize_with(new_container.capacity(), Zero::zero);

        let canvas: image::ImageBuffer<P, Vec<P::Subpixel>> = image::ImageBuffer::from_raw(
            new_canvas_dimensions.0,
            new_canvas_dimensions.1,
            new_container,
        )
        .unwrap();

        self.canvas = canvas;
    }

    fn get_next_paste_coordinates(&mut self) -> (u32, u32) {
        let available_images = (self.num_images_per_row * self.total_rows) - self.num_images;
        if available_images == 0 {
            // Resize the canvas to make room for the next row, we are out of space.
            self.grow_canvas();
        }

        // Calculate the next paste coordinates.
        let current_paste_index = self.last_pasted_index + 1;
        let x = current_paste_index % self.num_images_per_row;
        let y = current_paste_index / self.num_images_per_row;

        return (x, y);
    }

    /// Allows the merger to push an image to the canvas. This can be used in a loop to paste a large number of images without
    /// having to hold all them in memory.
    pub fn push<U: image::GenericImage<Pixel = P>>(&mut self, image: &Image<P, U>) -> () {
        let (x, y) = self.get_next_paste_coordinates();
        image::imageops::overlay(&mut self.canvas, image.get_underlying(), x as i64, y as i64);

        self.last_pasted_index += 1;
        self.num_images += 1;
    }

    /// Allows the merger to bulk push N images to the canvas. This is useful for when you have a large number of images to paste.
    /// The downside is that you have to hold all of the images in memory at once, which can be a problem if you have a large number of images.
    pub fn bulk_push<U: image::GenericImage<Pixel = P>>(&mut self, images: Vec<Image<P, U>>) {
        todo!()
    }

    /// Removes an image from the canvas at a given index. Indexing starts at 0 and works left to right, top to bottom.
    pub fn remove_image(&mut self, index: u32) {
        todo!()
    }
}
