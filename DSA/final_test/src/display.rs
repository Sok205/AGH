use crate::visualizer::SortingAlgorithm;
use std::io;

pub const RESET: &str = "\x1B[0m";
pub const BOLD: &str = "\x1B[1m";
pub const GREEN: &str = "\x1B[32m";
pub const YELLOW: &str = "\x1B[33m";
pub const BLUE: &str = "\x1B[34m";
pub const MAGENTA: &str = "\x1B[35m";
pub const CYAN: &str = "\x1B[36m";

pub enum Partitions{
    Lomuto,
    Hoare,
    Random,
    Median3,
}

pub enum Choice{
    Structures,
    Algos,
}

pub fn display_the_starting_menu(choice: &Choice) -> Vec<&'static str> {
    match choice {  
        Choice::Structures => {
            display_structures_menu()
        },
        Choice::Algos => {
            display_algorithm_menu()
        }
    }
}
pub fn display_time_complexity_table(algorithm: &SortingAlgorithm) {
    let algo_name = match algorithm {
        SortingAlgorithm::Selection => "Selection Sort",
        SortingAlgorithm::Insertion => "Insertion Sort",
        SortingAlgorithm::Bubble => "Bubble Sort",
        SortingAlgorithm::Merge => "Merge Sort",
        SortingAlgorithm::Quick => "Quick Sort",
    };

    println!("\n{}{}{} Time Complexity:{}", BOLD, CYAN, algo_name, RESET);

    println!("{}{:<15} {:<8} {:<8} {:<8}{}", BOLD, "", "BIG O", "BIG Ω", "BIG Θ", RESET);

    match algorithm {
        SortingAlgorithm::Selection => {
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Best Case", "n²", "n²", "n²", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Average Case", "n²", "n²", "n²", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Worst Case", "n²", "n²", "n²", RESET);
            println!();
        },
        SortingAlgorithm::Insertion => {
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", GREEN, "Best Case", "n", "n", "n", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Average Case", "n²", "n²", "n²", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Worst Case", "n²", "n²", "n²", RESET);
            println!();
        },
        SortingAlgorithm::Bubble => {
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", GREEN, "Best Case", "n", "n", "n", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Average Case", "n²", "n²", "n²", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Worst Case", "n²", "n²", "n²", RESET);
            println!();
        },
        SortingAlgorithm::Merge => {
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", BLUE, "Best Case", "nlogn", "nlogn", "nlogn", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", BLUE, "Average Case", "nlogn", "nlogn", "nlogn", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", BLUE, "Worst Case", "nlogn", "nlogn", "nlogn", RESET);
            println!();
        },
        SortingAlgorithm::Quick => {
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", GREEN, "Best Case", "nlogn", "nlogn", "nlogn", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", GREEN, "Average Case", "nlogn", "nlogn", "nlogn", RESET);
            println!("{}{:<15} {:<8} {:<8} {:<8}{}", YELLOW, "Worst Case", "n²", "n²", "n²", RESET);
            println!();
            println!("Do you want to read more about the types of partitioning?");
            println!("1 - yes, 0 - no");
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("read error");

            match choice.trim() {
                "1" => {
                    display_partitioning_options();
                },
                _ => {
                    println!("Invalid choice");
                }
            }
        }
    }
}

pub fn display_algorithm_menu() -> Vec<&'static str> {
    let algorithms = [
        "1. Selection Sort",
        "2. Insertion Sort",
        "3. Bubble Sort",
        "4. Merge Sort",
        "5. Quick Sort",
    ];

    println!("\n{}{}Choose algorithm or input none numerical value to quit:{}", BOLD, CYAN, RESET);
    for (i, algo) in algorithms.iter().enumerate() {
        let color = match i {
            0 => YELLOW,
            1 | 2 => GREEN,
            3 => BLUE,
            4 => MAGENTA,
            _ => RESET,
        };
        println!("{}{}{}", color, algo, RESET);
    }

    algorithms.to_vec()
}

pub fn display_sorting_header(algorithm_name: &str) {
    println!("\n {}{}Running {}:{}", BOLD, CYAN, algorithm_name, RESET);
}

pub fn display_partitioning_options() {
    loop {
        println!("\n{}{}Quicksort Partitioning Methods:{}", BOLD, CYAN, RESET);
        println!("1. Lomuto Partition");
        println!("2. Hoare Partition");
        println!("3. Random Pivot");
        println!("4. Median-of-Three");
        println!("0. Back");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("read error");

        match choice.trim() {
            "1" => display_info_about_partition(&Partitions::Lomuto),
            "2" => display_info_about_partition(&Partitions::Hoare),
            "3" => display_info_about_partition(&Partitions::Random),
            "4" => display_info_about_partition(&Partitions::Median3),
            "0" | "" => break,
            _ => println!("{}Invalid choice, please try again{}", YELLOW, RESET)
        }
    }
}

pub fn display_info_about_partition(partition: &Partitions) {
    match partition {
        Partitions::Lomuto => {
            println!("\n{}{}Lomuto Partition:{}", BOLD, CYAN, RESET);
            println!("- {}Pivot:{} Usually the last element.", BOLD, RESET);
            println!("- {}How it works:{} Maintains an index `i` for the 'less than pivot' region.", BOLD, RESET);
            println!("  Iterates through the array, swaps elements <= pivot with `A[i]`,");
            println!("  then places the pivot at `A[i]`.");
            println!("- {}Pros:{} Simple and easy to implement.", BOLD, RESET);
            println!("- {}Cons:{} Can be inefficient with many duplicates or already sorted data.", BOLD, RESET);
        },
        Partitions::Hoare => {
            println!("\n{}{}Hoare Partition:{}", BOLD, CYAN, RESET);
            println!("- {}Pivot:{} Usually the first element.", BOLD, RESET);
            println!("- {}How it works:{} Uses two indices that start at the ends of the array", BOLD, RESET);
            println!("  and move toward each other, swapping elements that are on the wrong side.");
            println!("- {}Pros:{} Generally more efficient than Lomuto, performs fewer swaps.", BOLD, RESET);
            println!("- {}Cons:{} Slightly more complex to implement correctly.", BOLD, RESET);
        },
        Partitions::Random => {
            println!("\n{}{}Random Pivot Partition:{}", BOLD, CYAN, RESET);
            println!("- {}Pivot:{} A randomly selected element from the array.", BOLD, RESET);
            println!("- {}How it works:{} Selects a random element as pivot, then applies", BOLD, RESET);
            println!("  Lomuto or Hoare partitioning scheme.");
            println!("- {}Pros:{} Avoids worst-case performance on already sorted arrays.", BOLD, RESET);
            println!("- {}Cons:{} Requires a random number generator, less predictable performance.", BOLD, RESET);
        },
        Partitions::Median3 => {
            println!("\n{}{}Median-of-Three Partition:{}", BOLD, CYAN, RESET);
            println!("- {}Pivot:{} The median of the first, middle, and last elements.", BOLD, RESET);
            println!("- {}How it works:{} Selects the median of three elements (first, middle, last)", BOLD, RESET);
            println!("  as the pivot, then applies Lomuto or Hoare partitioning scheme.");
            println!("- {}Pros:{} Better pivot selection than just using first/last element.", BOLD, RESET);
            println!("- {}Cons:{} Slightly more overhead for small arrays.", BOLD, RESET);
        }
    }

    println!("\n{}Press Enter to return to partitioning methods...{}", BOLD, RESET);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
}

pub fn display_structures_menu() -> Vec<&'static str> {
    let structures = [
        "1. Heaps",
        "2. Priority Queue"
    ];

    println!("\n{}{}Choose structure or input none numerical value to quit:{}", BOLD, CYAN, RESET);
    for (i, strct) in structures.iter().enumerate() {
        let color = match i {
            0 => YELLOW,
            1 => GREEN,
            _ => RESET,
        };
        println!("{}{}{}", color, strct, RESET);
    }

    structures.to_vec()
}

pub fn run_priority_queue_interaction() {
    use crate::interactive::priority_queue::run_priority_queue_demo;
    run_priority_queue_demo();
}

pub fn display_time_complexities_meun(){
    
}
