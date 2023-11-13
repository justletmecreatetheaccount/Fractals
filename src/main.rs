use std::ops::AddAssign;

use sfml::system::{Vector2f, Vector2i, Vector2u};
use sfml::window::{Event, Style, VideoMode, mouse, Key as SfKey};
use sfml::graphics::{RenderWindow, RenderTarget, Color, Shader, RenderTexture, RectangleShape, Rect, Sprite, RenderStates};

const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1000;
const ZOOM_SPEED: f32 = 0.5;

fn main() {
    let mut main_chrono = sfml::system::Clock::start();
    let mut shader: Shader = Shader::from_file("src\\shaders\\shaders.frag", sfml::graphics::ShaderType::Fragment).unwrap();
    // Create a new window
    let mut window = RenderWindow::new((WIDTH, HEIGHT),
                                       "SFML window",
                                       Style::CLOSE,
                                       &Default::default());
    window.set_framerate_limit(60);

    let texture: RenderTexture = RenderTexture::new(WIDTH, HEIGHT).unwrap();
    let sprite: Sprite = Sprite::with_texture(texture.texture());

    //controls
    let mut offset: Vector2f = Vector2f::new(0.0, 0.0);
    let mut zoom = 2.0;
    let mut mouse_move: Vector2i = Vector2i::new(0, 0);;
    let mut pause = false;
    let mut save_time = 0.0;

    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        shader.set_uniform_int("MAX_ITERATIONS", 50);
        shader.set_uniform_int("fractal_type", 1);
        shader.set_uniform_float("zoom", zoom);//1.0/(f32::powf(main_chrono.elapsed_time().as_seconds(), main_chrono.elapsed_time().as_seconds()/ZOOM_SPEED)));
        shader.set_uniform_vec2("iResolution", Vector2f::new(WIDTH as f32, HEIGHT as f32));
        shader.set_uniform_float("fractal_precision", 4.0);
        if !pause {
            shader.set_uniform_float("theta", save_time + main_chrono.elapsed_time().as_seconds());
        } else {
            shader.set_uniform_float("theta", save_time);
        }
        shader.set_uniform_vec2("offset", offset);
        // Event processing
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::MouseButtonPressed { button, x, y } => {
                    mouse_move = Vector2i::new(x, y);
                }
                Event::MouseButtonReleased { button, x, y } => {
                    offset += Vector2f::new((mouse_move.x - x) as f32 * 0.001 * zoom, (y - mouse_move.y) as f32 * 0.001 * zoom);
                }
                Event::KeyPressed { code, .. } => {
                    if code == SfKey::S {
                        zoom += zoom * ZOOM_SPEED;
                    }
                    if code == SfKey::Z {
                        zoom -= zoom * ZOOM_SPEED;
                    }
                    if code == SfKey::A {
                        if !pause {
                            save_time += main_chrono.elapsed_time().as_seconds();
                            pause = true;
                        } else {
                            pause = false;
                            main_chrono.restart();
                        }
                    }
                }
                _ => {}
            }
        }

        window.clear(Color::BLACK);
        // SFML drawing commands go here...

        // End the current frame and display its contents on screen
        let mut render: RenderStates = RenderStates::default();
        render.set_shader(Some(&shader));
        window.draw_with_renderstates(&sprite, &render);
        window.display();
    }
}