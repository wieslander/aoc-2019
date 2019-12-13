use std::cmp::max;
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use pancurses::{initscr, endwin};
use aoc::get_input;
use aoc::intcode::Program;

fn render(grid: &HashMap<(i64, i64), i64>, score: i64) -> String {
    let mut width = 0;
    let mut height = 0;

    for (x, y) in grid.keys() {
        width = max(width, *x);
        height = max(height, *y);
    }

    let mut output = vec![];

    for y in 0..=height {
        let line: Vec<_> = (0..=width).map(|x| {
            match grid.get(&(x, y)) {
                Some(0) => " ",
                Some(1) => "█",
                Some(2) => "╳",
                Some(3) => "▔",
                Some(4) => "●",
                None => " ",
                _ => "_",
            }
        }).collect();
        output.push(line.join(""));
    }

    format!("{}\n\nScore: {}\n", output.join("\n"), score)
}

fn get_joystick(grid: &HashMap<(i64, i64), i64>) -> i64 {
    let mut paddle = -1;
    let mut ball = -1;

    for ((x, _), tile) in grid {
        match tile {
            3 => paddle = *x,
            4 => ball = *x,
            _ => (),
        }
    }

    if paddle > ball && ball % 3 != 0 {
        -1
    } else if paddle < ball && ball % 3 != 0 {
        1
    } else {
        0
    }
}

fn main() {
    let input = get_input();
    let mut initial_memory: Vec<i64> = input
        .trim()
        .split(',')
        .map(|x| x.parse().expect("NaN"))
        .collect();
    initial_memory[0] = 2;
    let mut program = Program::new(&initial_memory);
    let mut score = 0;
    let mut grid = HashMap::new();

    let window = initscr();

    while program.is_running() {
        if program.needs_input() {
            let joystick = get_joystick(&grid);
            program.set_input(joystick);
        }

        program.step();

        if program.has_output() {
            let x = program.pause_on_output().unwrap();
            let y = program.pause_on_output().unwrap();

            if x == -1 && y == 0 {
                score = program.pause_on_output().unwrap();
            } else {
                let tile = program.pause_on_output().unwrap();
                grid.insert((x, y), tile);
            }

            let values: Vec<i64> = grid.values().map(|v| *v).collect();
            if values.contains(&3) && values.contains(&4) {
                let screen = render(&grid, score);
                window.mvprintw(0, 0, &screen);
                window.refresh();
                sleep(Duration::from_millis(25));
            }
        }
    }

    endwin();
    println!("{}", score);
}
