use std::fs;

fn main() {
    let strategy_guide = fs::read_to_string("src/input.txt");
    match strategy_guide {
        Ok(s) => {
            
        }, 

        Err(e) => {
            println!("Couldn't open the file!");
        }
    }
}
