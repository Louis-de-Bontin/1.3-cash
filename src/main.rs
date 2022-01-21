use std::io;

fn main() {
    let mut nb_coins = 0;
    let mut cents = get_cents();
    println!("You owe : {}", cents.to_string());


    let quarters = calculate_quarters(cents);
    cents = cents - 25*quarters;
    nb_coins += quarters;
    println!("Giving {} quarters, {}c left.", quarters, cents);

    let dimes = calculate_dimes(cents);
    cents = cents - 10*dimes;
    nb_coins += dimes;
    println!("Giving {} dimes, {}c left.", dimes, cents);

    let nickels = calculate_nickels(cents);
    cents = cents - 5*nickels;
    nb_coins += nickels;
    println!("Giving {} nickels, {}c left.", nickels, cents);

    nb_coins += cents;
    println!("Giving {} pennies. Giving {} coins.", cents, nb_coins);
}

fn get_cents() -> u32{
    let mut cents;

    loop {
        cents = String::new();
        println!("How much change do you owe ?");

        io::stdin()
            .read_line(&mut cents)
            .expect("Failed to read input.");
        
        let cents: u32 = match cents.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return cents;
    }
}

fn calculate_quarters(cents: u32) -> u32 {
    return cents/25;
}

fn calculate_dimes(cents: u32) -> u32 {
    return cents/10;
}

fn calculate_nickels(cents: u32) -> u32 {
    return cents/5;
}
