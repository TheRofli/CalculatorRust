use std::io;

fn main() {

    println!("Type exit to exit");

    let mut cur_nums: Vec<i32> = vec![];
    let mut result: i32 = 0;

    let mut buffer: String = String::new();
    let mut buffer_str: &str;

    while buffer.ne("exit") {
        buffer.clear();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");
        buffer = buffer.trim().to_string();
        buffer_str = buffer.as_str();
        match buffer_str {
            "+" | "-" | "/" | "*" => { 
                match buffer_str {
                    "+" => result += cur_nums.iter().sum::<i32>(),
                    "-" => result -= cur_nums.iter().sum::<i32>(),
                    "/" => cur_nums.iter().for_each(|&i| result /= i),
                    "*" => cur_nums.iter().for_each(|&i| result *= i),
                    _ => (),
                }
                cur_nums.clear();
            }
            "exit" => continue,
            _ => {
                match buffer.parse() { 
                    Ok(n) => cur_nums.push(n),
                    Err(_error) => println!("Unsupported operation"),
                }
                continue;
            },
        }
        println!("Current result: {}", result);
    }

    println!("Total result: {}", result);
}
