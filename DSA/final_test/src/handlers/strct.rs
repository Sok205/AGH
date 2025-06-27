use crate::visualizer::{Visualizer, DataStructures};
use crate::display::{display_structures_menu, CYAN, RESET};
use crate::Heap;
use std::io;

pub fn handle_structures_menu() {
    loop {
        let options = display_structures_menu();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice = choice.trim();

        if !choice.chars().all(char::is_numeric) {
            break;
        }

        match choice {
            "1" => {
                let mut max_heap: Heap<i32, _> = Heap::new(|a, b| a > b);
                let visualizer = Visualizer::new_structure(500, true, DataStructures::Heap);

                println!("\n{}Heap Visualization Demo:{}", "\x1b[1m\x1b[36m", "\x1b[0m");

                visualizer.visualize_heap_push(&mut max_heap, 256);
                visualizer.visualize_heap_push(&mut max_heap, 16);
                visualizer.visualize_heap_push(&mut max_heap, 8);
                visualizer.visualize_heap_push(&mut max_heap, 512);
                visualizer.visualize_heap_push(&mut max_heap, 4);
                visualizer.visualize_heap_push(&mut max_heap, 32);
                visualizer.visualize_heap_push(&mut max_heap, 2);
                visualizer.visualize_heap_push(&mut max_heap, 1);
                visualizer.visualize_heap_push(&mut max_heap, 128);

                visualizer.visualize_heap(&mut max_heap);

                while !max_heap.is_empty() {
                    visualizer.visualize_heap_pop(&mut max_heap);
                }
            },
            "2" =>{
                crate::theories::priority_queue::display_priority_queue_theory();

                println!("\n{}Would you like to try the interactive demo? (y/n){}", CYAN, RESET);
                let mut response = String::new();
                io::stdin().read_line(&mut response).expect("Failed to read line");

                if response.trim().to_lowercase() == "y" {
                    crate::display::run_priority_queue_interaction();
                } else {
                    crate::theories::priority_queue::display_interactive_hint();
                }
            }
            _ => break,
        }
    }
}