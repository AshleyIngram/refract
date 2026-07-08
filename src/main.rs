use std::io::stdout;

pub mod color;
pub mod vector3;

fn main() {
    let height = 256;
    let width = 256;

    println!("P3\n{} {}\n255", width, height);

    for i in 0..height {
        for j in 0..width {
            let r = i as f32 / (width - 1) as f32;
            let g = j as f32 / (height - 1) as f32;
            let b = 0.0;


            let color = color::Color::new(r, g, b);
            color.write_ppm(&mut stdout()).unwrap();
        }
    }
}
