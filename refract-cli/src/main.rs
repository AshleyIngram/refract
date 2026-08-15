use std::env;
use std::io::{Write, stderr, stdout};
use std::process;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

use refract::camera::Camera;
use refract::canvas::PixelSink;
use refract::canvas::write_ppm;
use refract::material::ReflectionType;
use refract::pixel_buffer::PixelBuffer;
use refract_scenes::SceneKind;

fn spawn_progress_reporter(
    buffer: Arc<PixelBuffer>,
    done: Arc<AtomicBool>,
) -> thread::JoinHandle<()> {
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

        let final_percent = if buffer.is_complete() {
            100
        } else {
            last_percent.unwrap_or(0)
        };
        let _ = write!(err, "\rRendering: {final_percent:3}%\n");
        let _ = err.flush();
    })
}

fn scene_names_for_help() -> String {
    SceneKind::all()
        .map(|scene| match scene {
            SceneKind::Book1 => "book1|book-1".to_string(),
            scene => format!("{scene:?}").to_lowercase(),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn print_usage(program: &str) {
    let scenes = scene_names_for_help();
    eprintln!("Usage: {program} [--scene <{scenes}>]");
    eprintln!();
    eprintln!("Options:");
    eprintln!("  --scene, -s   Scene to render (default: demo)");
    eprintln!("  --help,  -h   Show this help message");
}

fn parse_args() -> SceneKind {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "refract-cli".to_string());
    let scenes = scene_names_for_help();

    let mut scene = SceneKind::default();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--scene" | "-s" => {
                let value = args.next().unwrap_or_else(|| {
                    eprintln!("error: --scene requires a value ({scenes})");
                    process::exit(1);
                });
                scene = SceneKind::parse(&value).unwrap_or_else(|| {
                    eprintln!("error: unknown scene '{value}' (expected {scenes})");
                    process::exit(1);
                });
            }
            "--help" | "-h" => {
                print_usage(&program);
                process::exit(0);
            }
            other => {
                eprintln!("error: unknown argument '{other}'");
                print_usage(&program);
                process::exit(1);
            }
        }
    }

    scene
}

fn main() {
    let scene_kind = parse_args();
    let mut settings = scene_kind.default_render_settings();
    settings.samples_per_pixel = 10;

    let camera = Camera::new(settings);
    let buffer = Arc::new(PixelBuffer::new(camera.width as u32, camera.height as u32));
    let scene = scene_kind.build(ReflectionType::Lambertian);

    let done = Arc::new(AtomicBool::new(false));
    let reporter = spawn_progress_reporter(Arc::clone(&buffer), Arc::clone(&done));

    camera.render(&scene, buffer.as_ref());
    done.store(true, Ordering::Relaxed);
    let _ = reporter.join();

    write_ppm(buffer.as_ref(), &mut stdout()).expect("failed to write PPM to stdout");
}
