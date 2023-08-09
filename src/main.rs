mod interpreter;

use std::env;
use std::process;

fn main() -> std::io::Result<()> {

    use std::fs::File;
    use std::io::prelude::*;
    use crate::interpreter::Interpreter;

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Incorrect number of arguements");

        process::exit(0x0);
    }

    println!("file to open: {}", args[1]);

    // * Lines for file search
    // let mut file = File::open(args[1].clone())?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // let _result = Interpreter::new(contents, 0 as usize);

    Ok(())
}

#[cfg(test)]

mod tests {

    use crate::interpreter::Interpreter;

    #[test]

    fn basic_math_w_whitespace() {
        let equations = vec![
            "10 + 1 + 2 - 3 + 4 + 6 - 15",
            "100+100",
            "11 +    3",
            "2 * 5",
            "2 + 3 * 4 + 1",
            "10 + 6 / 2",
            "9 * 3 + 5 - 100 / 10",
            "91- 5+ 4+6",
            "1- 12  -9",
            "92 + 1-92",
            "20 * 10 - 150 / 50"
        ];

        let answers = vec![5, 200, 14, 10, 15, 13, 22, 96, -20, 1, 197];

        for (i, each) in equations.iter().enumerate() {
            let mut parser = Interpreter::new(each.to_string(), 0 as usize);

            let result = parser.expr().unwrap();

            assert_eq!(result, answers[i]);
        }
    }
}
