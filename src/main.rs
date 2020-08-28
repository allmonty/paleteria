extern crate image;

#[derive(Copy, Clone)]
pub struct Vector <T> {
    x: T,
    y: T,
    z: T,
}

impl From<image::Rgb<u8>> for Vector<u8> {
    fn from(image::Rgb([a1, a2, a3]) : image::Rgb<u8>) -> Vector<u8> {
        Vector{x: a1, y: a2, z: a3}
    }
}

impl From<Vector<u8>> for Vector<u32> {
    fn from(Vector{x: a1, y: a2, z: a3} : Vector<u8>) -> Vector<u32> {
        Vector{x: a1.into(), y: a2.into(), z: a3.into()}
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error>{
        write!(f, "{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}

impl Vector<u8>{
    fn subtract_vector(&self, &Vector{x: b1, y: b2, z: b3} : &Vector<u8>) -> Vector<u8> {
        Vector{
            x: self.x.checked_sub(b1).unwrap_or(0),
            y: self.y.checked_sub(b2).unwrap_or(0),
            z: self.z.checked_sub(b3).unwrap_or(0),
        }
    }

    fn add_vector(&self, &Vector{x: b1, y: b2, z: b3} : &Vector<u8>) -> Vector<u8> {
        Vector{
            x: self.x.wrapping_add(b1),
            y: self.y.wrapping_add(b2),
            z: self.z.wrapping_add(b3),
        }
    }

    fn divide(&self, &factor : &u8) -> Vector<u8> {
        Vector{
            x: self.x.wrapping_div(factor),
            y: self.y.wrapping_div(factor),
            z: self.z.wrapping_div(factor),
        }
    }

    fn multiply(&self, &factor : &f32) -> Vector<u8> {
        Vector{
            x: ((self.x as f32) * factor) as u8,
            y: ((self.y as f32) * factor) as u8,
            z: ((self.z as f32) * factor) as u8,
        }
    }

    fn into_rgb(&self) -> image::Rgb<u8> {
        [self.x, self.y, self.z].into()
    }
}

fn length(&Vector{x: a1, y: a2, z: a3} : &Vector<u32>) -> u32 {
    ((a1.pow(2) + a2.pow(2) + a3.pow(2)) as f32).sqrt() as u32
}

fn distance(vector_a : &Vector<u8>, vector_b : &Vector<u8>) -> u32 {
    length(&vector_a.subtract_vector(&vector_b).into())
}

fn main() {
    // let img = image::open("resources/image.jpg").unwrap().to_luma();
    let img = image::open("resources/landscape.jpeg").unwrap();
    let img_color = img.to_rgb();
    let img_bw = img.grayscale().into_rgb();
    let mut new_img = image::ImageBuffer::new(img_color.dimensions().0, img_color.dimensions().1);
    
    let pallete_colors: Vec<Vector<u8>> = vec![
        Vector::<u8>{x: 56, y: 74, z: 8},
        Vector::<u8>{x: 100, y: 168, z: 151},
        Vector::<u8>{x: 228, y: 193, z: 85},
        Vector::<u8>{x: 208, y: 123, z: 78},
        Vector::<u8>{x: 203, y: 86, z: 83},

        // Vector::<u8>{x: 202, y: 124, z: 133},
        // Vector::<u8>{x: 25, y: 46, z: 80},
        // Vector::<u8>{x: 126, y: 76, z: 204},
    ];

    for (x, y, pixel) in img_color.enumerate_pixels() {
        let color: Vector<u8> = image::Pixel::to_rgb(pixel).into();
        let gray_color: Vector<u8> = (*img_bw.get_pixel(x, y)).into();

        let mut chosen_color : &Vector<u8> = pallete_colors.first().unwrap();
        for pallete_color in &pallete_colors {
            if distance(&color, &pallete_color) < distance(&color, &chosen_color) {
                chosen_color = &pallete_color
            }
        }

        let pointing_vector = chosen_color.subtract_vector(&color);
        let distance = distance(&chosen_color, &color);

        if distance < 230 {
            let factor: f32 = distance as f32/441.0;
            new_img.put_pixel(x, y, color.add_vector(&pointing_vector.multiply(&factor)).into_rgb());
        }
        else
        {
            new_img.put_pixel(x, y, gray_color.into_rgb());
        }
    }

    // Write the contents of this image to the Writer in PNG format.
    new_img.save("outputs/test44.jpg").unwrap();
}