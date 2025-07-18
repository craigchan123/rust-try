
const BASE: i32 = 10;

fn nonnnto (g: i32 , t: i32) -> i32 {
    g * t
}
fn main () {
    let r = |portionmultiplier|{
    (BASE + 9) * portionmultiplier + nonnnto (8 , 6)
};
println! ("the result is : {}", r(22));
}

