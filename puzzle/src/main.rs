use std::env;
use std::io::stdin;
use std::process::exit;
use device_query::{DeviceEvents, DeviceState, Keycode};
mod puzzle;
use puzzle::Puzzle;
mod utils;
use utils::is_integer;
use std::sync::{Arc, Mutex};
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    let mut mode = 4;
    if args.len()>1 && is_integer(&args[1]) {
        mode = args[1].parse().unwrap();
    }
    if args.len()>2 && args[2]=="s" {
        let puzzle = Puzzle::new(mode);
        algebra(puzzle)
    }else {
        let puzzle = Puzzle::new(mode);
        direction(Arc::new(Mutex::new(puzzle)))
    }
}
fn direction(puzzle: Arc<Mutex<Puzzle>>){
    {
        let puzzle = puzzle.lock().unwrap();
        puzzle.log_state();
    }
    let device_state = DeviceState::new();
    let puzzle_clone= Arc::clone(&puzzle);
    let _guard_down  = device_state.on_key_down(move |key| {
        let mut puzzle = puzzle_clone.lock().unwrap();
        match key {
            Keycode::Escape => {
                exit(0);
            },
            Keycode::Up => {
                puzzle.move_sequence(&"U");
            },
            Keycode::Down => {
                puzzle.move_sequence(&"D");
            },
            Keycode::Left => {
                puzzle.move_sequence(&"L");
            },
            Keycode::Right => {
                puzzle.move_sequence(&"R");
            },
            _ => {}
        }
        puzzle.log_state();
        if puzzle.check(){
            println!("{}\n",puzzle.cmds_str);
            println!("你赢了,用时{}s",(puzzle.end_time-puzzle.start_time)as f64 / 1000.0);
            app_exit()
        }
    });
    app_exit()
}
fn algebra(mut puzzle: Puzzle){
    puzzle.log_state();
    println!("请输入反向U、D、L、R 代表上下左右: ");
    while !puzzle.check() {
        let mut input : String = String::new();
        stdin().read_line(&mut input).expect("读取输入失败");
        puzzle.move_sequence(&input);
        puzzle.log_state();
    }
    is_win(puzzle)
}

fn is_win(puzzle: Puzzle){
    println!("{}\n",puzzle.cmds_str);
    println!("你赢了,用时{}s",(puzzle.end_time-puzzle.start_time)as f64 / 1000.0);
    app_exit()
}

fn app_exit() -> !{
    loop {
        let time_struct:Duration = Duration::from_millis(500);
        std::thread::sleep(time_struct);
    }
}