use std::io;
pub mod hashmap;




fn main() {
    hashmap::hashmaps();
    
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{s2}, world!");

    let mut v = vec![2,15,9,111];
    v.push(59);
    for i in &mut v{
        *i += 104; 
    }
    let ans = v.get(2).unwrap().to_string();
    println!( "{}",ans);

    println!("Next, just guess a random number smaller than 10!");
    loop{
    let mut input = String::new();
    io::stdin().read_line(&mut input ).expect("msg");
    let input: u8 = match input.trim().parse(){
        Ok(n) => n,
        Err(_) => {println!("No good, enter numbers");
                  continue}
    };
    if input>9  {
    println!("your input is {}, and it's too big", input);
    } else if input<9 {
        println!("your input is {}, and it's just a bit too small", input);
    } else {
        println!("correct input of {}", input);
        break;
    }
    }
    


    


}