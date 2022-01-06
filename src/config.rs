use serde_json;
use std::env::current_exe;
use std::fs::read_to_string;

#[derive(Clone, Debug, Copy)]
pub struct Config {
    pub screen_w: f64,
    pub screen_h: f64,
    pub cell_w: f64,
    pub random_obstacle_count: u32,
    pub background_color: [f32; 4],
    pub seperator_line_color: [f32; 4],
    pub snake_head_color: [f32; 4],
    pub snake_first_color: [f32; 4],
    pub snake_second_color: [f32; 4],
    pub snake_turbo_first_color: [f32; 4],
    pub snake_turbo_second_color: [f32; 4],
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
        background_color: [0.321, 0.133, 0.403, 1.0],
        seperator_line_color: [0.0, 0.0, 0.0, 1.0],
        snake_head_color: [0.0, 0.0, 0.0, 1.0],
        snake_first_color: [123.0 / 255.0, 44.0 / 255.0, 191.0 / 255.0, 1.0],
        snake_second_color: [224.0 / 225.0, 170.0 / 255.0, 1.0, 1.0],
        snake_turbo_first_color: [13.0 / 255.0, 71.0 / 255.0, 161.0 / 255.0, 1.0],
        snake_turbo_second_color: [100.0 / 225.0, 181.0 / 255.0, 246.0 / 255.0, 1.0],
        obstacle_color: [16.0 / 255.0, 0.0, 43.0 / 255.0, 1.0],
        food_color: [199.0 / 255.0, 125.0 / 255.0, 1.0, 1.0],
    }
}

pub fn load_config_file() -> Result<Config, Box<dyn std::error::Error>> {
    let mut config = new();

    let mut config_path = current_exe().unwrap();
    config_path.pop();
    config_path.push("snake_config.json");

    match read_to_string(config_path) {
        Ok(file_content) => {
            let json: serde_json::Value = serde_json::from_str(file_content.as_str())?;

            if let Some(value) = json.get("screen_w") {
                if let serde_json::Value::Number(screen_w) = value {
                    config.screen_w = screen_w.as_f64().expect("screen_w should be a f64");
                }
            }

            if let Some(value) = json.get("screen_h") {
                if let serde_json::Value::Number(screen_h) = value {
                    config.screen_h = screen_h.as_f64().expect("screen_h should be a f64");
                }
            }

            if let Some(value) = json.get("cell_w") {
                if let serde_json::Value::Number(cell_w) = value {
                    config.cell_w = cell_w.as_f64().expect("cell_w should be a f64");
                }
            }

            if let Some(value) = json.get("random_obstacle_count") {
                if let serde_json::Value::Number(random_obstacle_count) = value {
                    config.random_obstacle_count = random_obstacle_count
                        .as_u64().expect("random obstacle_count should be a u64") as u32;
                }
            }

            if let Some(value) = json.get("background_color") {
                if let serde_json::Value::Array(background_color) = value {
                    config.background_color = vec_to_arr(background_color);
                }
            }

            if let Some(value) = json.get("seperator_line_color") {
                if let serde_json::Value::Array(seperator_line_color) = value {
                    config.seperator_line_color = vec_to_arr(seperator_line_color);
                }
            }

            if let Some(value) = json.get("snake_head_color") {
                if let serde_json::Value::Array(snake_head_color) = value {
                    config.snake_head_color = vec_to_arr(snake_head_color);
                }
            }

            if let Some(value) = json.get("snake_first_color") {
                if let serde_json::Value::Array(snake_first_color) = value {
                    config.snake_first_color = vec_to_arr(snake_first_color);
                }
            }

            if let Some(value) = json.get("snake_second_color") {
                if let serde_json::Value::Array(snake_second_color) = value {
                    config.snake_second_color = vec_to_arr(snake_second_color);
                }
            }

            if let Some(value) = json.get("snake_turbo_first_color") {
                if let serde_json::Value::Array(snake_turbo_first_color) = value {
                    config.snake_turbo_first_color = vec_to_arr(snake_turbo_first_color);
                }
            }

            if let Some(value) = json.get("snake_turbo_second_color") {
                if let serde_json::Value::Array(snake_turbo_second_color) = value {
                    config.snake_turbo_second_color = vec_to_arr(snake_turbo_second_color);
                }
            }

            if let Some(value) = json.get("obstacle_color") {
                if let serde_json::Value::Array(obstacle_color) = value {
                    config.obstacle_color = vec_to_arr(obstacle_color);
                }
            }

            if let Some(value) = json.get("food_color") {
                if let serde_json::Value::Array(food_color) = value {
                    config.food_color = vec_to_arr(food_color);
                }
            }
        }
        Err(_) => return Ok(config) // Just return the default config
    }

    Ok(config)
}

fn vec_to_arr(vec: &Vec<serde_json::Value>) -> [f32; 4] {
    if vec.len() < 4 {
        eprintln!("Expected vector to have at least 4 elements but it had {}", vec.len());
        std::process::exit(1);
    }
    [
        vec.get(0).unwrap().as_f64().expect("the value should be f64") as f32,
        vec.get(1).unwrap().as_f64().expect("the value should be f64") as f32,
        vec.get(2).unwrap().as_f64().expect("the value should be f64") as f32,
        vec.get(3).unwrap().as_f64().expect("the value should be f64") as f32,
    ]
}
