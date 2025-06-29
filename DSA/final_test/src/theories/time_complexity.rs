use crate::display::{BOLD, CYAN, GREEN, YELLOW, BLUE, RESET};

pub fn display_time_complexity_theory() {
    println!("\n{}{}Time Complexity Theory{}", BOLD, CYAN, RESET);

    println!("\n{}What is Time Complexity?{}", BOLD, GREEN);
    println!("Time complexity is a measure of the amount of time an algorithm takes to run");
    println!("as a function of the length of the input. It's typically expressed using Big O notation.");

    println!("\n{}Common Time Complexities:{}", BOLD, GREEN);
    println!("O(1) - Constant time: The operation takes the same amount of time regardless of input size");
    println!("O(log n) - Logarithmic time: Time increases logarithmically with input size");
    println!("O(n) - Linear time: Time increases linearly with input size");
    println!("O(n log n) - Log-linear time: Common in efficient sorting algorithms");
    println!("O(n²) - Quadratic time: Common in simple or nested iteration algorithms");
    println!("O(2^n) - Exponential time: Found in brute force algorithms");

    println!("\n{}{}Select a specific data structure to see its time complexity details.{}",
             BOLD, YELLOW, RESET);
}

pub fn display_heap_time_complexity() {
    println!("\n{}{}Heap Time Complexity{}", BOLD, CYAN, RESET);

    println!("\n{}{:<20} {:<15} {:<15}{}", BOLD, "Operation", "Time Complexity", "Space Complexity", RESET);
    println!("{}{:-<20} {:-<15} {:-<15}{}", BOLD, "", "", "", RESET);

    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Build Heap", "O(n)", "O(n)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Insert", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Extract Min/Max", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Get Min/Max", "O(1)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Heapify", "O(log n)", "O(1)", RESET);

    println!("\n{}Heap Properties:{}", BOLD, GREEN);
    println!("- Complete binary tree with the heap property");
    println!("- Max Heap: Parent key ≥ children keys");
    println!("- Min Heap: Parent key ≤ children keys");
    println!("- Used as the foundation for priority queues");

    println!("\n{}Applications:{}", BOLD, GREEN);
    println!("- Heap Sort");
    println!("- Priority Queues");
    println!("- Graph algorithms (Dijkstra, Prim)");
}

pub fn display_priority_queue_time_complexity() {
    println!("\n{}{}Priority Queue Time Complexity{}", BOLD, CYAN, RESET);

    println!("\n{}{:<20} {:<15} {:<15}{}", BOLD, "Operation", "Time Complexity", "Space Complexity", RESET);
    println!("{}{:-<20} {:-<15} {:-<15}{}", BOLD, "", "", "", RESET);

    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Enqueue", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Dequeue", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Peek", "O(1)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Is Empty", "O(1)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Get Length", "O(1)", "O(1)", RESET);

    println!("\n{}Implementation Methods:{}", BOLD, GREEN);
    println!("{}{:<20} {:<15} {:<15} {:<15}{}", BOLD, "Method", "Enqueue", "Dequeue", "Peek", RESET);
    println!("{}{:-<20} {:-<15} {:-<15} {:-<15}{}", BOLD, "", "", "", "", RESET);
    println!("{}{:<20} {:<15} {:<15} {:<15}{}", BLUE, "Binary Heap", "O(log n)", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15} {:<15}{}", BLUE, "Unsorted Array", "O(1)", "O(n)", "O(n)", RESET);
    println!("{}{:<20} {:<15} {:<15} {:<15}{}", BLUE, "Sorted Array", "O(n)", "O(1)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15} {:<15}{}", BLUE, "Fibonacci Heap*", "O(1)", "O(log n)", "O(1)", RESET);

    println!("\n{}*Amortized time complexity", YELLOW);

    println!("\n{}Applications:{}", BOLD, GREEN);
    println!("- Task scheduling in operating systems");
    println!("- Dijkstra's shortest path algorithm");
    println!("- A* search algorithm");
    println!("- Huffman coding for data compression");
}

pub fn display_bst_time_complexity() {
    println!("\n{}{}Binary Search Tree Time Complexity{}", BOLD, CYAN, RESET);

    println!("\n{}{:<20} {:<15} {:<15}{}", BOLD, "Operation", "Time Complexity", "Space Complexity", RESET);
    println!("{}{:-<20} {:-<15} {:-<15}{}", BOLD, "", "", "", RESET);

    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Search", "O(h)*", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Insert", "O(h)*", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Delete", "O(h)*", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Traversal", "O(n)", "O(h)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Min/Max", "O(h)", "O(1)", RESET);

    println!("\n{}*where h is the height of the tree (worst case: O(n) for unbalanced trees){}", YELLOW, RESET);

    println!("\n{}BST Properties:{}", BOLD, GREEN);
    println!("- For any node, all values in left subtree < node's value");
    println!("- For any node, all values in right subtree > node's value");
    println!("- No duplicate values (in standard implementation)");
    println!("- In-order traversal produces sorted output");

    println!("\n{}Applications:{}", BOLD, GREEN);
    println!("- Symbol tables in compilers");
    println!("- Database indexing");
    println!("- Implementing associative arrays");
    println!("- Implementation of sets and maps in STL libraries");
}

pub fn display_avl_time_complexity() {
    println!("\n{}{}AVL Tree Time Complexity{}", BOLD, CYAN, RESET);

    println!("\n{}{:<20} {:<15} {:<15}{}", BOLD, "Operation", "Time Complexity", "Space Complexity", RESET);
    println!("{}{:-<20} {:-<15} {:-<15}{}", BOLD, "", "", "", RESET);

    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Search", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Insert", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Delete", "O(log n)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Traversal", "O(n)", "O(log n)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Rotation", "O(1)", "O(1)", RESET);
    println!("{}{:<20} {:<15} {:<15}{}", GREEN, "Rebalancing", "O(log n)", "O(1)", RESET);

    println!("\n{}AVL Tree Properties:{}", BOLD, GREEN);
    println!("- Self-balancing binary search tree");
    println!("- For any node, height difference between left and right subtrees ≤ 1");
    println!("- Balance factor = height(left subtree) - height(right subtree)");
    println!("- Balance factor must be -1, 0, or 1 for all nodes");
    println!("- Guaranteed O(log n) height");

    println!("\n{}Applications:{}", BOLD, GREEN);
    println!("- Database indexing");
    println!("- In-memory sorting");
    println!("- Network routing algorithms");
    println!("- Applications where lookups are more frequent than insertions/deletions");
}