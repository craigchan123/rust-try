use std::{io};
use std::collections::HashMap;


fn main() {
    // let mut namevec = Vec::new();
    let mut company = HashMap::new();


    loop {
    println!("The name of the employee? Or type quit to leave");
    let mut name_input = String::new();
    io::stdin().read_line(&mut name_input).expect("msg");
    if name_input.trim() == "quit" {
        break;
    } else {
    
    println!("At which department?");
    
    let mut department_input = String::new();
    io::stdin().read_line(&mut department_input).expect("msg");
    
    
    company.entry(department_input.trim().to_string()).or_insert_with(Vec::new).push(name_input.trim().to_string());
    
    }
    }

    loop {
    println!("press 1 to see the list of all people in the company, and the department they are at.");
    println!("press 2 to get the people that are in a specific department.");
    println!("press 3 to exit.");


    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("should be 1 or 2.");
    let choice :u8 = match choice.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    if choice ==  1  {
        for (department, name) in &company{
        println!("{:?} is working at {} department.", name, department.trim());
        }
    } else if choice == 2 {
        println!("which department do you want to see?");
        let mut choice_of_department = String::new();
        io::stdin().read_line(&mut choice_of_department).expect("should be name of deparment");
        let choice_of_department = choice_of_department.trim();
        
        println!("People that are working at {} department are : {:?}",choice_of_department, company.get(choice_of_department).unwrap());
    }else{
        break;
    }
    }
}



