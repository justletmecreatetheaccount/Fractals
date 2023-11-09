use sfml::window::{Event, Style};
use sfml::graphics::{RenderWindow, RenderTarget, Color, Shape};

fn main() {
    let mut circle = sfml::graphics::CircleShape::new(20.0, 30);
    circle.set_fill_color(Color::RED);
    // Create a new window
    let mut window = RenderWindow::new((800, 600),
                                       "SFML window",
                                       Style::CLOSE,
                                       &Default::default());
    // Limit the framerate to 60 frames per second (this step is optional)
    window.set_framerate_limit(60);

    // The main loop - ends as soon as the window is closed
    while window.is_open() {
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
        window.draw(&circle);
        window.display();
    }
}