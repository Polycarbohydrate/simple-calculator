use std::io;

enum Calculations   {
    Addition(f64, f64),
    Subtraction(f64, f64),
    Multiplication(f64, f64),
    Division(f64, f64),
}

impl Calculations   {
    fn add(first_num: f64, sec_num: f64)    {
        let x = first_num + sec_num;
        println!("===");
        println!("{x}");
    }

    fn sub(first_num: f64, sec_num: f64)    {
        let x = first_num - sec_num;
        println!("===");
        println!("{x}");
    }

    fn multi(first_num: f64, sec_num: f64)  {
        let x = first_num * sec_num;
        println!("===");
        println!("{x}");
    }

    fn divide(first_num: f64, sec_num: f64) {
        let x = first_num / sec_num;
        println!("===");
        println!("{x}");
    }
}

fn main()   {
    loop{
        println!("==================================================================");
        println!("Enter the number to the relevant calculation you would like to do.");
        println!("  1. Addition | 2. Subtraction | 3. Multiplication | 4. Division  ");
        println!("==================================================================");
        let mut user_calc = String::new();
        io::stdin().read_line(&mut user_calc).expect("Could not read line.");
        let user_calc: u32 = match user_calc.trim().parse() {
            Ok(num) => num,
            Err(_) =>   {
                println!("===============================================");
                println!("Enter a numeric number please. (1, 2, 3, 4 ...)");
                continue
            }
        };

        fn two_numbers() -> (f64, f64)  {
            println!("=======================");
            println!("Enter the first number:");
            println!("=======================");
            let mut first_num = String::new();
            io::stdin().read_line(&mut first_num).expect("Could not read line.");
            let first_num: f64 = match first_num.trim().parse() {
                Ok(num) => num,
                Err(_) =>   {
                    println!("====================================");
                    println!("Only enter a numerical number please");
                    return two_numbers();
                }
            };
            println!("=======================");
            println!("Enter the second number:");
            println!("=======================");
            let mut sec_num = String::new();
            io::stdin().read_line(&mut sec_num).expect("Could not read line.");
            let sec_num: f64 = match sec_num.trim().parse() {
                Ok(num) => num,
                Err(_) =>   {
                    println!("====================================");
                    println!("Only enter a numerical number please");
                    return two_numbers();
                }
            };
            (first_num, sec_num)
        }
        match user_calc {
            1 => {
                let (first_num, sec_num) = two_numbers();
                Calculations::add(first_num, sec_num);
            }
            2 =>    {
                let (first_num, sec_num) = two_numbers();
                Calculations::sub(first_num, sec_num);
            }
            3 =>    {
                let (first_num, sec_num) = two_numbers();
                Calculations::multi(first_num, sec_num);
            }
            4 =>    {
                let (first_num, sec_num) = two_numbers();
                Calculations::divide(first_num, sec_num);
            }
            _ =>    {
                println!("=============================================================");
                println!("Invalid choice. Please choose a valid option (1, 2, 3, or 4).");
            }
        }
        println!("===============================================");
        println!("Would you like to make another calculation? y/n");
        println!("===============================================");
        let mut user_again = String::new();
        io::stdin().read_line(&mut user_again).expect("Could not read line.");
        if user_again.trim().to_lowercase() != "y"  {
            break
        }
    }
}