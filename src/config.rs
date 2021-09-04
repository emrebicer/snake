#[derive(Clone, Debug, Copy)]
pub struct Config {
    pub screen_w: f64,
    pub screen_h: f64,
    pub cell_w: f64,
    pub random_obstacle_count: u16,
    pub background_color: [f32; 4],
    pub seperator_line_color: [f32; 4],
    pub snake_head_color: [f32; 4],
    pub snake_first_color: [f32; 4],
    pub snake_second_color: [f32; 4],
    pub obstacle_color: [f32; 4],
    pub food_color: [f32; 4],
}

pub fn new() -> Config {
    // Return the default config
    Config {
        screen_w: 500.0,
        screen_h: 600.0,
        cell_w: 25.0,
        random_obstacle_count: 10,
        background_color: [0.42, 0.0, 0.5, 1.0],
        seperator_line_color: [0.0, 0.0, 0.0, 1.0],
        snake_head_color: [0.0, 0.0, 0.0, 1.0],
        snake_first_color: [1.0, 0.0, 0.0, 1.0],
        snake_second_color: [0.0, 0.0, 1.0, 1.0],
        obstacle_color: [1.0, 0.0, 0.5, 1.0],
        food_color: [1.0, 1.0, 0.0, 1.0],
    }
}
