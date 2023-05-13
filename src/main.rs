use std::process::exit;

use rsdvd::Dvd;

fn main() {
    let mut dvd = Dvd::new();

    if let Err(e) = dvd.move_and_print(5) {
        println!("Application error: {e}");
        exit(1);
    }
}
