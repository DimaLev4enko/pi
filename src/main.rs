use dashu::float::DBig;
use std::io;

fn main() {
    let mut buffer = String::new();
    println!("Enter method:\n1.Slow\n2.Fast f64\n3.Fast Big\n4.Euler big");
    let choice = loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Ошибка");
        if let Ok(num) = buffer.trim().parse::<u8>() {
            if num > 0 && num <= 4 {
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
    let mut precision = 0;
    if choice == 3 || choice == 4 {
        println!("Введите точность: (желательно не больше 1000");
        precision = loop {
            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Error");
            if let Ok(num) = buffer.trim().parse::<usize>() {
                if num > 0 {
                    break num;
                } else {
                    println!("Wrong number");
                }
            } else {
                println!("Not number");
            }
        };
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
            if i == itter {
                println!("Финальный ответ, число пи {}", pi);
            } else if i % info == 0 {
                println!("Число пи {}", pi);
            }
        }
    } else if choice == 2 {
        let con = 4.0;
        let mut pi = 3.0;
        let (mut b1, mut b2, mut b3) = (2.0, 3.0, 4.0);
        let mut plus: bool = true;
        for i in 0..=itter {
            pi += if plus {
                plus = false;
                1.0
            } else {
                plus = true;
                -1.0
            } * (con / (b1 * b2 * b3));
            (b1, b2, b3) = (b1 + 2.0, b2 + 2.0, b3 + 2.0);
            if i == itter {
                println!("Финальный ответ, число пи {}", pi);
            } else if i % info == 0 {
                println!("Число пи {}", pi);
            }
        }
    } else if choice == 3 {
        let mut pi = DBig::from(3u32).with_precision(precision).value();
        let mut b1 = DBig::from(2u32).with_precision(precision).value();
        let mut b2 = DBig::from(3u32).with_precision(precision).value();
        let mut b3 = DBig::from(4u32).with_precision(precision).value();
        let con = DBig::from(4u32).with_precision(precision).value();
        let mut plus = true;
        for i in 0..=itter {
            let znak = if plus {
                plus = false;
                DBig::from(1u32)
            } else {
                plus = true;
                DBig::from(-1i32)
            };
            pi += &znak * (&con / (&b1 * &b2 * &b3));
            b1 += 2;
            b2 += 2;
            b3 += 2;
            if i == itter {
                println!("Финальный ответ, число пи {}", pi);
            } else if i % info == 0 {
                println!("Число пи {}", pi);
            }
        }
    } else if choice == 4 {
        let mut e = DBig::from(1u32).with_precision(precision).value();
        let mut fact = DBig::from(1u32).with_precision(precision).value();
        let one = DBig::from(1u32).with_precision(precision).value();
        for i in 1..=itter {
            fact *= i;
            e += &one / &fact;
            if i == itter {
                println!("Финальный ответ, число e {}", e);
            } else if i % info == 0 {
                println!("Число e {}", e);
            }
        }
    }
}
