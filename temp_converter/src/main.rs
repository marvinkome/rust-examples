use std::io;

fn main() {
    println!("Temperature Converter!!");
    let temp = get_temp();
    let unit = get_unit();
    let mut result = 0.0;

    if unit == "C" {
        result = to_cels(temp);
    } else if unit == "F" {
        result = to_fahr(temp);
    }

    println!("Temperature is {}*{}", result, unit);
}

fn get_unit() -> String {
    let mut unit;

    loop {
        println!("Convert to? (F for fahrenheit & C for celsius");

        unit = String::new();
        io::stdin()
            .read_line(&mut unit)
            .expect("Failed to read line");

        unit = unit.trim().to_string();

        if unit == "C" || unit == "F" {
            break;
        }
    }

    unit
}

fn get_temp() -> f64 {
    let mut temp;

    loop {
        println!("Please input temperature");

        temp = String::new();
        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: f64 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break temp;
    }
}

fn to_fahr(celsius: f64) -> f64 {
    (celsius * (9.0 / 5.0)) + 32.0
}

fn to_cels(fahr: f64) -> f64 {
    (fahr - 32.0) * (5.0 / 9.0)
}
