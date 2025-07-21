const A : i32 = 31;
const B : i32 = 8;
const C : i32 = 21;

fn jake (d: i32, e: i32) -> i32 {
    let f = |g| {g * 4};
    d * e * ( A - B ) + C + f(2) 
}

fn main () {
    let iterated = vec![2, 5, 8, 9, 13];
    
    //.iter borrow elements and yields references
    // so we need to define x as a reference to i32 (&i32)
    // and then when parsed into jake or filter, we dereference it with *
    let results: Vec<_> = iterated.iter().map(|x: &i32| jake(*x, 5)).filter(|x: &i32| *x%3 > 0 ).collect();
   
    println!("{:?}", results);
}