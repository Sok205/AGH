use final_test::handlers::algo::handle_algorithms_menu;
use final_test::handlers::strct::handle_structures_menu;
use final_test::handlers::time_complexity::handle_time_complexity_menu;
use std::io;
fn main() {
    let array = [64, 32, 4, 8, 2, 128, 1, 512, 256];

    loop {
        println!("\n{}Choose what to explore:{}", "\x1b[1m\x1b[36m", "\x1b[0m");
        println!("1. Data Structures");
        println!("2. Sorting Algorithms");
        println!("3. Time Complexities");
        println!("0. Exit");

        let mut main_choice = String::new();
        io::stdin()
            .read_line(&mut main_choice)
            .expect("Failed to read line");

        match main_choice.trim() {
            "1" => handle_structures_menu(),
            "2" => handle_algorithms_menu(&array),
            "3" => handle_time_complexity_menu(),
            "0" | "" => break,
            _ => println!("{}Invalid choice, please try again{}", "\x1b[33m", "\x1b[0m"),
        }
    }
}