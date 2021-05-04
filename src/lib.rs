pub mod cfg {
    use std::fs;

    pub struct Config {
        pub starting_board: String,
        pub steps: i32,
    }

    impl Config {
        pub fn new(args: &[String]) -> Result<Config, ConfigError> {
            if args.len() != 3 {
                return Err(ConfigError::ArgumentCountError { found: args.len(), expected: 3 })
            }

            let starting_board = fs::read_to_string(args[1].clone()).map(|str| str.trim().to_string());
            let steps = args[2].parse::<i32>();

            match (starting_board, steps) {
                (Ok(starting_board), Ok(steps)) if steps >= 0 => Ok(Config { starting_board, steps }),
                (Err(_), _) => Err(ConfigError::FileAccessError { file_name: args[1].clone() }),
                (_, Ok(steps)) => Err(ConfigError::StepCountError { found: Some(steps) }),
                (_, Err(_)) => Err(ConfigError::StepCountError {found: None }),
            }
        }
    }

    pub enum ConfigError {
        ArgumentCountError { found: usize, expected: usize },
        FileAccessError { file_name: String },
        StepCountError { found: Option<i32> }
    }
}

pub mod engine {
    use std::fmt;
    use std::fmt::Formatter;
    use crate::cfg::Config;

    pub fn run(config: Config) {
        let Config { starting_board, steps } = config;
        match Board::new(starting_board) {
            Some(board) => {
                println!("Starting board:\n{}", board);
            },
            None => eprintln!("Unable to parse board."),
        }
    }

    enum Cell {
        Alive,
        Dead,
    }

    /// A Board which stores a grid of [`Cell`] in row-major order.
    struct Board {
        grid: Vec<Vec<Cell>>,
    }

    impl Board {
        pub fn new(str: String) -> Option<Board> {
            let mut str_iter = str.lines();

            let grid_size_str = str_iter.next()?;
            let grid_size = grid_size_str.parse::<usize>().ok()?;
            let grid: Vec<Vec<Cell>> = str_iter.map(|grid_line|
                grid_line.split(" ").map(|grid_char| match grid_char {
                    "1" => Cell::Alive,
                    _   => Cell::Dead,
                }).collect()
            ).collect();

            if grid_size == grid.len() && grid.iter().all(|grid_line| grid_size == grid_line.len()) {
                Some(Board { grid })
            } else {
                None
            }
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            for grid_line in self.grid.iter() {
                let grid_line_str = grid_line.iter().map(|cell| match cell {
                    Cell::Alive => "1".to_string(),
                    Cell::Dead  => ".".to_string(),
                }).collect::<Vec<String>>().join(" ");
                
                writeln!(f, "{}", grid_line_str)?
            }
            
            Ok(())
        }
    }
}