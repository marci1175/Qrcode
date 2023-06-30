use std::io::prelude::*;
use std::io;
use std::fs::OpenOptions;

//to debug
//use std::time::Duration;

use rand::Rng;
use bardecoder;
use std::env;
use qrcode_generator::QrCodeEcc;

fn clear_console() {
    let _ = std::process::Command::new("cmd")
        .arg("/C")
        .arg("cls")
        .status();
}   
fn bardecode(pathtoimage : String, mut sepwords : Vec<String>) -> Vec<String>{
    
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
    let mut charactervector: Vec<char> = Vec::new();
    for &byte in &contents {
        let character = byte as char;
        charactervector.push(character);
    }
    let finalstring: String;
    let char_array: Box<[char]> = charactervector.into_boxed_slice();
    let char_array_ref: &[char] = &char_array;
    finalstring = char_array_ref.iter().collect();
    //last data
    let lastprice: i32;

    //check if its the first run => Vector has nothing in it
    let isfirst: bool;
    if sepwords.len() == 0 {
        isfirst = true;
    }
    else {
        isfirst = false;
    }

    //if its not not its first run read, store price from last cycle
    if !isfirst.clone() {
        lastprice = sepwords[0].clone().parse().unwrap();
    }
    else {
        //ensure safety in  future upadtes i.e leftover code
        lastprice = 0;
    }
    if isfirst {
        for linee in finalstring.lines() {
            sepwords.push(linee.to_string()); 
        } 
    }
    

    let mut readfile: Vec<String> = Vec::new();

        for linee in finalstring.lines() {
            readfile.push(linee.to_string()); 
        }

    let price = (readfile[0].parse::<i32>().unwrap() + lastprice.clone()).to_string();
    
    //sepwords.remove(0);
    sepwords.remove(0);
    sepwords.insert(0, price);
    if !isfirst {
        sepwords.push(readfile[1].clone()); 
    }
    return sepwords;

    //create loop by returning the (was empty) vector filled with data
    

}
fn barcodegenerator(data: String) {
    qrcode_generator::to_png_to_file(data.clone(), QrCodeEcc::Low, 1024, data + ".png").unwrap();
}
fn admin() {
    let mut input = String::new();
    let random_number = rand::thread_rng().gen_range(1..=1000000000);

    let mut file = OpenOptions::new()
    .create(true)
    .write(true)
    .open(random_number.clone().to_string() + ".data")
    .expect("Failed to open file");

    println!("Enter the price of the product");

    io::stdin().read_line(&mut input).expect("failed to read msg");
    
    write!(file ,"$").expect("failed to but $ sign xd");
 
    println!("Enter the name of the product");

    io::stdin().read_line(&mut input).expect("failed to read msg");

        //write price into file
        match write!(file ,"{}", input.clone()){
            Ok(_) => {},
            Err(e) => {
                println!("Error opening the file : {}", e);
            }
        }
        //exit after done

        //1. Price 2. Name

        barcodegenerator(random_number.to_string());

}
fn user() {
    clear_console();
    //init vec (self note: Marci te autista)
    let mut sepwords:Vec<String> = Vec::new();
    loop {
        
        let mut input = String::new();
        println!("{}", input);
        println!("Enter the name of the picture having the barcode, to quit type in 'quit'");
        io::stdin().read_line(&mut input).expect("failed to read msg");
        if input.trim() == "quit" {
            break;
        }
        //initalize vector so we can store data from last cycle (self note: Nagyon hülye vagy)
        clear_console();
        sepwords = bardecode(input, sepwords);

        println!("Price : {}", sepwords[0]);
        println!("Shopping cart : ");
        for i in 1..sepwords.len() {
            println!("{}", sepwords[i]);
        }
    }
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