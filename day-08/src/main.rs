use std::fs;

fn load_data() -> String {
    fs::read_to_string("data/data.txt").expect("Should have read")
}

fn part_one(data: &String) -> usize {
    let rows = data.lines();
    let num_rows = rows.clone().count();
    let num_cols = rows.clone().next().unwrap().chars().count();

    let mut visibility: Vec<Vec<bool>> = vec![vec![false; num_cols]; num_rows];

    // Left
    for (y, row) in rows.clone().enumerate() {
        let mut tallest = '0' as i32 - 1;
        for (x, tree) in row.chars().enumerate() {
            if tree as i32 <= tallest { continue; }

            tallest = tree as i32;
            visibility[y][x] = true;
        }
    }

    // Right
    for (y, row) in rows.clone().enumerate() {
        let mut tallest = '0' as i32 - 1;
        for (x, tree) in row.chars().rev().enumerate() {
            if tree as i32 <= tallest { continue; }

            tallest = tree as i32;
            visibility[y][num_cols - 1 - x] = true;
        }
    }

    // top
    for x in 0..num_cols {
        let mut tallest = '0' as i32 - 1;
        for y in 0..num_rows {
            let tree = rows.clone().nth(y).unwrap().chars().nth(x).unwrap();
            if tree as i32 <= tallest { continue; }

            tallest = tree as i32;
            visibility[y][x] = true;
        }
    }

    // Bottom
    for x in 0..num_cols {
        let mut tallest = '0' as i32 - 1;
        for y in (0..num_rows).rev() {
            let tree = rows.clone().nth(y).unwrap().chars().nth(x).unwrap();
            if tree as i32 <= tallest { continue; }

            tallest = tree as i32;
            visibility[y][x] = true;
        }
    }

    visibility.iter().map(|row| {
        row.iter().filter(|&&t| t).count()
    }).sum()
}

fn part_two(data: &String) -> usize {
    let rows = data.lines();
    let num_rows = rows.clone().count();
    let num_cols = rows.clone().next().unwrap().chars().count();

    let mut visibility: Vec<Vec<usize>> = vec![vec![0; num_cols]; num_rows];

    for y in 0..num_rows {
        for x in 0..num_cols {
            let row = rows.clone().nth(y).unwrap();
            let house_height = row.clone().chars().nth(x).unwrap();

            // Right
            let right = match (x+1..num_cols).position(|tx| { 
                 row.clone().chars().nth(tx).unwrap() >= house_height 
            }) {
                None => num_cols - x - 1,
                Some(tx) => tx + 1
            };

            // Left
            let left = match (1..=x).position(|tx| { 
                 row.clone().chars().nth(x - tx).unwrap() >= house_height 
            }) {
                None => x,
                Some(tx) => tx + 1
            };
            
            // Down
            let down = match (y+1..num_rows).position(|ty| { 
                 rows.clone().nth(ty).unwrap().chars().nth(x).unwrap() >= house_height 
            }) {
                None => num_rows - y - 1,
                Some(ty) => ty + 1
            };
            
            // Up
            let up = match (1..=y).position(|ty| { 
                 rows.clone().nth(y - ty).unwrap().chars().nth(x).unwrap() >= house_height 
            }) {
                None => y,
                Some(ty) => ty + 1
            };

            visibility[y][x] = right * left * up * down;
        }
    }

    *visibility.iter().flatten().max().unwrap()
}

fn main() {
    let data = load_data();
    let count = part_one(&data);
    println!("Part one: {}", count);
    let scenic = part_two(&data);
    println!("Part two: {}", scenic);
}
