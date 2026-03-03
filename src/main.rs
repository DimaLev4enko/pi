fn main() {
let c =4.0;
 let mut pi = 0.0;
let mut b = 1.0;
    let mut plus: bool =true;
    for i in 0..u128::MAX{
    pi = pi + if plus == true {
            plus = false;
            1.0
        } else {
                plus = true;
                -1.0
            } * (c/b);
        b = b + 2.0;
        if i % 100000000 == 0 {
            println!("число пи {}", pi);
        }  
    }
}
