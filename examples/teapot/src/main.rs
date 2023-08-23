use shrimp::shrimp::Shrimp;

use game_loop::game_loop;

use winit::{
    event_loop::EventLoop,
    window::WindowBuilder, dpi::LogicalSize,
};

const WIDTH: u32 = 1280;
const HEIGHT: u32= 720;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let size = LogicalSize::new(WIDTH, HEIGHT);

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
                    .with_title("Shrimp Demo")
                    .with_inner_size(size)
                    .with_min_inner_size(size)
                    .build(&event_loop).unwrap();
    let shrimp: Shrimp = Shrimp::new(1280, 720, &window)?;

    game_loop(
        event_loop,
        window,
        shrimp,
        240,
        0.1,
        |g, | {
            g.game.update();
        },
        |g| {
            match g.game.render() {
                Ok(_) => (),
                Err(e) => panic!("Error rendering: {}", e),
            }
        },
        |g, event| {
            match g.game.handle_input(event, &g.window) {
                Ok(_) => (),
                Err(e) => panic!("Error handling input: {}", e),
            }
        }
    );
}
