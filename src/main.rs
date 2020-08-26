extern crate image;

fn subtract_vector(image::Rgb([a1, a2, a3]) : image::Rgb<u8>, image::Rgb([b1, b2, b3]) : image::Rgb<u8>) -> image::Rgb<u8> {
    [a1.wrapping_sub(b1), a2.wrapping_sub(b2), a3.wrapping_sub(b3)].into()
}

fn add_vector(image::Rgb([a1, a2, a3]) : image::Rgb<u8>, image::Rgb([b1, b2, b3]) : image::Rgb<u8>) -> image::Rgb<u8> {
    [a1.wrapping_add(b1), a2.wrapping_add(b2), a3.wrapping_add(b3)].into()
}

fn divide_vector(image::Rgb([a1, a2, a3]) : image::Rgb<u8>, factor : u8) -> image::Rgb<u8> {
    [a1.wrapping_div(factor), a2.wrapping_div(factor), a3.wrapping_div(factor)].into()
}

fn mult_vector(image::Rgb([a1, a2, a3]) : image::Rgb<u8>, factor : u8) -> image::Rgb<u8> {
    [a1.wrapping_mul(factor), a2.wrapping_mul(factor), a3.wrapping_mul(factor)].into()
}

fn array_to_u32_array(image::Rgb([a1, a2, a3]) : image::Rgb<u8>) -> [u32; 3] {
    [a1.into(), a2.into(), a3.into()]
}

fn sum_axes_of_vector([a1, a2, a3] : [u32; 3]) -> u32 {
    a1 + a2 + a3
}

fn main() {
    let img = image::open("resources/image.jpg").unwrap().to_luma();
    // let img = image::open("resources/image.jpg").unwrap().to_rgb();
    
    let pallete_colors: Vec<image::Rgb<u8>> = vec![
        [56, 74, 8].into(),
        [100, 168, 151].into(),
        [228, 193, 85].into(),
        [208, 123, 78].into(),
        [203, 86, 83].into()
    ];

    let mut new_img = image::ImageBuffer::new(img.dimensions().0, img.dimensions().1);

    for (x, y, pixel) in img.enumerate_pixels() {
        let pixel_color = image::Pixel::to_rgb(pixel);

        let chosen_color : image::Rgb<u8> = pallete_colors.iter().fold(*pallete_colors.first().unwrap(), |x, y|{
            let dist_x = sum_axes_of_vector(array_to_u32_array(subtract_vector(pixel_color, x)));
            let dist_y = sum_axes_of_vector(array_to_u32_array(subtract_vector(pixel_color, *y)));

            if dist_x > dist_y { x } else { *y }
        });

        let gray = mult_vector(divide_vector(pixel_color, 10), 2);
        let color = mult_vector(divide_vector(subtract_vector(pixel_color, chosen_color), 10), 3);
        // let color = mult_vector(divide_vector(chosen_color, 10), 3);

        // se perto, então mantém
        // se longe, move pra perto mas só um pouco
        // se muito longe, gray

        new_img.put_pixel(x, y, add_vector(color, gray));
    }

    // Write the contents of this image to the Writer in PNG format.
    new_img.save("test31.jpg").unwrap();
}