use ggez::{
    conf::{WindowMode, WindowSetup},
    error::GameResult,
    event,
    graphics::{clear, draw, present, Color, MeshBuilder},
    nalgebra::Point2,
    Context,
};
use std::time::Duration;

fn l_system_next_generation(current_generation: &str) -> String {
    let f_rule = "f-h";
    let h_rule = "f+h";
    let mut next_gen = String::new();
    for char in current_generation.chars() {
        match char {
            'f' => next_gen.push_str(f_rule),
            'h' => next_gen.push_str(h_rule),
            '-' | '+' => next_gen.push(char),
            _ => panic!("Unknown char {}", char),
        }
    }
    next_gen
}

struct MainState {
    start_gen: String,
    next_gen: String,
    line_length: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let start_gen = "f";
        let next_gen = String::new();
        let line_length = 21.0;
        Ok(MainState {
            start_gen: start_gen.to_string(),
            next_gen,
            line_length,
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if self.next_gen.len() < 1_000_000 {
            self.next_gen = l_system_next_generation(&self.start_gen);
            self.start_gen = self.next_gen.clone();
            self.line_length -= 1.0;
        }
        ggez::timer::sleep(Duration::from_millis(1000));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let grey = Color::from_rgb(77, 77, 77);
        clear(ctx, grey);
        draw_lines(&self.next_gen, ctx, self.line_length)?;
        present(ctx)?;
        Ok(())
    }
}

fn next_point(current_point: Point2<f32>, heading: f32, line_length: f32) -> Point2<f32> {
    let next_point = (
        (current_point.x + (line_length * heading.to_radians().cos().trunc() as f32)),
        (current_point.y + (line_length * heading.to_radians().sin().trunc() as f32)),
    );
    Point2::new(next_point.0, next_point.1)
}

fn draw_lines(instructions: &str, ctx: &mut Context, line_length: f32) -> GameResult {
    let blue = Color::from_rgb(51, 153, 255);
    let line_width = 2.0;
    let mut heading = 0.0;
    let turn_angle = 90.0;
    let initial_point = Point2::new(100.0, 100.0);

    let mut start_point = initial_point;

    let mut line_builder = MeshBuilder::new();
    for char in instructions.chars() {
        let end_point = next_point(start_point, heading, line_length);
        match char {
            'f' | 'h' => {
                line_builder.line(&[start_point, end_point], line_width, blue)?;
                start_point = end_point;
            }
            '+' => heading += turn_angle,
            '-' => heading -= turn_angle,
            _ => panic!("Unknown char {}", char),
        }
    }
    let lines = line_builder.build(ctx)?;
    draw(ctx, &lines, (initial_point,))?;
    Ok(())
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("dragon curve", "huw")
        .window_setup(WindowSetup::default().title("Dragon curve"))
        .window_mode(WindowMode::default().dimensions(700.0, 700.0));
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}
