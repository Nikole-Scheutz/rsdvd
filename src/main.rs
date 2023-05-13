use std::process::exit;

use rsdvd::Dvd;

fn main() {
    let mut dvd = Dvd::new();

    if let Err(e) = dvd.change_position(5) {
        println!("Application error: {e}");
        exit(1);
    }
}
