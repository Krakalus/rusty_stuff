use std::io;
use std::io::Write;
#[derive(Debug)]
enum Unit {
    Celsius,
    Fahrenheit,
    Kelvin,
}

#[derive(Debug)]
struct Temperature {
    value: f64,
    unit: Unit,
}

fn main() {
    println!();
    println!("🌵 Welcome to Rusty McThermometer’s Temperature Saloon 🌵");
    println!("Enter temperature + unit (examples: 32 f, 100 c, 373.15 k)");
    println!("Type 'quit' or 'q' to ride off into the sunset");
    println!("---------------------------------------------------------");

    loop {
        println!();
        print!("→ Your temperature: ");
        io::stdout().flush().unwrap(); // makes the arrow appear instantly

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read your rope");

        let input = input.trim();

        if input == "q" || input == "quit" {
            println!("Safe travels, partner! 🐎");
            break;
        }

        match parse_input(input) {
            Ok(temp) => {
                display_conversions(&temp);
                print_cowboy_comment(&temp);
            }
            Err(e) => println!("Hold your horses! {e}"),
        }
    }
}

// ——— Parsing magic (returns Result so we stay safe) ———
fn parse_input(s: &str) -> Result<Temperature, String> {
    let parts: Vec<&str> = s.split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Gimme a number and a unit, like '25 c'".into());
    }

    let value = parts[0]
        .parse::<f64>()
        .map_err(|_| "That number's drunk! Use somethin' like 25.5".to_string())?;

    let unit = match parts[1].to_lowercase().as_str() {
        "c" | "celsius" => Unit::Celsius,
        "f" | "fahrenheit" => Unit::Fahrenheit,
        "k" | "kelvin" => Unit::Kelvin,
        _ => return Err("Unknown unit! Only c, f, or k, partner".into()),
    };

    Ok(Temperature { value, unit })
}

// ——— The actual conversions ———
fn display_conversions(temp: &Temperature) {
    let c = match temp.unit {
        Unit::Celsius => temp.value,
        Unit::Fahrenheit => (temp.value - 32.0) * 5.0 / 9.0,
        Unit::Kelvin => temp.value - 273.15,
    };

    let f = c * 9.0 / 5.0 + 32.0;
    let k = c + 273.15;

    println!("    {c:.2} °C");
    println!("    {f:.2} °F");
    println!("    {k:.2} K");
}

// ——— Pure cowboy flavor ———
fn print_cowboy_comment(temp: &Temperature) {
    let c = match temp.unit {
        Unit::Celsius => temp.value,
        Unit::Fahrenheit => (temp.value - 32.0) * 5.0 / 9.0,
        Unit::Kelvin => temp.value - 273.15,
    };

    match c {
        x if x < -50.0 => println!("Colder than a polar bear’s toenails! 🥶"),
        x if x < 0.0 => println!("Freezin’ like a forgotten popsicle!"),
        x if x < 15.0 => println!("Chilly enough to make a penguin jealous."),
        x if x < 25.0 => println!("Nice and comfy, partner."),
        x if x < 35.0 => println!("Gettin’ toasty!"),
        x if x < 50.0 => println!("Hotter than a two-dollar pistol! 🔥"),
        x if x < 100.0 => println!("You could fry an egg on that sidewalk!"),
        _ => println!("That’s boilin’ the devil’s coffee! ☢️"),
    }
}