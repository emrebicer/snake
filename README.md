# snake
A simple customizable snake game.

## How to play
Use ***WASD*** or ***arrow keys*** for changing the direction of the snake, use ***left shift*** to move faster and score more.

## Compiling and running
Use Cargo for compiling
```bash
cargo build --release # to compile
./target/release/snake # to run
```

## Customizing the game
When the game is started, it looks for a ***snake_config.json*** file at the same location as executable for custom configuration. If that file does not exist or some key/value pairs are not found in the config file, the default values will be used.

### Default configuration is as following;
```json
{
    "screen_w": 500.0,
    "screen_h": 600.0,
    "cell_w": 25.0,
    "random_obstacle_count": 10,
    "background_color": [0.321, 0.133, 0.403, 1.0],
    "seperator_line_color": [0.0, 0.0, 0.0, 1.0],
    "snake_head_color": [0.0, 0.0, 0.0, 1.0],
    "snake_first_color": [0.482, 0.172, 0.749, 1.0],
    "snake_second_color": [0.878, 0.666, 1.0, 1.0],
    "snake_turbo_first_color": [0.050, 0.278, 0.631, 1.0],
    "snake_turbo_second_color": [0.392, 0.709, 0.964, 1.0],
    "obstacle_color": [0.062, 0.0, 0.168, 1.0],
    "food_color": [0.780, 0.490, 1.0, 1.0]
}
```
