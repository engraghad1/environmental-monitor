mod input;
mod room;
use input::{get_choice, get_input};
use room::Room;
fn menu() {
    println!("Room Temperature Logger");
    println!("1. Add Room");
    println!("2. List Rooms");
    println!("3. Exit");
    println!("Choose an option: ");
}
fn main() {
    let mut rooms: Vec<Room> = Vec::new();

    loop {
        menu();
        let choice = get_choice();

        if choice == 0 {
            println!("Goodbye!");
            break;
        }
        if choice == 1 {
            let name = get_input("Room name :");
            let tmp_str = get_input("Temperature :");
            let temperature: f64 = tmp_str.parse().expect("Error");
            let humidity_str = get_input("Humidity :");
            let humidity: u32 = humidity_str.parse().expect("Error");
            let room = Room {
                name,
                temperature,
                humidity,
            };
            rooms.push(room);
            println!("Room added!");
        }
        if choice == 2 {
            if rooms.is_empty() {
                println!("No rooms add yet");
            } else {
                println!("Room list ");
                for (i, room) in rooms.iter().enumerate() {
                    println!(
                        "{}.{} | Temp : {} | Hmidity : {}",
                        i + 1,
                        room.name,
                        room.temperature,
                        room.humidity
                    );
                }
            }
        }
    }
}
