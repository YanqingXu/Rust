enum Direction {
    East = 1,
    West,
    North,
    South,
}

fn main() {
    let directions = [Direction::East, Direction::West, Direction::North, Direction::South];
    for dire in &directions {
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            },
            _ => println!("West"),
        };
    }
}