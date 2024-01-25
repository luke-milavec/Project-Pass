mod pentry;

use crate::pentry::prompt;
use crate::pentry::read_passwords;


fn clr(){
    //print!("{}[2J", 27 as char);

}

fn main(){
    clr();
    let ascii = r#"
    __________                      ____   ____            .__   __   
    \______   \_____    ______ _____\   \ /   /____   __ __|  |_/  |_ 
    |     ___/\__  \  /  ___//  ___/\   Y   /\__  \ |  |  \  |\   __\
    |    |     / __ \_\___ \ \___ \  \     /  / __ \|  |  /  |_|  |  
    |____|    (____  /____  >____  >  \___/  (____  /____/|____/__|  
                    \/     \/     \/               \/                 
    "#;
    print!("{ascii}");
    loop{
        println!("Password Manager Menu:");
        println!("1. Add Entry");
        println!("2. List Entries");
        println!("3. Search Entry");
        println!("4. Quit");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        match choice.trim(){
            "1" => {
                clr();
                let entry = pentry::ServiceInfo::new(
                    prompt("Service: "),
                    prompt("Username: "),
                    pentry::lock(&prompt("Password: ")),
                );
                println!("Entry added successfully");
                entry.write_to_file();
            }
            "2" => {
                clr();
                let services = read_passwords().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}",err);
                    Vec::new()
                });
                for item in &services{
                    println!(
                        "Service = {}
                        - Username: {}
                        - Password: {}",
                        item.service,item.username,pentry::unlock(&item.password)
                    );
                }
            }
            "3" => {
                clr();
                let services = read_passwords().unwrap_or_else(|err| {
                    eprintln!("Error reading passwords: {}",err);
                    Vec::new()
                });
                let search = prompt("Search :");
                for item in &services{
                    if item.service.as_str() == search.as_str(){
                        println!(
                            "Service = {}
                            - Username: {}
                            - Password: {}",
                            item.service,item.username,pentry::unlock(&item.password)
                        );
                    }
                }
            }
            "4" => {
                clr();
                println!("Goodbye");
                break
            }
            _ => println!("Invalid Choice."),
        }
        println!("\n\n")
    }
}