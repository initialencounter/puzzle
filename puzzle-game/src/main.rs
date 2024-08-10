use device_query::{DeviceEvents, DeviceState, Keycode};
use std::env;
use std::io::stdin;
use std::process::exit;
use puzzle_lib::Puzzle;
mod utils;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use utils::{is_integer, log_state};

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let mut mode = 4;
    if args.len() > 1 && is_integer(&args[1]) {
        mode = args[1].parse().unwrap();
    }
    if args.len() > 2 && args[2] == "s" {
        let puzzle = Puzzle::new(mode);
        algebra(puzzle)
    } else {
        let puzzle = Puzzle::new(mode);
        direction(Arc::new(Mutex::new(puzzle)))
    }
}
fn direction(puzzle: Arc<Mutex<Puzzle>>) {
    {
        let puzzle = puzzle.lock().unwrap();
        log_state(&puzzle);
    }
    let device_state = DeviceState::new();
    let puzzle_clone = Arc::clone(&puzzle);
    let _guard_down = device_state.on_key_down(move |key| {
        let mut puzzle = puzzle_clone.lock().unwrap();
        match key {
            Keycode::Escape => {
                exit(0);
            }
            Keycode::Up => {
                puzzle.move_tile('U');
            }
            Keycode::Down => {
                puzzle.move_tile('D');
            }
            Keycode::Left => {
                puzzle.move_tile('L');
            }
            Keycode::Right => {
                puzzle.move_tile('R');
            }
            _ => {}
        }
        log_state(&puzzle);
        if puzzle.check() {
            println!("{}\n", puzzle.cmds_str);
            println!(
                "你赢了,用时{}s",
                (puzzle.end_time - puzzle.start_time) as f64 / 1000.0
            );
            exit(0)
        }
    });
    app_loop()
}
fn algebra(mut puzzle: Puzzle) {
    log_state(&puzzle);
    println!("请输入反向U、D、L、R 代表上下左右: ");
    while !puzzle.check() {
        let mut input: String = String::new();
        stdin().read_line(&mut input).expect("读取输入失败");
        puzzle.move_sequence(&input);
        log_state(&puzzle);
    }
    is_win(puzzle)
}

fn is_win(puzzle: Puzzle) {
    println!("{}\n", puzzle.cmds_str);
    println!(
        "你赢了,用时{}s",
        (puzzle.end_time - puzzle.start_time) as f64 / 1000.0
    );
    exit(0)
}

fn app_loop() -> ! {
    loop {
        let time_struct: Duration = Duration::from_millis(500);
        std::thread::sleep(time_struct);
    }
}
