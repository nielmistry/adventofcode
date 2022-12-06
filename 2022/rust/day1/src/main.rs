use std::{fs};

fn get_n_max_sum(data: &String, n: u32) -> u32 {
    let values = data.split("\n");
    let mut vec_values = Vec::new();
    let mut current_col = Vec::new();

    for v in values { 
        let current_value = u32::from_str_radix(v, 10);

        match current_value {
            Ok(v) => {
                current_col.push(v);
            },
            Err(_e) => {
                vec_values.push(current_col.iter().sum());
                current_col.clear();
            }
        }

    }

    vec_values.sort_unstable();
    let len = vec_values.len(); 
    let n_usize: usize = n.try_into().unwrap();   
    vec_values[(len - n_usize)..len].iter().sum()
}

fn main() {
    let file_contents = fs::read_to_string("src/input.txt");
    match file_contents {
        Ok(v) => println!("Max Sum: {}", get_n_max_sum(&v, 3)),
        Err(e) => println!("Couldn't read file because {e}"),
    }

}
