use std::io;
pub fn get_input(prompt: &str) -> String {
    println!("prompt:{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
pub fn get_choice() -> u32 {
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();
    let choice = choice.trim();
    if choice == "3" || choice == "exit" {
        return 0;
    }
    match choice.parse::<u32>() {
        Ok(num) => {
            
if (1..=3).contains(&num) {

    num

}

 else {
                println!("please choose 1-3");
                0
            }
        }
        Err(_) => {
            println!("Please enter a valid number");
            0
        }
    }
}
