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

fn mult_vector(image::Rgb([a1, a2, a3]) : image::Rgb<u8>, factor : f32) -> image::Rgb<u8> {
    let na1 = (a1 as f32 * factor) as u8;
    let na2 = (a2 as f32 * factor) as u8;
    let na3 = (a3 as f32 * factor) as u8;

    // println!("{} - {} - {}", na1, na2, na3);

    [na1, na2, na3].into()
}

fn array_to_u32_array(image::Rgb([a1, a2, a3]) : image::Rgb<u8>) -> [u32; 3] {
    [a1.into(), a2.into(), a3.into()]
}

fn length([a1, a2, a3] : [u32; 3]) -> u32 {
    ((a1.pow(2) + a2.pow(2) + a3.pow(2)) as f32).sqrt() as u32
}

fn main() {
    // let img = image::open("resources/image.jpg").unwrap().to_luma();
    let img = image::open("resources/landscape.jpeg").unwrap();
    let img_color = img.to_rgb();
    let img_bw = img.grayscale().into_rgb();
    
    let pallete_colors: Vec<image::Rgb<u8>> = vec![
        // [56, 74, 8].into(),
        // [100, 168, 151].into(),
        // [228, 193, 85].into(),
        // [208, 123, 78].into(),
        // [203, 86, 83].into()

        [202, 124, 133].into(),
        [25, 46, 80].into(),
        [126, 76, 204].into(),
    ];

    let mut new_img = image::ImageBuffer::new(img_color.dimensions().0, img_color.dimensions().1);

    for (x, y, pixel) in img_color.enumerate_pixels() {
        let color = image::Pixel::to_rgb(pixel);
        let gray_color = img_bw.get_pixel(x, y);

        let chosen_color : image::Rgb<u8> = pallete_colors.iter().fold(*pallete_colors.first().unwrap(), |x, y|{
            let dist_x = length(array_to_u32_array(subtract_vector(color, x)));
            let dist_y = length(array_to_u32_array(subtract_vector(color, *y)));

            // println!("distx {} disty {}", dist_x, dist_y);

            if dist_x <= dist_y { x } else { *y }
        });


        let pointing_vector = subtract_vector(chosen_color, color);

        let distance = length(array_to_u32_array(pointing_vector));

        // let addition_color = mult_vector(pointing_vector, factor);

        if distance < 230 {
            let factor: f32 = distance as f32/441.0;
            // new_img.put_pixel(x, y, add_vector(color, mult_vector(pointing_vector, factor)));
            let np = image::Pixel::blend(&mut color, &chosen_color);
            new_img.put_pixel(x, y, np);
        }
        else
        {
            new_img.put_pixel(x, y, *gray_color);
        }
    }

    // Write the contents of this image to the Writer in PNG format.
    new_img.save("outputs/test43.jpg").unwrap();
}