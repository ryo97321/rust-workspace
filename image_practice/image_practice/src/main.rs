use image::open;

fn main() {
    let img = open("src/sample_image.jpg").unwrap().to_rgb();

    let img_width = img.width();
    let img_height = img.height();

    for y in 0..img_height {
        for x in 0..img_width {
            let pixel = img.get_pixel(x, y);
            let r = pixel[0];
            let g = pixel[1];
            let b = pixel[2];
            println!("({}, {}, {})", r, g, b);
        }
    }
}
