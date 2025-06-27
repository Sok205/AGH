use std::io;
use crate::PriorityQueue;
use crate::display::{BOLD, CYAN, GREEN, YELLOW, RESET};

pub fn run_priority_queue_demo() {
    println!("\n{}{}Priority Queue Demo:{}", BOLD, CYAN, RESET);

    // Create an interactive demo for users
    println!("Select priority queue type:");
    println!("1. Max Priority Queue (largest element has highest priority)");
    println!("2. Min Priority Queue (smallest element has highest priority)");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");

    match choice.trim() {
        "1" => demonstrate_priority_queue(QueueType::Max),
        "2" => demonstrate_priority_queue(QueueType::Min),
        _ => {
            println!("{}Invalid choice, defaulting to Max Priority Queue{}", YELLOW, RESET);
            demonstrate_priority_queue(QueueType::Max)
        }
    }
}

enum QueueType {
    Max,
    Min,
}

fn demonstrate_priority_queue(queue_type: QueueType) {
    let type_name = match queue_type {
        QueueType::Max => "Max",
        QueueType::Min => "Min",
    };

    println!("\n{}{}{} Priority Queue Example:{}", BOLD, GREEN, type_name, RESET);
    println!("Creating a {} priority queue", type_name.to_lowercase());

    let mut pq: PriorityQueue<i32> = match queue_type {
        QueueType::Max => PriorityQueue::new(),
        QueueType::Min => PriorityQueue::new_min(),
    };

    loop {
        println!("\nOperations:");
        println!("1. Enqueue (add) an element");
        println!("2. Dequeue (remove) highest priority element");
        println!("3. Peek at highest priority element");
        println!("4. Check if queue is empty");
        println!("5. Get queue length");
        println!("6. Visualize the queue");
        println!("0. Exit demo");

        let mut op = String::new();
        io::stdin().read_line(&mut op).expect("Failed to read line");

        match op.trim() {
            "1" => {
                println!("Enter an integer to enqueue:");
                let mut val = String::new();
                io::stdin().read_line(&mut val).expect("Failed to read line");
                if let Ok(num) = val.trim().parse::<i32>() {
                    pq.enqueue(num);
                    println!("{}Enqueued: {}{}", GREEN, num, RESET);
                    visualize_queue(&pq);
                } else {
                    println!("{}Invalid input. Please enter an integer.{}", YELLOW, RESET);
                }
            },
            "2" => {
                match pq.dequeue() {
                    Some(val) => {
                        println!("{}Dequeued: {}{}", GREEN, val, RESET);
                        visualize_queue(&pq);
                    },
                    None => println!("{}Queue is empty{}", YELLOW, RESET)
                }
            },
            "3" => {
                match pq.peek() {
                    Some(val) => println!("{}Highest priority element: {}{}", GREEN, val, RESET),
                    None => println!("{}Queue is empty{}", YELLOW, RESET)
                }
            },
            "4" => {
                println!("{}Queue is {}{}",
                         if pq.is_empty() { GREEN } else { YELLOW },
                         if pq.is_empty() { "empty" } else { "not empty" },
                         RESET);
            },
            "5" => {
                println!("{}Queue length: {}{}", GREEN, pq.len(), RESET);
            },
            "6" => {
                visualize_queue(&pq);
            },
            "0" => break,
            _ => println!("{}Invalid option{}", YELLOW, RESET)
        }
    }
}

fn visualize_queue<T: Ord + std::fmt::Display>(pq: &PriorityQueue<T>) {
    if pq.is_empty() {
        println!("{}Queue is empty{}", YELLOW, RESET);
        return;
    }

    println!("{}Queue visualization (as array):{}", CYAN, RESET);

    let mut temp_pq = unsafe { std::ptr::read(pq) };
    let mut elements = Vec::new();

    while let Some(element) = temp_pq.dequeue() {
        elements.push(element);
    }

    std::mem::forget(temp_pq);

    if elements.is_empty() {
        return;
    }

    print!("[");
    for (i, element) in elements.iter().enumerate() {
        print!("{}{}{}", GREEN, element, RESET);
        if i < elements.len() - 1 {
            print!(", ");
        }
    }
    println!("]");

    println!("\n{}Note: Elements are shown in priority order. The leftmost element has the highest priority.{}",
             YELLOW, RESET);
}
