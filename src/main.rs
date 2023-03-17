fn main() {
    println!("Please enter the number of queens: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid number");
    let d = input
        .trim()
        .parse::<usize>()
        .expect("Please enter a valid number");
    if d < 5 {
        eprintln!("Number must be bigger than 4");
        return;
    }
    let cols: Vec<u8> = solve(d);
    display(&cols);
}

fn solve(d: usize) -> Vec<u8> {
    let mut columns: Vec<u8> = Vec::new();
    let mut row: u8 = 0;
    let mut col: u8 = 0;
    loop {
        while col < d as u8 {
            if is_safe(&columns, col as i8) {
                columns.push(col);
                row += 1;
                col = 0;
                break;
            } else {
                col += 1;
            }
        }

        if row == d as u8 {
            return columns;
        }

        if col == d as u8 {
            if let Some(x) = columns.pop() {
                row -= 1;
                col = 1 + x;
            } else {
                panic!("NO SOLUTION FOUND!");
            }
        }
    }
}

fn is_safe(columns: &Vec<u8>, col: i8) -> bool {
    let row = columns.len();

    if let Some(_) = columns.iter().find(|&&i| i == col as u8) {
        return false;
    }
    for (q_row, &q_col) in columns.iter().enumerate() {
        if (q_col as i8 - q_row as i8) as i8 == col - row as i8 {
            return false;
        }

        if (columns.len() as i8 - q_col as i8) - q_row as i8
            == ((columns.len() as i8 - col) - row as i8) as i8
        {
            return false;
        }
    }
    true
}

fn display(vec: &Vec<u8>) {
    (0..vec.len()).for_each(|i| {
        (0..vec.len()).for_each(|j| {
            if j as u8 == vec[i] {
                print!("🟥");
            } else if (i + j) % 2 == 0 {
                print!("⬜️");
            } else {
                print!("⬛️");
            }
        });
        print!("\n");
    })
}
