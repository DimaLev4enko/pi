use std::io;
fn main() {
    let mut buffer = String::new();
    println!("Enter method:\n1.Slow\n2.Not now");
    let choice = loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Ошибка");
        if let Ok(num) = buffer.trim().parse::<u8>() {
            if num > 0 && num <= 1 {
                break num;
            } else {
                println!("Wrong number");
            }
        } else {
            println!("Not number");
        }
    };
    println!("Enter itteraion");
    let itter = loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Error");
        if let Ok(num) = buffer.trim().parse::<u128>() {
            if num > 0 {
                break num;
            } else {
                println!("Wrong number");
            }
        } else {
            println!("Not number");
        }
    };
    println!("Через соклько итераций виводить информацию?");
    let info = loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Error");
        if let Ok(num) = buffer.trim().parse::<u128>() {
            if num > 0 {
                break num;
            } else {
                println!("Wrong number");
            }
        } else {
            println!("Not number");
        }
    };
    if choice == 1 {
        let c = 4.0;
        let mut pi = 0.0;
        let mut b = 1.0;
        let mut plus: bool = true;
        for i in 0..=itter {
            pi += if plus {
                plus = false;
                1.0
            } else {
                plus = true;
                -1.0
            } * (c / b);
            b += 2.0;
            if i % info == 0 {
                println!("Число пи {}", pi);
            }
        }
    }
}
