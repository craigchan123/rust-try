use rand::{thread_rng,Rng};
use std::io;
pub use std::collections::HashMap;

pub fn median (){
  loop {
    println!("How many numbers of random positive integers do you want?");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("nono");
    let input: usize = match input.trim().parse() {
        Ok(num) =>num,
        Err(_) => continue,
    };
    
    let mut v = vec![1u16; input];
    
    thread_rng().fill(&mut v[..]);

    println!("The random list of {} integers is {:?}",input, v );

    v.sort();

    let len = v.len() as usize;
    let middle: usize = len /2 ;
    
    if len %2 == 0{
        let median_1 = v.get(middle-1).unwrap() ;
        let median_2 = v.get(middle).unwrap() ;
        fn safe_add (a: u16, b: u16) -> f32{
            (a as f32 + b as f32)/2.0
        }
        let ans = safe_add(*median_1, *median_2);
        println!("The median of the random list of integer is {}", ans);
        
    } else {
        let ans = v.get(middle).unwrap().to_string();
        println!("The median of the random list of integer is {}", ans)
    };

    let mut map = HashMap::new();
    for numbers in &mut v{
        let count = map.entry(numbers).or_insert(0);
        *count +=1;
    }

    println!("{:?}", map);
    
    break;
}
}