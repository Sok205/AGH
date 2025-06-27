use std::io;

const RESET: &str = "\x1B[0m";
const BOLD: &str = "\x1B[1m";
const CYAN: &str = "\x1B[36m";

pub fn display_heap_theory() {
    println!("\n{}{}Heap Data Structure Theory:{}", BOLD, CYAN, RESET);

    println!("\n{}What is a Heap?{}", BOLD, RESET);
    println!("A heap is a specialized tree-based data structure that satisfies the heap property:");
    println!("- In a max heap, for any given node, the value of the node is greater than or equal to the values of its children");
    println!("- In a min heap, for any given node, the value of the node is less than or equal to the values of its children");
    println!("Heaps are commonly implemented as binary trees and are efficient for priority queue operations.");

    println!("\n{}Max Heap vs Min Heap{}", BOLD, RESET);
    println!("{}Max Heap:{}", BOLD, RESET);
    println!("- Root node contains the maximum value in the heap");
    println!("- For any node at index i, the value is greater than or equal to values at indices 2i+1 and 2i+2 (its children)");
    println!("- Used when we need quick access to the maximum element (e.g., finding largest elements)");

    println!("\n{}Min Heap:{}", BOLD, RESET);
    println!("- Root node contains the minimum value in the heap");
    println!("- For any node at index i, the value is less than or equal to values at indices 2i+1 and 2i+2 (its children)");
    println!("- Used when we need quick access to the minimum element (e.g., priority queues, Dijkstra's algorithm)");

    println!("\n{}Common Heap Operations:{}", BOLD, RESET);
    println!("- Insert: O(log n) - Add element and bubble up until heap property is maintained");
    println!("- Extract Max/Min: O(log n) - Remove root, replace with last element, and sift down");
    println!("- Peek: O(1) - Get max/min value without removing it");
    println!("- Heapify: O(n) - Convert an array into a valid heap");

    println!("\n{}How Heapify Works:{}", BOLD, RESET);
    println!("Heapify is the process of creating a heap data structure from a binary tree.");
    println!("The process starts from the last non-leaf node and works up to the root:");
    println!("1. Start from the last non-leaf node (n/2-1 where n is the number of nodes)");
    println!("2. For each node, compare with its children and swap if needed to maintain heap property");
    println!("3. Move upward to the parent node and repeat until reaching the root");
    println!("4. This bottom-up approach ensures the heap property is maintained at each step");

    println!("\n{}Max Heapify Process:{}", BOLD, RESET);
    println!("Max heapify is a key procedure to maintain the max heap property:");
    println!("1. Start with a node i which might violate the max heap property");
    println!("2. Find the largest value among the node and its children");
    println!("3. If the largest value is not the node itself, swap the node with that child");
    println!("4. Recursively apply max heapify to the affected subtree");
    println!("This ensures that the subtree rooted at index i follows the max heap property.");

    println!("\n{}Implementation Considerations:{}", BOLD, RESET);
    println!("- Array-based implementation: A binary heap can be represented as an array");
    println!("- For a node at index i:");
    println!("  * Parent is at ⌊(i-1)/2⌋");
    println!("  * Left child is at 2i+1");
    println!("  * Right child is at 2i+2");
    println!("- Heaps are complete binary trees, making them suitable for array representation");

    println!("\n{}Applications of Heaps:{}", BOLD, RESET);
    println!("- Priority Queues: Used in many algorithms like Dijkstra's and Prim's");
    println!("- Heap Sort: An efficient O(n log n) sorting algorithm");
    println!("- Selection algorithms: Finding kth smallest/largest elements");
    println!("- Graph algorithms: Shortest path, minimum spanning tree");
    println!("- Operating systems: Process scheduling, memory management");

    println!("\n{}Press Enter to return...{}", BOLD, RESET);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line");
}