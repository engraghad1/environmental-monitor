use std::io;
struct Sensor{
name :String, 
temperature:f64 ,
status:bool ,
}
fn get_input(prompt: &str) -> String {
println!("{}", prompt);
let mut input = String::new();
io::stdin().read_line(&mut input).expect("Error");
input.trim().to_string()
}
fn add_sensor( sensors:&mut Vec <Sensor> ,sensor:Sensor){
sensors.push(sensor);
}
fn list_sensor(sensors :& Vec <Sensor>)
{
for sensor in sensors{
println!("name:{}",sensor.name);
println!("temperature:{}",sensor.temperature);
println!("status:{}",sensor.status);
}
}
fn get_choice()->u32{
let mut choice = String ::new ();
io::stdin().read_line(&mut choice).expect("Error");
let choice = choice.trim();    
if choice == "3" || choice == "exit" {
return 0; }
match choice.parse::<u32>(){
Ok (num)=>num,
Err (_)=>{println!("Please enter a valid number");0}
}}
fn menu() {
println!("Sensor Tracker ");
println!("1. Add Sensor");
println!("2. List Sensor");
println!("3. Exit");
println!("Choose an option:");
}
fn main(){
let mut sensors = Vec ::new();
loop{
menu();
let choice = get_choice() ;
if choice == 0 {
println!("Bye");break;}
if choice == 1 {
let name = get_input("Sensor Name:");
let temperature = get_input("Sensor Temperature:");
let temperature = temperature.parse::<f64>().expect("Error");
let status = get_input("Sensor Status (true/false):").parse::<bool>().expect("Error");
let sensor = Sensor {
name: name,
temperature: temperature,
status: status,};
add_sensor(&mut sensors, sensor);}
if choice == 2 {list_sensor(&sensors);}}}

