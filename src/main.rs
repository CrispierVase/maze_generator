use std::time;

use image;
use rand::prelude::*;

const WIDTH: u32 = 51;
const HEIGHT: u32 = 51;

#[derive(Clone, Copy, Debug, std::cmp::PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let mut visited = [[true; WIDTH as usize]; HEIGHT as usize];
    let mut maze = [[false; WIDTH as usize]; HEIGHT as usize];
    let tmp_visited = visited.clone();
    for (idx, row) in tmp_visited.iter().enumerate() {
        for (idx1, _col) in row.iter().enumerate() {
            if idx % 2 == 1 && idx1 % 2 == 1 {
                maze[idx][idx1] = false;
                visited[idx][idx1] = true;
            } else {
                visited[idx][idx1] = false;
            }
        }
    }
    let stack: Vec<Point> = Vec::new();
            generate_maze(visited, maze, stack);
}

fn image_from_maze(maze: &[[bool; WIDTH as usize]; HEIGHT as usize]) {
    let mut img: image::RgbImage = image::ImageBuffer::new(WIDTH, HEIGHT);
    for (idx, row) in maze.iter().enumerate() {
        for (idx1, col) in row.iter().enumerate() {
            if col == &false {
                *img.get_pixel_mut(idx as u32, idx1 as u32) = image::Rgb([0, 0, 0]);
            } else if col == &true {
                *img.get_pixel_mut(idx as u32, idx1 as u32) = image::Rgb([255, 255, 255]);
            }
        }
    }
    img.save("result.png").unwrap();
}

fn get_in_between(old: Point, new: Point) -> Point {
    let x_change: u32 = (old.x + new.x) / 2;
    let y_change: u32 = (old.y + new.y) / 2;
    Point {
        x: x_change,
        y: y_change,
    }
}

fn find_neighbors(
    visited: &[[bool; WIDTH as usize]; HEIGHT as usize],
    location: &Point,
) -> (Vec<Point>, bool) {
    let mut neighbors: Vec<Point> = Vec::new();
    // up
    if location.y as i32 - 2 >= 0 {
        if visited[location.x as usize][location.y as usize - 2] == false {
            neighbors.push(Point {
                x: location.x,
                y: location.y - 2,
            });
        }
    }
    // down
    if location.y + 2 < HEIGHT {
        if visited[location.x as usize][location.y as usize + 2] == false {
            neighbors.push(Point {
                x: location.x,
                y: location.y + 2,
            })
        }
    }
    // left
    if location.x as i32 - 2 >= 0 {
        if visited[location.x as usize - 2][location.y as usize] == false {
            neighbors.push(Point {
                x: location.x - 2,
                y: location.y,
            })
        }
    }
    // right
    if location.x + 2 <= WIDTH {
        if visited[location.x as usize + 2][location.y as usize] == false {
            neighbors.push(Point {
                x: location.x + 2,
                y: location.y,
            })
        }
    }
    if neighbors.len() > 0 {
        return (neighbors, true);
    } else {
        return (neighbors, false);
    }
}

fn backtrack(mut stack: Vec<Point>) -> (Vec<Point>, bool) {
    if stack.len() == 1 {
        return (Vec::new(), false);
    } else {
        stack.remove(stack.len() - 1);
        return (stack, true);
    }
}

fn pick_random_int(max: u32) -> u32 {
    if max == 0 {
        return max;
    }
    let mut rng = rand::thread_rng();
    let val: u32 = rng.gen();
    return val % max;
}

fn generate_maze(
    mut visited: [[bool; WIDTH as usize]; HEIGHT as usize],
    mut maze: [[bool; WIDTH as usize]; HEIGHT as usize],
    mut stack: Vec<Point>,
) {
    let start_time = time::Instant::now();

    let mut start = Point {
        x: pick_random_int(WIDTH),
        y: 0,
    };
    if start.x % 2 == 1 && start.x + 1 < WIDTH {
        start.x = start.x + 1
    } else if start.x % 2 == 1 {
        start.x = start.x - 1
    }
    let mut current = Point {
        x: start.x,
        y: start.y + 1,
    };
    maze[current.x as usize][current.y as usize] = true;
    maze[start.x as usize][start.y as usize] = true;
    visited[start.x as usize][start.y as usize] = true;
    visited[current.x as usize][current.y as usize] = true;
    stack.push(current);
    while stack.len() >= 1 {
        let mut neighbors_result = find_neighbors(&visited, &current);

        while neighbors_result.1 == false {
            let stack_result = backtrack(stack);
            if stack_result.1 == false {
                println!(" Maze Generation took {:?}", start_time.elapsed());
                image_from_maze(&maze);
                return;
            } else {
                stack = stack_result.0;
            }
            current = stack[stack.len() - 1];
            neighbors_result = find_neighbors(&visited, &current);
        }
        let neighbors = neighbors_result.0;

        let old = current.clone();
        if !stack.contains(&current) {
            stack.push(current);
        }
        current = neighbors[pick_random_int(neighbors.len() as u32) as usize];
        maze[current.x as usize][current.y as usize] = true;
        let middle = get_in_between(old, current);
        maze[middle.x as usize][middle.y as usize] = true;
        visited[current.x as usize][current.y as usize] = true;
        maze[WIDTH as usize - 1][HEIGHT as usize - 1] = true;
    }
    image_from_maze(&maze);
}

fn solve_maze(maze: [[bool; WIDTH as usize]; HEIGHT as usize]) {
    let start_time = time::Instant::now();
    let mut stack: Vec<Point> = Vec::new();
    let mut distances: [[u32; WIDTH as usize]; HEIGHT as usize] = [[0; WIDTH as usize]; HEIGHT as usize];   
    let mut start = Point{x: 0, y: 0};

    let mut visited = [[true; WIDTH as usize]; HEIGHT as usize];
    let tmp_visited = visited.clone();
    for (idx, row) in tmp_visited.iter().enumerate() {
        for (idx1, _col) in row.iter().enumerate() {
            if idx % 2 == 1 && idx1 % 2 == 1 {
                visited[idx][idx1] = true;
            } else {
                visited[idx][idx1] = false;
            }
        }
    }

    for (idx, row) in maze.iter().enumerate() {
        if idx > 0 {
            break
        }
        for (idx1, col) in row.iter().enumerate() {
            if *col == true {
                start.x = idx1 as u32;
            }
        }
    }
    stack.push(start);
    let mut current = start.clone();
    // set distances
    while stack.len() >= 1 {
        let mut neighbors_result = find_neighbors(&visited, &current);

        while neighbors_result.1 == false {
            let stack_result = backtrack(stack);
            if stack_result.1 == false {
                println!(" Maze Generation took {:?}", start_time.elapsed());
                image_from_maze(&maze);
                return;
            } else {
                stack = stack_result.0;
            }
            current = stack[stack.len() - 1];
            neighbors_result = find_neighbors(&visited, &current);
        }
        let neighbors = neighbors_result.0;

        let old = current.clone();
        if !stack.contains(&current) {
            stack.push(current);
        }
        let old = current.clone();
        current = neighbors[pick_random_int(neighbors.len() as u32) as usize];
        distances[current.x as usize][current.y as usize] = distances[old.x as usize][old.y as usize] + 1;
        
    }

}   