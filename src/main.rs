use std::io;

fn main() {
    clear_terminal();
    println!("   _____ __  __ _____   _____          _      ");
    println!("  / ____|  \\/  |  __ \\ / ____|   /\\   | |     ");
    println!(" | |    | \\  / | |  | | |       /  \\  | |     ");
    println!(" | |    | |\\/| | |  | | |      / /\\ \\ | |     ");
    println!(" | |____| |  | | |__| | |____ / ____ \\| |____ ");
    println!("  \\_____|_|  |_|_____/ \\_____/_/    \\_\\______|");
    menu()
}

fn menu() {
    let mut caltype;
    loop {
        println!("What type of calculation would you like to do?");
        println!("+ for addition");
        println!("- for subtract");
        println!("* for multiply");
        println!("/ for divide");
        println!("Exit to well... exit");
        caltype = String::new();
        io::stdin().read_line(&mut caltype).expect("Cannot read line");
        match caltype.to_lowercase().trim() {
            "+" => {
                add();
                break;
            }
            "-" => {
                subtract();
                break;
            }
            "*" => {
                multiply();
                break;
            }
            "/" => {
                divide();
                break;
            }
            "exit" => {
                clear_terminal();
                println!("Bye ig");
                break;
            }
            _ => {
                clear_terminal();
                println!("Invalid Opperation");
                println!("Try Again");
            }
        }
    }
}

fn add() {
    let operation1;
    let operation2;
    loop {
        let mut operation1str = String::new();
        clear_terminal();
        println!("Addition Selected");
        println!("Current operation: null + null");
        println!("What is the first number.");
        io::stdin().read_line(&mut operation1str).expect("Cannot read line");
        match operation1str.trim().parse::<f64>() {
            Ok(num) => {
                operation1 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    loop {
        let mut operation2str = String::new();
        clear_terminal();
        println!("Addition Selected");
        println!("Current operation: {operation1} + null");
        println!("What is the second number.");
        io::stdin().read_line(&mut operation2str).expect("Cannot read line");
        match operation2str.trim().parse::<f64>() {
            Ok(num) => {
                operation2 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    clear_terminal();
    let result = operation1 + operation2;
    println!("{operation1} + {operation2} = {result}");
    menu();
}

fn subtract() {
    let operation1;
    let operation2;
    loop {
        let mut operation1str = String::new();
        clear_terminal();
        println!("Subtract Selected");
        println!("Current operation: null - null");
        println!("What is the first number.");
        io::stdin().read_line(&mut operation1str).expect("Cannot read line");
        match operation1str.trim().parse::<f64>() {
            Ok(num) => {
                operation1 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    loop {
        let mut operation2str = String::new();
        clear_terminal();
        println!("Subtract Selected");
        println!("Current operation: {operation1} - null");
        println!("What is the second number.");
        io::stdin().read_line(&mut operation2str).expect("Cannot read line");
        match operation2str.trim().parse::<f64>() {
            Ok(num) => {
                operation2 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    clear_terminal();
    let result = operation1 - operation2;
    println!("{operation1} - {operation2} = {result}");
    menu();
}

fn multiply() {
    let operation1;
    let operation2;
    loop {
        let mut operation1str = String::new();
        clear_terminal();
        println!("Multiply Selected");
        println!("Current operation: null * null");
        println!("What is the first number.");
        io::stdin().read_line(&mut operation1str).expect("Cannot read line");
        match operation1str.trim().parse::<f64>() {
            Ok(num) => {
                operation1 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    loop {
        let mut operation2str = String::new();
        clear_terminal();
        println!("Multiply Selected");
        println!("Current operation: {operation1} * null");
        println!("What is the second number.");
        io::stdin().read_line(&mut operation2str).expect("Cannot read line");
        match operation2str.trim().parse::<f64>() {
            Ok(num) => {
                operation2 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    clear_terminal();
    let result = operation1 * operation2;
    println!("{operation1} * {operation2} = {result}");
    menu();
}

fn divide() {
    let operation1;
    let operation2;
    loop {
        let mut operation1str = String::new();
        clear_terminal();
        println!("Divide Selected");
        println!("Current operation: null / null");
        println!("What is the first number.");
        io::stdin().read_line(&mut operation1str).expect("Cannot read line");
        match operation1str.trim().parse::<f64>() {
            Ok(num) => {
                operation1 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    loop {
        let mut operation2str = String::new();
        clear_terminal();
        println!("Divide Selected");
        println!("Current operation: {operation1} / null");
        println!("What is the second number.");
        io::stdin().read_line(&mut operation2str).expect("Cannot read line");
        match operation2str.trim().parse::<f64>() {
            Ok(num) => {
                operation2 = num;
                break;
            }
            Err(_) => {
                println!("Not a valid number");
                println!("Try again");
                clear_terminal();
            }
        }
    }
    clear_terminal();
    let result = operation1 / operation2;
    println!("{operation1} / {operation2} = {result}");
    menu();
}

fn clear_terminal() {
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd").args(["/c", "cls"]).status().unwrap();
    #[cfg(target_os = "linux")]
    std::process::Command::new("clear").status().unwrap();
}