use crate::sorting_algorithms::{selection_sort, insertion_sort, bubble_sort, merge_sort, quick_sort};
use crate::visualizer::{Visualizer, SortingAlgorithm};
use crate::display::{display_time_complexity_table, display_algorithm_menu, display_sorting_header};
use std::io;

pub fn handle_algorithms_menu(array: &[i32]) {
    loop {
        let options = display_algorithm_menu();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                display_sorting_header("Selection Sort");
                let vis = Visualizer::new(500, true, SortingAlgorithm::Selection);
                vis.visualize(array, selection_sort);
                display_time_complexity_table(&SortingAlgorithm::Selection);
            },
            "2" => {
                display_sorting_header("Insertion Sort");
                let vis = Visualizer::new(500, true, SortingAlgorithm::Insertion);
                vis.visualize(array, insertion_sort);
                display_time_complexity_table(&SortingAlgorithm::Insertion);
            },
            "3" => {
                display_sorting_header("Bubble Sort");
                let vis = Visualizer::new(500, true, SortingAlgorithm::Bubble);
                vis.visualize(array, bubble_sort);
                display_time_complexity_table(&SortingAlgorithm::Bubble);
            },
            "4" => {
                display_sorting_header("Merge Sort");
                let vis = Visualizer::new(500, true, SortingAlgorithm::Merge);
                vis.visualize(array, merge_sort);
                display_time_complexity_table(&SortingAlgorithm::Merge);
            },
            "5" => {
                display_sorting_header("Quick Sort");
                let vis = Visualizer::new(500, true, SortingAlgorithm::Quick);
                vis.visualize(array, quick_sort);
                display_time_complexity_table(&SortingAlgorithm::Quick);
            },
            _ => break,
        }
    }
}