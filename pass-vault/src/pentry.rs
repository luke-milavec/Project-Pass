extern crate crypto;

use serde::{Deserialize, Serialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;

use crypto::aes::{self, KeySize};
use crypto::blockmodes::PkcsPadding;
use crypto::symmetriccipher::*;
use crypto::buffer::{BufferResult,RefWriteBuffer, RefReadBuffer};
use crate::pentry::crypto::buffer::WriteBuffer;
use crate::pentry::crypto::buffer::ReadBuffer;




#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceInfo{
    pub service: String,
    pub username: String,
    pub password: Vec<u8>
}

impl ServiceInfo{
    pub fn new(service: String, username: String, password: Vec<u8>) -> Self{
        ServiceInfo {
            service,
            username,
            password,
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error>{
        serde_json::from_str(json_string)
    }

    #[allow(dead_code)]
    pub fn from_user_input() -> Self{

        println!("Enter Service:");
        let mut service = String::new();
        io::stdin()
            .read_line(&mut service)
            .expect("Failed to read line");

        println!("Enter Username:");
        let mut username = String::new();
        io::stdin()
            .read_line(&mut username)
            .expect("Failed to read line");

        println!("Enter Password:");
        let mut password = String::new();
        io::stdin()
            .read_line(&mut password)
            .expect("Failed to read line");

        ServiceInfo::new(
            service.trim().to_string(),
            username.trim().to_string(), 
            password.into_bytes(),
        )
    }

    pub fn to_json(&self) -> String{
        serde_json::to_string(&self).expect("Failed to serialize JSON")
    }

    pub fn write_to_file(&self){
        let json_output = format!("{}\n",self.to_json());
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open("passwords.json")
            {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_output.as_bytes()){
                        eprintln!("Error writing to file: {}",e);
                    } else {
                        println!("successfully wrote to passwords.json")
                    }
                }
                Err(e) => eprintln!("Error opening file: {}",e)
            }
    }
}

pub fn lock(pass: &str) -> Vec<u8>{
    let key = b"0123456789abcdef"; 
    let iv = b"0123456789abcdef";

    let mut encryptor = aes::cbc_encryptor(
        KeySize::KeySize128,
        key,
        iv,
        PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(pass.as_bytes());
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = encryptor.encrypt(&mut read_buffer, &mut write_buffer, true);

        
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));

        match result {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => { },
            Err(_) => todo!(),
        }
    }
    

    return final_result
}

pub fn unlock(ciph: &Vec<u8>) -> String {
    let key = b"0123456789abcdef"; 
    let iv = b"0123456789abcdef";
    
    let mut decryptor = aes::cbc_decryptor(
        KeySize::KeySize128,
        key,
        iv,
        PkcsPadding,
    );

    let mut final_result = Vec::<u8>::new();
    let mut read_buffer = RefReadBuffer::new(ciph);
    let mut buffer = [0; 4096];
    let mut write_buffer = RefWriteBuffer::new(&mut buffer);

    loop {
        let result = decryptor.decrypt(&mut read_buffer, &mut write_buffer, true);
        final_result.extend(write_buffer.take_read_buffer().take_remaining().iter().map(|&i| i));
        match result {
            Ok(BufferResult::BufferUnderflow) => break,
            Ok(BufferResult::BufferOverflow) => { },
            Err(_) => todo!(),
        }
    }
    
    

    
    return String::from_utf8(final_result).expect("Found invalid UTF-8")
   
    
}


pub fn read_passwords() -> Result<Vec<ServiceInfo>,io::Error>{
    let file = File::open("passwords.json")?;
    let reader = io::BufReader::new(file);
    let mut services = Vec::new();

    for line in reader.lines(){
        if let Ok(json_string) = line{
            if let Ok(service_info) = ServiceInfo::from_json(&json_string){
                services.push(service_info);
            }
        }
    }

    Ok(services)
}

pub fn prompt(prompt: &str) -> String{
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

