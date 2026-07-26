use std::io::{stderr, stdout, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use refract::camera::{Camera, RenderSettings};
use refract::canvas::PixelSink;
use refract::canvas::write_ppm;
use refract::material::ReflectionType;
use refract::pixel_buffer::PixelBuffer;
use refract_scenes::Book1Scene;

fn spawn_progress_reporter(buffer: Arc<PixelBuffer>, done: Arc<AtomicBool>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut err = stderr();
        let mut last_percent: Option<u32> = None;

        loop {
            let percent = (buffer.progress() * 100.0).floor().clamp(0.0, 100.0) as u32;
            if last_percent != Some(percent) {
                let _ = write!(err, "\rRendering: {percent:3}%");
                let _ = err.flush();
                last_percent = Some(percent);
            }

            if done.load(Ordering::Relaxed) || buffer.is_complete() || buffer.is_cancelled() {
                break;
            }

            thread::sleep(Duration::from_millis(100));
        }

        let final_percent = if buffer.is_complete() { 100 } else { last_percent.unwrap_or(0) };
        let _ = write!(err, "\rRendering: {final_percent:3}%\n");
        let _ = err.flush();
    })
}

fn main() {
    let camera = Camera::new(RenderSettings {
        samples_per_pixel: 10,
        ..RenderSettings::default()
    });
    let buffer = Arc::new(PixelBuffer::new(camera.width as u32, camera.height as u32));
    let scene = Book1Scene::build(ReflectionType::Lambertian);

    let done = Arc::new(AtomicBool::new(false));
    let reporter = spawn_progress_reporter(Arc::clone(&buffer), Arc::clone(&done));

    camera.render(&scene, buffer.as_ref());
    done.store(true, Ordering::Relaxed);
    let _ = reporter.join();

    write_ppm(buffer.as_ref(), &mut stdout()).expect("failed to write PPM to stdout");
}
