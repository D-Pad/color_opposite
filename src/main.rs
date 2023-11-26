use std::io;
use colored::Colorize;

fn main() {
    // Declare a string variable that will be used to store the user input
    let mut input = String::new();

    // Prompt the user to let them know they need to input a hex string
    println!("Enter the color hex string that you want to calculate an opposite for: ");

    // Create a string for storing RGB values:
    let mut opposite_color = String::new();
    let mut rgb_values: Vec<u8> = vec![];

    match io::stdin().read_line(&mut input) {
        Ok(_) => {

            // Trim whitespaces off of input and convert to uppercase
            let input = input.trim().to_uppercase().replace("#", "");

            // Split the characters into pairs
            let pairs: Vec<_> = input.chars().collect();

            // Loop through the pairs
            for chunk in pairs.chunks(2) {
                // Store the string for printing later.
                let mut hex_string = String::from(chunk[0]);
                hex_string.push(chunk[1]);

                // Create an iterator for the chunk
                let hex_collection: String = chunk.iter().collect();
                let value = u8::from_str_radix(&hex_collection, 16);

                match value {
                    Ok(result) => {
                        rgb_values.push(result);
                        let new_value: u8 = 255 - result;
                        let hex_value: String = format!("{:02X}", new_value);
                        opposite_color.push_str(&hex_value);
                    }
                    Err(_e) => {
                        println!("Invalid hex string: {}", hex_collection);
                    }
                }

            }
        }
        Err(_e) => println!("Something went wrong")
    }

    println!("\n{}\nHex: #{}", "Starting Color".yellow().underline(), input.trim().to_uppercase().cyan());
    println!("{}{}{}: {}, {}, {}", "R".red(), "G".green(), "B".blue(),
             rgb_values[0], rgb_values[1], rgb_values[2]);
    println!("\n{}\nHex: #{}", "Opposite Color".purple().underline(), opposite_color.cyan());
    println!("{}{}{}: {}, {}, {}", "R".red(), "G".green(), "B".blue(),
             255 - rgb_values[0], 255 - rgb_values[1], 255 -rgb_values[2]);

}
