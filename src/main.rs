use std::io;

fn main() {
    // Declare a string variable that will be used to store the user input
    let mut input = String::new();

    // Prompt the user to let them know they need to input a hex string
    println!("Enter the color hex string that you want to calculate an opposite for: ");

    // Create a string for storing RGB values:
    let mut opposite_color = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_) => {

            // Trim whitespaces off of input and convert to uppercase
            let input = input.trim().to_uppercase().replace("#", "");
            println!("\nCalculating opposite of: {}", input);

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
                        let mut new_value: u8 = 255 - result;
                        let mut hex_value: String = format!("{:02X}", new_value);
                        opposite_color.push_str(&hex_value);
                    }
                    Err(e) => {
                        println!("Invalid hex string: {}", hex_collection);
                    }
                }

            }
        }
        Err(e) => println!("Something went wrong")
    }

    println!("Result: {}", opposite_color);

}
