use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::thread;
use std::time;
use rand::Rng;

fn write(stuff : &str) {
    print!("{} ", stuff);
    io::stdout().flush().unwrap();
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H"); // wat?
}

fn sleep_millis(millis : u64) {
    thread::sleep(time::Duration::from_millis(millis));
}

fn next_guess(already_guessed: bool) -> Option<i32> {
    match already_guessed {
        true => write("Try again:"),
        false => write("Enter a number:")
    }

    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("failed to read line.");

    let result = guess.trim().parse();
    match result {
        Ok(v) => Some(v),
        Err(_) => {
            print!("{} is not a number. ", guess.trim());
            None
        }
    }
}

fn play_again() -> bool {
    write("You win! Play again? (y/n)");

    let mut response = String::new();
    io::stdin().read_line(&mut response)
        .expect("failed to read line.");
    response.trim().eq_ignore_ascii_case("y")
}

fn main() {
    clear_screen();
    println!("Welcome to a game!");
    sleep_millis(750);

    loop {
        clear_screen();
        let secret_number = rand::thread_rng().gen_range(1..101);
        let mut already_guessed = false;

        'guess: loop {
            let guess = next_guess(already_guessed);
            match guess { None => continue, _ => {} }
            already_guessed = true;

            match guess.expect("should be i32")
                .cmp(&secret_number) {
                Ordering::Less => write("Too small!"),
                Ordering::Greater => write("Too big!"),
                Ordering::Equal => {
                    write("Congrats!");
                    break 'guess
                }
            }
        }
        match play_again() { true => continue, _ => break }
    }

    println!("Thanks for playing!");
    sleep_millis(750);
    clear_screen();
}
