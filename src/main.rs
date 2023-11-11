use sfml::system::{Vector2f, Vector2i, Vector2u};
use sfml::window::{Event, Style, VideoMode};
use sfml::graphics::{RenderWindow, RenderTarget, Color, Shader, RenderTexture, RectangleShape, Rect, Sprite, RenderStates};

const WIDTH: u32 = 1500;
const HEIGHT: u32 = 1000;
const ZOOM_SPEED: f32 = 10.0;

fn main() {
    let main_chrono = sfml::system::Clock::start();
    let mut shader: Shader = Shader::from_file("src\\shaders\\shaders.frag", sfml::graphics::ShaderType::Fragment).unwrap();
    // Create a new window
    let mut window = RenderWindow::new((WIDTH, HEIGHT),
                                       "SFML window",
                                       Style::CLOSE,
                                       &Default::default());
    window.set_framerate_limit(60);

    let texture: RenderTexture = RenderTexture::new(WIDTH, HEIGHT).unwrap();
    let sprite: Sprite = Sprite::with_texture(texture.texture());

    // The main loop - ends as soon as the window is closed
    while window.is_open() {
        shader.set_uniform_int("MAX_ITERATIONS", 50);
        shader.set_uniform_int("fractal_type", 1);
        shader.set_uniform_float("zoom", 2.0);//1.0/(f32::powf(main_chrono.elapsed_time().as_seconds(), main_chrono.elapsed_time().as_seconds()/ZOOM_SPEED)));
        shader.set_uniform_vec2("iResolution", Vector2f::new(WIDTH as f32, HEIGHT as f32));
        shader.set_uniform_float("fractal_precision", 4.0);
        shader.set_uniform_float("theta", main_chrono.elapsed_time().as_seconds());
        shader.set_uniform_vec2("offset", Vector2f::new(1.0, 0.0));
        // Event processing
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
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