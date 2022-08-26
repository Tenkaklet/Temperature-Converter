use std::io;

fn main() {
    println!("Please input an option:");

    let mut temp = String::new();

    io::stdin().read_line(&mut temp).expect("Can't read input");

    let mut t_number = String::new();

    let temp = temp.trim();
    if temp == "c" {
        println!("Input a temperature number:");
        io::stdin()
            .read_line(&mut t_number)
            .expect("Can't read number...");

        let t_number: i32 = t_number.trim().parse().expect("Expected a number");
        let res: i32 = convert_c(t_number);
        println!("The temperature is: {res}");
    } else {
        println!("Input a temperature number:");
        io::stdin()
            .read_line(&mut t_number)
            .expect("Can't read number...");

        let t_number: i32 = t_number.trim().parse().expect("Expected a number");
        let res = convert_f(t_number);
        println!("The temperature is: {res}");
    }
}

fn convert_f(farenheight: i32) -> i32 {
    let temperature = (farenheight - 32) * 5 / 9;
    return temperature;
}

fn convert_c(celcius: i32) -> i32 {
    return celcius * 9 / 5 + 32;
}
