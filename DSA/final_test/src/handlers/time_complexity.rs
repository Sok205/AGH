use std::io;
use crate::display::{BOLD, CYAN, YELLOW, RESET};
use crate::theories::time_complexity;
use crate::visualizer::SortingAlgorithm;
use crate::display::display_time_complexity_table;
use crate::handlers::other::handle_other_algorithms_menu;
use crate::theories::rabin_karp::display_rabin_karp_time_complexity;

pub fn handle_time_complexity_menu() {
    time_complexity::display_time_complexity_theory();

    loop {
        println!("\n{}{}Choose a category to view time complexity:{}", BOLD, CYAN, RESET);
        println!("1. Data Structures");
        println!("2. Sorting Algorithms");
        println!("3. Other Algorithms");
        println!("0. Back to main menu");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => handle_data_structure_complexity(),
            "2" => handle_sorting_algorithm_complexity(),
            "3" => handle_other_algorithm_complexity(),
            "0" | "" => break,
            _ => println!("{}Invalid choice, please try again{}", YELLOW, RESET),
        }
    }
}

fn handle_data_structure_complexity() {
    loop {
        println!("\n{}{}Choose a data structure to view time complexity:{}", BOLD, CYAN, RESET);
        println!("1. Heap");
        println!("2. Priority Queue");
        println!("3. Binary Search Tree (BST)");
        println!("4. AVL Tree");
        println!("0. Back");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => time_complexity::display_heap_time_complexity(),
            "2" => time_complexity::display_priority_queue_time_complexity(),
            "3" => time_complexity::display_bst_time_complexity(),
            "4" => time_complexity::display_avl_time_complexity(),
            "0" | "" => break,
            _ => println!("{}Invalid choice, please try again{}", YELLOW, RESET),
        }
    }
}

fn handle_sorting_algorithm_complexity() {
    loop {
        println!("\n{}{}Choose a sorting algorithm to view time complexity:{}", BOLD, CYAN, RESET);
        println!("1. Selection Sort");
        println!("2. Insertion Sort");
        println!("3. Bubble Sort");
        println!("4. Merge Sort");
        println!("5. Quick Sort");
        println!("6. Bucket Sort");
        println!("0. Back");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => display_time_complexity_table(&SortingAlgorithm::Selection),
            "2" => display_time_complexity_table(&SortingAlgorithm::Insertion),
            "3" => display_time_complexity_table(&SortingAlgorithm::Bubble),
            "4" => display_time_complexity_table(&SortingAlgorithm::Merge),
            "5" => display_time_complexity_table(&SortingAlgorithm::Quick),
            "6" => display_time_complexity_table(&SortingAlgorithm::Bucket),
            "0" | "" => break,
            _ => println!("{}Invalid choice, please try again{}", YELLOW, RESET),
        }
    }
}

fn handle_other_algorithm_complexity() {
    loop {
        println!("Choose an algorithm to view time complexity:");
        println!("1. Rabin-Karp");
        println!("0. Exit");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => display_rabin_karp_time_complexity(),
            "0" => break,
            _ => println!("{}Invalid choice, please try again{}", YELLOW, RESET),
        }
    }
}