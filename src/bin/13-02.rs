use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use pancurses;
use aoc::get_input;
use aoc::intcode::Program;

fn render(window: &pancurses::Window, grid: &mut HashMap<(i64, i64), i64>, score: i64) {
    let board_offset = 1;

    for (&(x, y), tile) in grid.iter_mut() {
        let output = match tile {
            0 => "   ",
            1 => "███",
            2 => "▐█▌",
            3 => "▀▀▀",
            4 => " ● ",
            5 => " ▓ ",
            6 => " ▒ ",
            7 => " ░ ",
            _ => "_",
        };

        let attr = pancurses::COLOR_PAIR(*tile as u32);

        if *tile == 5 || *tile == 6 {
            *tile += 1;
        } else if *tile == 7 {
            *tile = 0;
        }


        window.attron(attr);
        let y = (y + board_offset) as i32;
        let x = (x * 3) as i32;
        window.mvprintw(y, x, output);
        window.attroff(attr);
    }

    window.attron(pancurses::COLOR_PAIR(2));
    window.mvprintw(0, 114, format!("Score: {:5}", score));
    window.attroff(pancurses::COLOR_PAIR(2));
    window.refresh();
}

fn get_joystick(grid: &HashMap<(i64, i64), i64>) -> i64 {
    let has_floor = match grid.get(&(1, 23)) {
        Some(0) => false,
        _ => true,
    };
    let mut paddle = -1;
    let mut ball = -1;

    for ((x, _), tile) in grid {
        match tile {
            3 => paddle = *x,
            4 => ball = *x,
            _ => (),
        }
    }

    if paddle > ball && (ball % 4 != 0 || !has_floor) {
        -1
    } else if paddle < ball && (ball % 4 != 0 || !has_floor) {
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

    let window = pancurses::initscr();
    pancurses::start_color();
    pancurses::init_pair(0, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    pancurses::init_pair(1, pancurses::COLOR_WHITE, pancurses::COLOR_BLACK);
    pancurses::init_pair(2, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);
    pancurses::init_pair(3, pancurses::COLOR_BLUE, pancurses::COLOR_BLACK);
    pancurses::init_pair(4, pancurses::COLOR_RED, pancurses::COLOR_BLACK);
    pancurses::init_pair(5, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);
    pancurses::init_pair(6, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);
    pancurses::init_pair(7, pancurses::COLOR_YELLOW, pancurses::COLOR_BLACK);
    pancurses::curs_set(0);

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
                let mut tile = program.pause_on_output().unwrap();
                if let Some(2) = grid.get(&(x, y)) {
                    if tile != 2 {
                        tile = 5;
                    }
                }
                grid.insert((x, y), tile);
            }

            let values: Vec<i64> = grid.values().map(|v| *v).collect();
            if values.contains(&3) && values.contains(&4) {
                render(&window, &mut grid, score);
                sleep(Duration::from_millis(50));
            }
        }
    }

    pancurses::endwin();
    println!("{}", score);
}
