use std::io::{Stdout, stdout};
use crossterm::cursor::MoveTo;
use crossterm::ExecutableCommand;
use crossterm::style::{Color, PrintStyledContent, Stylize};
use crossterm::terminal::{Clear, ClearType};
use puzzle_lib::Puzzle;
pub fn is_integer(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}

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

pub fn log_state(puzzle: &Puzzle) {
    let mut stdout = stdout();
    stdout.execute(Clear(ClearType::All)).unwrap();
    stdout.execute(MoveTo(0, 0)).unwrap();
    for (i, row) in puzzle.puzzle.iter().enumerate() {
        for (j, &value) in row.iter().enumerate() {
            stdout
                .execute(MoveTo((j * 4) as u16, (i * 2) as u16))
                .unwrap();
            print_with_color(value, &mut stdout, puzzle.mode)
        }
    }
    println!("\n")
}