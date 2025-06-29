use crate::display::{BOLD, CYAN, YELLOW, RESET, GREEN, BLUE};

pub fn display_rabin_karp_time_complexity() {
    println!("\n{}{}Rabin-Karp Time Complexity{}", BOLD, CYAN, RESET);

    println!("\n{}{:<20} {:<15} {:<15}{}", BOLD, "Operation", "Time Complexity", "Space Complexity", RESET);
    println!("{}{:-<20} {:-<15} {:-<15}{}", BOLD, "", "", "", RESET);

    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Hash Calculation", "O(m)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Rolling Hash", "O(1)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Pattern Matching", "O(n+m)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Multiple Patterns", "O(n+m*k)", "O(k)", RESET);

    println!("\n{}Best, Average, and Worst Case:{}", BOLD, BLUE);
    println!("{}{:<15} {:<15} {:<15} {:<20}{}", BOLD, "Case", "Time", "Space", "When", RESET);
    println!("{}{:-<15} {:-<15} {:-<15} {:-<20}{}", BOLD, "", "", "", "", RESET);
    println!("{}{:<15} {:<15} {:<15} {:<20}{}", GREEN, "Best", "O(n+m)", "O(1)", "No hash collisions", RESET);
    println!("{}{:<15} {:<15} {:<15} {:<20}{}", BLUE, "Average", "O(n+m)", "O(1)", "Few hash collisions", RESET);
    println!("{}{:<15} {:<15} {:<15} {:<20}{}", YELLOW, "Worst", "O(n*m)", "O(1)", "Many collisions", RESET);

    println!("\n{}Variables:{}", BOLD, BLUE);
    println!("- n: Length of the text");
    println!("- m: Length of the pattern");
    println!("- k: Number of patterns (for multi-pattern search)");

    println!("\n{}Key Insights:{}", BOLD, GREEN);
    println!("- Hash function quality directly impacts performance");
    println!("- Worst case occurs when all substrings hash to same value");
    println!("- More efficient than naive approach for multiple patterns");
    println!("- Rolling hash enables O(1) calculation of next window hash");
}