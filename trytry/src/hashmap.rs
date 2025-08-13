pub use std::collections::HashMap;

pub fn hashmaps(){
    let mut scores = HashMap::new();
    scores.insert(String::from("Team Blue"), 10);
    scores.insert(String::from("Team Red"), 40);
    
    println!("{scores:?}");

    for (key, value) in &scores {
    println!("{key}:{value}");
    }
    
    let text = "hello to the hello hello world";
    let mut map = HashMap::new();
    for word in text.split_whitespace(){
        let count = map.entry(word).or_insert(0);
        *count +=1;
    }
    println!("{map:?}");
}