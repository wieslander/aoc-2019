use std::process;
use aoc::get_input;
use aoc::intcode::Program;

fn scan_point(x: i64, y: i64, memory: &Vec<i64>) -> i64 {
    let mut p = Program::new(&memory);
    p.set_input(x);
    p.set_input(y);
    p.pause_on_output().unwrap()
}

fn main() {
    let memory = get_input()
        .trim()
        .split(',')
        .map(|x| x.trim().parse().expect("NaN"))
        .collect();
    let scan = |x, y| {
        scan_point(x, y, &memory)
    };

    let height = |x, y_start| {
        let mut y = y_start;
        while scan(x, y) == 1 {
            y += 1;
        }
        let h = y - y_start;
        println!("height({}, {}) = {}", x, y_start, h);
        h
    };

    let width = |x_start, y| {
        let mut x = x_start;
        while scan(x, y) == 1 {
            x += 1;
        }
        let w = x - x_start;
        println!("width({}, {}) = {}", x_start, y, w);
        w
    };

    let backtrack_from = |x_end, y_end| {
        let mut x = x_end;
        let mut y = y_end;

        let mut prev_x = i64::max_value();
        let mut prev_y = i64::max_value();

        while prev_x != x || prev_y != y {
            prev_x = x;
            prev_y = y;

            while height(x, y) >= 100 {
                x -= 1;
            }
            x += 1;

            while width(x, y) >= 100 {
                y -= 1;
            }
            y += 1;
        }

        println!("{}", x * 10000 + y);
        process::exit(0);
    };

    // The beam is very narrow at the start, so skip a bit
    let mut x = 0;
    let mut y = 75;

    // find the horizontal start of the beam
    loop {
        let out = scan(x, y);

        if out == 1 {
            break;
        } else {
            x += 1;
        }
    }

    loop {
        let line_start = x;
        let mut line_end = 0;

        while line_end == 0 {
            let out = scan(x, y);

            if out == 0 {
                line_end = x - 1;
            } else {
                x += 1;
            }
        }

        let line_width = line_end - line_start + 1;

        if line_width >= 150 {
            for candidate_x in (line_start..=(line_end - 99)).rev() {
                if height(candidate_x, y) >= 100 {
                    backtrack_from(candidate_x, y);
                } else {
                    break;
                }
            }
        }

        let col_start = y;
        x = line_end;

        y += 1;
        let mut col_end = 0;

        while col_end == 0 {
            let out = scan(x, y);

            if out == 0 {
                col_end = y - 1;
            }

            y += 1;
        }

        let col_height = col_end - col_start + 1;

        if col_height >= 150 {
            for candidate_y in (col_start..=(col_end - 99)).rev() {
                if width(x, candidate_y) >= 100 {
                    backtrack_from(x, candidate_y);
                } else {
                    break;
                }
            }
        }

        y = col_end;
    }
}
