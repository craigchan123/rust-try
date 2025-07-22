fn main() {
    let s1 = String::from("hello to you");
    
    // here it use s1 as reference
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");
}

// here it borrows the String
// and references are immutable by default
fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
  // Here, s goes out of scope. But because s does not have ownership of what
  // it refers to, the value is not dropped.
}