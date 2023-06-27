use std::io::prelude::*;
use std::io;
use std::fs::OpenOptions;
use rand::Rng;
use bardecoder;
use std::env;
use std::thread;
use std::time::Duration;
use qrcode_generator::QrCodeEcc;

fn clear_console() {
    let _ = std::process::Command::new("cmd")
        .arg("/C")
        .arg("cls")
        .status();
}   
fn bardecode(pathtoimage : String){
    clear_console();
    let current_dir = env::current_dir().expect("Failed to retrieve current directory").to_string_lossy().into_owned();
    let formatteddir: String = format!("{}\\{}.png", current_dir.trim(), pathtoimage.trim());
    let img = image::open(formatteddir).unwrap();
    
    // Use default decoder
    let decoder = bardecoder::default_decoder();
    let mut file_to_open : String = "0".to_owned();
    let results = decoder.decode(&img);
    for result in results {
        file_to_open = result.unwrap();
    }
    let to_be_read: String = format!("{}\\{}.data", current_dir.trim(), file_to_open.trim());
    let mut file = std::fs::File::open(to_be_read).expect("Couldnt open file");
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).expect("Couldnt read file!");
    //after reading the file and loading into a string display it
    for &byte in &contents {
        let character = byte as char;
        print!("{}", character);
    }
    thread::sleep(Duration::from_secs(10)); 
}
fn barcodegenerator(data: String) {
    qrcode_generator::to_png_to_file(data.clone(), QrCodeEcc::Low, 1024, data + ".png").unwrap();
}
fn admin() {
    let mut input = String::new();
    clear_console();
    println!("Enter the name of the product");

    io::stdin().read_line(&mut input).expect("failed to read msg");
    
    let random_number = rand::thread_rng().gen_range(1..=1000000000);

    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .open(random_number.clone().to_string() + ".data")
    .expect("Failed to open file");
 
    println!("Enter the price of the product");

    io::stdin().read_line(&mut input).expect("failed to read msg");

        //write price into file
        match write!(file ,"{}", input.clone()){
            Ok(_) => {},
            Err(e) => {
                println!("Error opening the file : {}", e);
            }
        }
        //exit after done

        //1. Name 2. Price

        barcodegenerator(random_number.to_string());

}
fn user() {
    clear_console();
    let mut input = String::new();
    println!("{}", input);
    println!("Enter the name of the picture having the barcode");
    io::stdin().read_line(&mut input).expect("failed to read msg");
    bardecode(input);
    
}




fn main() {
    let mut input = String::new();
    loop{
        input.clear();
        clear_console();
        println!("[1]Add products\n[2]Scan");
        io::stdin().read_line(&mut input).expect("failed to read msg");
        if input.trim() == "1" {
            admin();
        }
        else if input.trim() == "2" {
            user();
        }
        else {
            continue;
        }
    }
}