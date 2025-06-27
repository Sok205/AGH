use crate::display::{BOLD, CYAN, GREEN, YELLOW, RESET};

pub fn display_priority_queue_theory() {
    println!("\n{}{}Priority Queue Theory{}", BOLD, CYAN, RESET);

    println!("\n{}What is a Priority Queue?{}", BOLD, GREEN);
    println!("A priority queue is an abstract data type similar to a regular queue or stack,");
    println!("but where each element has a 'priority' associated with it.");
    println!("Elements with higher priority are served before elements with lower priority,");
    println!("regardless of their order in the queue.");

    println!("\n{}Applications of Priority Queues:{}", BOLD, GREEN);
    println!("1. CPU Scheduling in operating systems");
    println!("2. Dijkstra's shortest path algorithm");
    println!("3. Huffman coding (data compression)");
    println!("4. Event-driven simulation");
    println!("5. A* search algorithm");

    println!("\n{}Key Operations:{}", BOLD, GREEN);
    println!("1. enqueue(item, priority): Insert an item with given priority");
    println!("2. dequeue(): Remove and return the highest priority item");
    println!("3. peek(): View the highest priority item without removing it");

    println!("\n{}Implementation Methods:{}", BOLD, GREEN);
    println!("1. Binary Heap - Most common implementation (O(log n) enqueue/dequeue)");
    println!("2. Fibonacci Heap - Theoretical advantage for some operations");
    println!("3. Unsorted Array/List - Simplest but inefficient (O(1) enqueue, O(n) dequeue)");
    println!("4. Sorted Array/List - Efficient dequeue (O(1)) but inefficient enqueue (O(n))");

    println!("\n{}Binary Heap Implementation:{}", BOLD, GREEN);
    println!("A binary heap is a complete binary tree where:");
    println!("- In a max heap: each node is greater than or equal to its children");
    println!("- In a min heap: each node is less than or equal to its children");
    println!("This property ensures that the root is always the maximum/minimum element.");

    println!("\n{}Complexity Analysis:{}", BOLD, GREEN);
    println!("Operation | Binary Heap | Unsorted Array | Sorted Array");
    println!("-----------------------------------------------------");
    println!("Enqueue   | O(log n)    | O(1)           | O(n)");
    println!("Dequeue   | O(log n)    | O(n)           | O(1)");
    println!("Peek      | O(1)        | O(n)           | O(1)");

    println!("\n{}{}For an interactive demo of priority queues, select the Priority Queue option from the structures menu.{}",
             BOLD, YELLOW, RESET);
}

pub fn display_interactive_hint() {
    println!("\n{}{}To see Priority Queue in action:{}", BOLD, CYAN, RESET);
    println!("1. Return to the main structures menu");
    println!("2. Select 'Priority Queue'");
    println!("3. Choose between Max and Min priority queue");
    println!("4. Try operations like enqueue, dequeue, and visualization");
}