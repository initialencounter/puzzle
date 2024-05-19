use log::info;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};

use crossterm::style::{Color, PrintStyledContent, Stylize};
use crossterm::{
    cursor::MoveTo,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Stdout};

const DIRECTION_DIST: phf::Map<&'static str, [i32; 2]> = phf::phf_map! {
    "U" => [-1, 0],
    "D" => [1, 0],
    "L" => [0, -1],
    "R" => [0, 1]
};

const DIRECTION_LIST: [&str; 4] = ["U", "D", "L", "R"];

const COLOR_MATRIX: [[Color; 10]; 10] = [
    [
        Color::Grey,
        Color::Grey,
        Color::Grey,
        Color::Grey,
        Color::Grey,
        Color::Grey,
        Color::Grey,
        Color::Grey,
        Color::Grey,
        Color::Grey,
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Green,
        Color::Green,
        Color::Green,
        Color::Green,
        Color::Green,
        Color::Green,
        Color::Green,
        Color::Green,
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Blue,
        Color::Blue,
        Color::Blue,
        Color::Blue,
        Color::Blue,
        Color::Blue,
        Color::Blue,
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Red,
        Color::Red,
        Color::Red,
        Color::Red,
        Color::Red,
        Color::Red,
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Yellow,
        Color::Yellow,
        Color::Yellow,
        Color::Yellow,
        Color::Yellow,
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Yellow,
        Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
        Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
        Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
        Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Yellow,
        Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
        Color::Cyan,
        Color::Cyan,
        Color::Cyan,
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Yellow,
        Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
        Color::Cyan,
        Color::Rgb {
            r: 138,
            g: 43,
            b: 226,
        },
        Color::Rgb {
            r: 138,
            g: 43,
            b: 226,
        },
    ],
    [
        Color::Grey,
        Color::Green,
        Color::Blue,
        Color::Red,
        Color::Rgb {
            r: 255,
            g: 165,
            b: 0,
        },
        Color::Yellow,
        Color::Rgb {
            r: 50,
            g: 170,
            b: 50,
        },
        Color::Cyan,
        Color::Rgb {
            r: 138,
            g: 43,
            b: 226,
        },
        Color::Black,
    ],
];
pub struct Puzzle {
    pub cmds_str: String,
    mode: usize,
    puzzle: Vec<Vec<i32>>,
    correct_puzzle: Vec<Vec<i32>>,
    pub start_time: u128,
    pub end_time: u128,
}

impl Puzzle {
    pub fn new(mode: usize) -> Self {
        let mut puzzle = vec![vec![0; mode]; mode];
        let mut correct_puzzle = vec![vec![0; mode]; mode];
        let mut num = 1;

        for i in 0..mode {
            for j in 0..mode {
                puzzle[i][j] = num;
                correct_puzzle[i][j] = num;
                num += 1;
            }
        }

        puzzle[mode - 1][mode - 1] = 0;
        correct_puzzle[mode - 1][mode - 1] = 0;

        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();

        let mut instance = Puzzle {
            cmds_str: String::new(),
            mode,
            puzzle,
            correct_puzzle,
            start_time,
            end_time: 0,
        };

        instance.shuffle();
        instance
    }

    fn find_0(&self) -> Option<(usize, usize)> {
        for (i, row) in self.puzzle.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                if value == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn move_tile<'a>(&'a mut self, direction: &'a str) -> &str {
        self.cmds_str.push_str(direction);
        if let Some((r, c)) = self.find_0() {
            if (r == 0 && direction == "U")
                || (r == self.mode - 1 && direction == "D")
                || (c == 0 && direction == "L")
                || (c == self.mode - 1 && direction == "R")
            {
                return "";
            }

            if let Some(&[dr, dc]) = DIRECTION_DIST.get(direction) {
                let rr = (r as i32 + dr) as usize;
                let cc = (c as i32 + dc) as usize;

                let num1 = self.puzzle[rr][cc];
                self.puzzle[r][c] = num1;
                self.puzzle[rr][cc] = 0;
                return direction;
            }
        }
        ""
    }

    pub fn check(&mut self) -> bool {
        for i in 0..self.mode {
            for j in 0..self.mode {
                if self.puzzle[i][j] != self.correct_puzzle[i][j] {
                    return false;
                }
            }
        }
        self.end_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        true
    }

    pub fn move_sequence(&mut self, sequence: &str) -> bool {
        let uppercase = sequence.to_uppercase();
        self.cmds_str.clear();
        for command in uppercase.chars() {
            let command_str = command.to_string();
            let _ = self.move_tile(&command_str);

            if self.check() {
                let duration = self.duration();
                info!("Puzzle solved, time taken: {}", duration);
                return true;
            }
        }
        false
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        for _ in 0..1000 {
            let random_direction = DIRECTION_LIST[rng.gen_range(0..4)];
            self.move_tile(random_direction);
        }
    }

    pub fn log_state(&self) {
        let mut stdout = stdout();
        stdout.execute(Clear(ClearType::All)).unwrap();
        stdout.execute(MoveTo(0, 0)).unwrap();
        for (i, row) in self.puzzle.iter().enumerate() {
            for (j, &value) in row.iter().enumerate() {
                stdout
                    .execute(MoveTo((j * 4) as u16, (i * 2) as u16))
                    .unwrap();
                print_with_color(value, &mut stdout, self.mode)
            }
        }
        println!("\n")
    }

    pub fn duration(&self) -> String {
        let time_now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis();
        let duration = time_now - self.start_time;
        self.format_duration(duration)
    }

    fn format_duration(&self, duration: u128) -> String {
        let hours = duration / 3600000;
        let minutes = (duration % 3600000) / 60000;
        let seconds = (duration % 60000) / 1000;
        format!("{}:{}:{}", hours, minutes, seconds)
    }
}

fn print_with_color(value: i32, stdout: &mut Stdout, mode: usize) {
    if value == 0 {
        let styled_value = "■".with(Color::White);
        stdout.execute(PrintStyledContent(styled_value)).unwrap();
    } else {
        let i = ((value - 1) / mode as i32) as usize;
        let j = (value - 1) as usize % mode;
        let styled_value = format!("{}", value).with(COLOR_MATRIX[i][j]);
        stdout.execute(PrintStyledContent(styled_value)).unwrap();
    }
}
