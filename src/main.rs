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

            let ir = (255.9 * r) as i32;
            let ig = (255.9 * g) as i32;
            let ib = (255.9 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
