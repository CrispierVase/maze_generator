use image;

const WIDTH: u32 = 5;
const HEIGHT: u32 = 5;

#[derive(Clone, Copy, Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let mut visited = [[true; WIDTH as usize]; HEIGHT as usize];
    let mut maze = visited.clone();
    let mut tmp_visited = visited.clone();
    for (idx, row) in tmp_visited.iter().enumerate() {
        for (idx1, col) in row.iter().enumerate() {
            if idx % 2 == 1 && idx1 % 2 == 1 {
                maze[idx][idx1] = false;
                visited[idx][idx1] = true;
            } else {
                visited[idx][idx1] = false;
            }
        }
    }
    image_from_maze(&maze);
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
    maze: &[[bool; WIDTH as usize]; HEIGHT as usize],
    visited: &[[bool; WIDTH as usize]; HEIGHT as usize],
    location: Point,
) -> (Vec<Point>, bool) {
    let mut neighbors: Vec<Point> = Vec::new();
    // up
    if location.y as i32 - 2 >= 0 {
        if visited[location.x as usize][location.y as usize - 2] == false
            && maze[location.x as usize][location.y as usize - 2] == true
        {
            neighbors.push(Point {
                x: location.x,
                y: location.y - 2,
            });
        }
    }
    // down
    if location.y + 2 < HEIGHT {
        if visited[location.x as usize][location.y as usize + 2] == false
            && maze[location.x as usize][location.y as usize + 2] == true
        {
            neighbors.push(Point {
                x: location.x,
                y: location.y + 2,
            })
        }
    }
    // left
    if location.x as i32 - 2 >= 0 {
        if visited[location.x as usize - 2][location.y as usize] == false
            && maze[location.x as usize - 2][location.y as usize] == true
        {
            neighbors.push(Point {
                x: location.x - 2,
                y: location.y,
            })
        }
    }
    // right
    if location.x + 2 < WIDTH {
        if visited[location.x as usize + 2][location.y as usize] == false
            && maze[location.x as usize + 2][location.y as usize] == true
        {
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
