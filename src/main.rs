use dashu::float::DBig;
use std::io;
use std::time::Instant;

fn my_pow(main: DBig, exp: u128, percision: usize) -> DBig {
    let mut res = DBig::from(1u32).with_precision(percision).value();
    for _ in 1..=exp {
        res *= &main;
    }
    res
}

fn main() {
    let mut buffer = String::new();
    println!(
        "Enter method:\n1.Slow\n2.Fast f64\n3.Fast Big\n4.Euler big\n5.Golden ratio big super fast sqrt(5)\n6.cos(x)"
    );
    let choice = loop {
        buffer.clear();
        io::stdin().read_line(&mut buffer).expect("Ошибка");
        if let Ok(num) = buffer.trim().parse::<u8>() {
            if num > 0 && num <= 6 {
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
    let mut precision = 1;
    if choice == 3 || choice == 4 || choice == 5 || choice == 6 {
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
    let mut x = DBig::from(1u32).with_precision(precision).value();
    if choice == 6 {
        println!("Enter x: ");
        x = loop {
            buffer.clear();
            io::stdin().read_line(&mut buffer).expect("Error");
            if let Ok(num) = buffer.trim().parse::<DBig>() {
                break num.with_precision(precision).value();
            } else {
                println!("Это не число! Давай еще раз:");
            }
        };
    }
    let start = Instant::now();
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
    } else if choice == 5 {
        let s = DBig::from(5u32).with_precision(precision).value();
        let one = DBig::from(1u32).with_precision(precision).value();
        let two = DBig::from(2u32).with_precision(precision).value();
        let mut x = &s / &two;
        for i in 0..=itter {
            x = (&x + (&s / &x)) / &two;
            if i % info == 0 {
                println!("Корень из 5 равняеться: {}", x);
            }
        }
        let phi = (&one + &x) / &two;
        println!("Golden ratio: {}", phi);
    } else if choice == 6 {
        let mut fac = DBig::from(6u32).with_precision(precision).value();
        let mut plus = false;
        let mut sinx = x.clone();
        let mut k = 3u128;
        for i in 0..=itter {
            let powx = my_pow(x.clone(), k, precision);
            if plus {
                plus = false;
                sinx += powx / &fac;
            } else {
                plus = true;
                sinx -= powx / &fac;
            }
            k += 1;
            fac *= k;
            k += 1;
            fac *= k;
            if i == itter {
                println!("Финальный ответ sin(x): {}", sinx);
            } else if i % info == 0 {
                println!("Расщет sin(x): {}", sinx);
            }
        }
    }
    let duration = start.elapsed();
    println!("Время: {:?}", duration);
}
