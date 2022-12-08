use std::env;
use std::io::{prelude::*, BufReader};

fn part1(heights: &Vec<Vec<i32>>) {

    let rows = heights.len();
    let columns = heights[0].len();

    let mut left = vec![vec![-1 as i32; columns]; rows];
    let mut right = vec![vec![-1 as i32; columns]; rows];
    let mut top = vec![vec![-1 as i32; columns]; rows];
    let mut bottom = vec![vec![-1 as i32; columns]; rows];

    for i in 0..rows {
        let bi = rows - 1 - i;

        for j in 0..columns {
            let rj = columns - 1 - j;

            if i != 0 {
                top[i][j] = std::cmp::max(top[i - 1][j], heights[i - 1][j]);
            }

            if bi != rows - 1 {
                bottom[bi][j] = std::cmp::max(bottom[bi + 1][j], heights[bi + 1][j]);
            }

            if j != 0 {
                left[i][j] = std::cmp::max(left[i][j - 1], heights[i][j - 1]);
            }

            if rj != columns - 1 {
                right[i][rj] = std::cmp::max(right[i][rj + 1], heights[i][rj + 1]);
            }
        }
    }

    let mut visible = 0;

    for i in 0..rows {
        for j in 0..columns {
            visible += (heights[i][j] > left[i][j]
                || heights[i][j] > right[i][j]
                || heights[i][j] > top[i][j]
                || heights[i][j] > bottom[i][j]) as i32;
        }
    }

    println!("Part 1: {}", visible);
}

fn part2(heights: &Vec<Vec<i32>>) {

    let rows = heights.len();
    let columns = heights[0].len();

    let mut best_scene = 1;
    for i in 1..rows-1 {
        for j in 1..columns-1 {

            let mut current_scene = 1;
            for l in 1..j+1 {
                if heights[i][j] <= heights[i][j-l] || l == j {
                    current_scene *= l;
                    break;
                }
            }

            for r in 1..columns-j {
                if heights[i][j] <= heights[i][j+r] || r == columns - j -1 {
                    current_scene *= r;
                    break;
                }
            }

            for t in 1..i+1 {
                if heights[i][j] <= heights[i-t][j] || t == i {
                    current_scene *= t;
                    break;
                }
            }

            for b in 1..rows-i {
                if heights[i][j] <= heights[i+b][j] || b == rows - i -1 {
                    current_scene *= b;
                    break;
                }
            }

            best_scene = std::cmp::max(best_scene, current_scene);
        }

    }

    println!("Part 2: {}", best_scene);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let reader = BufReader::new(std::fs::File::open(&args[1]).unwrap());

    let mut heights: Vec<Vec<i32>> = Vec::new();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        heights.push(line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
    }

    part1(&heights);
    part2(&heights);

}
