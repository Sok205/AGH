use std::io;
use crate::display::{BOLD, CYAN, GREEN, YELLOW, RED, BLUE, RESET};
use rand::{seq::SliceRandom, rngs::StdRng, SeedableRng};
use std::time::{SystemTime, UNIX_EPOCH};

struct Question {
    prompt: String,
    options: Vec<String>,
    correct_answer: usize,
    explanation: String,
}

pub fn run_time_complexity_quiz() {
    println!("\n{}{}Time Complexity Quiz{}", BOLD, CYAN, RESET);
    println!("Test your knowledge of algorithmic time complexities.");
    println!("Choose the correct answer for each question.");

    let now = SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards");
    let seconds_in_hour = 3600;
    let hour_seed = now.as_secs() / seconds_in_hour;

    let mut questions = get_quiz_questions();
    let mut rng = StdRng::seed_from_u64(hour_seed);
    questions.shuffle(&mut rng);

    let mut score = 0;
    let total = questions.len();

    for (i, question) in questions.iter().enumerate() {
        println!("\n{}Question {}/{}: {}{}", BOLD, i + 1, total, question.prompt, RESET);

        for (j, option) in question.options.iter().enumerate() {
            println!("{}. {}", j + 1, option);
        }

        println!("\nYour answer (1-{}): ", question.options.len());

        let mut user_answer = String::new();
        io::stdin()
            .read_line(&mut user_answer)
            .expect("Failed to read line");

        let user_choice = match user_answer.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= question.options.len() => num,
            _ => {
                println!("{}Invalid choice. Moving to next question.{}", YELLOW, RESET);
                continue;
            }
        };

        if user_choice == question.correct_answer {
            println!("{}Correct!{}", GREEN, RESET);
            score += 1;
        } else {
            println!("{}Incorrect. The correct answer is: {}{}",
                     RED, question.options[question.correct_answer - 1], RESET);
        }

        println!("{}Explanation: {}{}", BLUE, question.explanation, RESET);

        println!("\nPress Enter to continue...");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).expect("Failed to read line");
    }

    let percentage = (score as f32 / total as f32) * 100.0;

    println!("\n{}{}Quiz Complete!{}", BOLD, CYAN, RESET);
    println!("Your score: {}/{} ({}%)", score, total, percentage);

    match percentage {
        p if p >= 90.0 => println!("{}Excellent! You have mastered time complexity concepts!{}", GREEN, RESET),
        p if p >= 70.0 => println!("{}Good job! You have a solid understanding of time complexities.{}", GREEN, RESET),
        p if p >= 50.0 => println!("{}Not bad, but there's room for improvement.{}", YELLOW, RESET),
        _ => println!("{}You might want to review the time complexity concepts again.{}", RED, RESET),
    }
}

fn get_quiz_questions() -> Vec<Question> {
    let questions = vec![
        Question {
            prompt: "What is the time complexity of extracting the minimum element from a min heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Extracting the minimum element requires removing the root and rebalancing the heap, which takes O(log n) time.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of Quick Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Quick Sort's worst-case occurs when the pivot selection consistently results in highly unbalanced partitions, leading to O(n²) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the best-case time complexity of Insertion Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In the best case (when the array is already sorted), Insertion Sort only needs to scan through the array once, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the average-case time complexity of Merge Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(n³)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Merge Sort consistently divides the array in half and merges the sorted halves, resulting in O(n log n) time complexity in all cases.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of building a heap from an array of n elements?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(log n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Although it might seem like O(n log n) since each heapify operation is O(log n), a tighter analysis shows that building a heap can be done in O(n) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of inserting an element into a binary search tree in the worst case?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In the worst case (when the tree is a linear chain), insertion into a BST is O(h) where h is the height. For an unbalanced tree, h can be n, resulting in O(n) time.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of Bubble Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Bubble Sort uses nested iterations over the array, resulting in O(n²) worst-case time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the minimum element in a min heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "In a min heap, the minimum element is always at the root, making it accessible in constant time, O(1).".to_string(),
        },
        Question {
            prompt: "What is the best-case time complexity of Quick Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In the best case, Quick Sort divides the array into roughly equal halves at each step, resulting in O(n log n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of inserting an element into an AVL tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "AVL trees maintain balance during insertions, ensuring O(log n) time complexity even in the worst case.".to_string(),
        },
        Question {
            prompt: "What is the average-case time complexity of Selection Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Selection Sort always makes n(n-1)/2 comparisons regardless of the input, resulting in O(n²) time complexity in all cases.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the heapify operation in a heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Heapify operation fixes a single violation of the heap property by moving an element down the tree, which takes O(log n) time in the worst case.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of Bucket Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(n + k)".to_string(),
            ],
            correct_answer: 3,
            explanation: "If all elements are placed in a single bucket and the sorting algorithm for that bucket is quadratic, Bucket Sort degrades to O(n²).".to_string(),
        },
        Question {
            prompt: "What is the best-case time complexity of Bucket Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n + k)".to_string(),
                "O(k)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In the best case with uniform distribution across k buckets, Bucket Sort achieves O(n + k) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of searching for an element in an AVL tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "AVL trees maintain a balanced structure, ensuring O(log n) search time even in the worst case.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'peek' operation in a priority queue implemented with a binary heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "The 'peek' operation just returns the highest priority element without removing it, which is at the root of the heap, taking O(1) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the maximum element in a max heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "In a max heap, the maximum element is always at the root, accessible in constant time, O(1).".to_string(),
        },
        Question {
            prompt: "What is the average-case time complexity of Quick Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "On average, Quick Sort divides the array into reasonably balanced partitions, resulting in O(n log n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of deleting an element from an AVL tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Deletion in an AVL tree includes finding the node (O(log n)) and rebalancing (also O(log n)), resulting in O(log n) overall time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'enqueue' operation in a priority queue implemented with a binary heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Enqueuing an element requires inserting it at the end and then bubbling it up to its correct position, which takes O(log n) time in the worst case.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of finding an element in a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In the worst case (a degenerate tree), the search path can include all nodes, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the best-case time complexity of Bubble Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 2,
            explanation: "With an optimization to detect if the list is already sorted, Bubble Sort can achieve O(n) time complexity in the best case.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of traversing all elements in a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Traversing all n nodes in a BST requires visiting each node exactly once, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'dequeue' operation in a priority queue implemented with a binary heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Dequeuing requires removing the root element and then restoring the heap property by moving the last element to the root and bubbling it down, which takes O(log n) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the Lomuto partition scheme in Quick Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "The Lomuto partition scheme requires a single pass through the subarray being partitioned, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the Hoare partition scheme in Quick Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "The Hoare partition scheme, like Lomuto, requires a single pass through the subarray, resulting in O(n) time complexity, though it generally performs fewer swaps.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of Merge Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Merge Sort requires O(n) additional space for the temporary arrays used during merging.".to_string(),
        },
        Question {
            prompt: "What is the best-case time complexity of Selection Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 4,
            explanation: "Selection Sort always performs the same number of comparisons regardless of the input arrangement, resulting in O(n²) time complexity even in the best case.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of Quick Sort in the average case?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Quick Sort uses O(log n) space for the recursion stack in the average case when the partitioning is balanced.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the median in a min-max heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the median in a standard min-max heap requires O(n) time as the elements are not stored in a way that makes the median easily accessible.".to_string(),
        },
        Question {
            prompt: "What is the worst-case space complexity of Quick Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In the worst case (highly unbalanced partitioning), the recursion depth can reach n, resulting in O(n) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of inserting n elements into an initially empty binary heap?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Inserting n elements one by one, with each insertion taking O(log i) time where i is the current heap size, results in O(n log n) total time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the Median-of-Three pivot selection in Quick Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Selecting the median of three elements (first, middle, last) takes constant time, O(1).".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of Insertion Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In the worst case (reversed order), each element must be compared with all previous elements, resulting in O(n²) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of rebalancing an AVL tree after an insertion?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Rebalancing requires at most O(log n) rotations and updates, as the height of an AVL tree is O(log n).".to_string(),
        },
        Question {
            prompt: "What is the average-case time complexity of finding an element in a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In a randomly built BST, the expected height is O(log n), resulting in O(log n) average search time.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of Bubble Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Bubble Sort uses only a constant amount of extra space regardless of input size, resulting in O(1) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of performing an in-order traversal of a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "An in-order traversal visits all n nodes exactly once, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of using Random Pivot selection in Quick Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Selecting a random pivot element takes constant time, O(1).".to_string(),
        },
        Question {
            prompt: "What is the space complexity of an AVL tree for storing n elements?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "An AVL tree with n elements requires O(n) space to store all the nodes.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the minimum element in a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the minimum requires following left child pointers until reaching a leaf, which takes O(h) time where h is the height. In the worst case, h = n.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the minimum element in a balanced BST (like AVL)?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In a balanced BST, the height is guaranteed to be O(log n), so finding the minimum takes O(log n) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the kth smallest element in a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(k)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Without additional data structures, finding the kth smallest requires an in-order traversal until reaching the kth element, taking O(n) time in the worst case.".to_string(),
        },
        Question {
            prompt: "What is the worst-case space complexity of Merge Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Merge Sort requires O(n) extra space for the temporary arrays used during merging.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of heapsort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Heapsort involves building a heap (O(n)) and extracting each element (n extractions at O(log n) each), resulting in O(n log n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'isEmpty' operation in a priority queue?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "The 'isEmpty' operation simply checks if the queue has any elements, which takes constant time, O(1).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'getLength' operation in a priority queue?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "The 'getLength' operation simply returns the number of elements, which is typically tracked as a variable, taking O(1) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'merge' operation in Merge Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "The merge operation combines two sorted subarrays of size n/2 each, requiring O(n) time to merge them.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of Selection Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Selection Sort only needs a constant amount of extra space regardless of input size, resulting in O(1) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of Insertion Sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Insertion Sort operates in-place with only a constant amount of extra space, resulting in O(1) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the average-case time complexity of Bucket Sort with k buckets?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n + k)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 3,
            explanation: "With a good hash function and uniform distribution, Bucket Sort achieves O(n + k) average-case time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of AVL tree rotation operations?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "AVL tree rotations (single or double) involve only a constant number of pointer reassignments, resulting in O(1) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the average-case time complexity of Insertion Sort?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "On average, each element must be compared with approximately half of the already sorted elements, resulting in O(n²) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of a binary heap for storing n elements?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "A binary heap requires O(n) space to store all n elements.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding an element in a priority queue implemented with an unsorted array?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding a specific element in an unsorted array requires checking each element in the worst case, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'dequeue' operation in a priority queue implemented with a sorted array?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "In a sorted array implementation, the highest priority element is at one end of the array, making dequeue an O(1) operation.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'enqueue' operation in a priority queue implemented with a sorted array?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In a sorted array, inserting a new element requires finding the correct position (which could be done in O(log n) time with binary search) and then shifting elements to make space, resulting in O(n) time overall.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'peek' operation in a priority queue implemented with a Fibonacci heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Fibonacci heaps maintain a pointer to the minimum element, allowing O(1) access to the highest priority element.".to_string(),
        },
        Question {
            prompt: "What is the amortized time complexity of the 'enqueue' operation in a priority queue implemented with a Fibonacci heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Fibonacci heaps provide O(1) amortized time for enqueue operations, which is one of their key advantages.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of a BST traversal if the tree height is h?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(h)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "A complete traversal visits all n nodes, resulting in O(n) time complexity regardless of the height.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of computing the height of a binary tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Computing the height requires visiting all nodes in the worst case, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the in-order successor operation in a BST?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(h)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the in-order successor takes O(h) time where h is the height of the tree. In the worst case for an unbalanced tree, h can be n.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the in-order successor operation in an AVL tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In an AVL tree, the height is guaranteed to be O(log n), so finding the in-order successor takes O(log n) time.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of an in-order traversal of a binary tree if implemented recursively?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(h)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "A recursive in-order traversal uses stack space proportional to the height of the tree, which is O(h). In the worst case, h can be n.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of an in-order traversal of an AVL tree if implemented recursively?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "A recursive in-order traversal uses stack space proportional to the height, which is O(log n) for an AVL tree.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'enqueue' operation in a priority queue implemented with an unsorted array?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Inserting into an unsorted array just requires adding the element at the end, which takes constant time, O(1).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'dequeue' operation in a priority queue implemented with an unsorted array?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the highest priority element in an unsorted array requires scanning all elements, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of Quick Sort if the pivot is always the smallest element?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "If the pivot is always the smallest element, one partition will always be empty and the other will have n-1 elements, leading to a recursion depth of n and O(n²) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of building a balanced BST from a sorted array?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Building a balanced BST from a sorted array can be done in O(n) time by recursively selecting the middle element as the root.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of Quick Sort in the best case?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In the best case (balanced partitioning), Quick Sort's recursion depth is O(log n), resulting in O(log n) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of deleting the minimum element from a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(h)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Deleting the minimum requires finding it (O(h)) and then performing the deletion (O(1)), resulting in O(h) overall. In the worst case, h can be n.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of deleting the minimum element from an AVL tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In an AVL tree, finding and deleting the minimum takes O(log n) time because the height is guaranteed to be O(log n).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the second largest element in a BST?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(h)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the second largest requires finding the largest (O(h)) and then its predecessor, resulting in O(h) overall. In the worst case, h can be n.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the second largest element in an AVL tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In an AVL tree, finding the second largest takes O(log n) time due to the guaranteed O(log n) height.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the Lomuto partition scheme in the worst case?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 3,
            explanation: "The Lomuto partition scheme always requires a single pass through the array, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of the Lomuto partition scheme?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "The Lomuto partition scheme operates in-place with only a constant amount of extra space, resulting in O(1) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the Hoare partition scheme in the worst case?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 3,
            explanation: "The Hoare partition scheme always requires a single pass through the array, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of the Hoare partition scheme?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "The Hoare partition scheme operates in-place with only a constant amount of extra space, resulting in O(1) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of checking if a binary tree is a valid binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Validating a BST requires visiting each node once and checking its value against appropriate bounds, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of converting a binary search tree to a sorted array?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Converting a BST to a sorted array is equivalent to an in-order traversal, which takes O(n) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of constructing a heap from an unsorted array using the heapify approach?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "The heapify approach builds a heap in O(n) time by starting from the bottom and working up.".to_string(),
        },
        Question {
            prompt: "What is the space complexity of heap sort?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Heap sort operates in-place with only a constant amount of extra space, resulting in O(1) space complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of reversing a binary search tree (turning it into a 'greater than' tree)?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Reversing a BST requires visiting each node once and swapping its children, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the lowest common ancestor (LCA) in a binary search tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(h)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the LCA in a BST takes O(h) time where h is the height. In the worst case for an unbalanced tree, h can be n.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the lowest common ancestor (LCA) in an AVL tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In an AVL tree, finding the LCA takes O(log n) time due to the guaranteed O(log n) height.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the diameter of a binary tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the diameter can be done in a single tree traversal with appropriate height calculations, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of checking if a binary tree is balanced?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Checking if a tree is balanced requires computing the height of each subtree, which can be done in O(n) time with a bottom-up approach.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of the merge operation in Merge Sort if the arrays are already in memory?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "The merge operation always requires examining each element in both subarrays, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding all elements in a BST that fall within a given range?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(k + log n)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding elements in a range requires O(log n) to find the first element in the range, plus O(k) to visit all k elements in the range. In the worst case, all elements could be in the range, making it O(n).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the kth largest element in a BST without additional data structures?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(h + k)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Finding the kth largest requires a reverse in-order traversal until reaching the kth element, which takes O(h + k) time. In the worst case, h can be n.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of checking if a binary tree is symmetric?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Checking if a tree is symmetric requires visiting each node once and comparing corresponding nodes in the left and right subtrees, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of converting a sorted array to a height-balanced BST?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Converting a sorted array to a balanced BST requires visiting each element once and creating a node for it, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of counting the number of elements in a BST that are greater than a given value?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(h)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 4,
            explanation: "In the worst case, counting elements greater than a value might require visiting all nodes, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'decrease key' operation in a binary heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Decreasing a key might violate the heap property, requiring the element to bubble up, which takes O(log n) time in the worst case.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'increase key' operation in a binary heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Increasing a key might violate the heap property, requiring the element to bubble down, which takes O(log n) time in the worst case.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'decrease key' operation in a Fibonacci heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "Fibonacci heaps support O(1) amortized time for the 'decrease key' operation, which is one of their key advantages.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of deleting an arbitrary node (not necessarily the min/max) from a binary heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Deleting an arbitrary node requires finding it (O(n) in worst case) and then performing the removal and reheapify operations (O(log n)), but in practice with a supplementary data structure to track node positions, the overall operation can be O(log n).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of sorting an array using a binary heap?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n²)".to_string(),
                "O(2^n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Heap sort requires building a heap (O(n)) and then extracting each element (n extractions at O(log n) each), resulting in O(n log n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of Bucket Sort with k buckets?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(n + k)".to_string(),
                "O(n^2)".to_string(),
            ],
            correct_answer: 4,
            explanation: "If all elements are placed in a single bucket and the sorting algorithm for that bucket is quadratic, Bucket Sort degrades to O(n²).".to_string(),
        },
        Question {
            prompt: "What is the space complexity of Bucket Sort with k buckets?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(n)".to_string(),
                "O(n + k)".to_string(),
                "O(n²)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Bucket Sort requires O(n + k) space for the n elements and k buckets.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of determining if a binary tree is a heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Checking if a tree is a heap requires verifying both the complete tree property and the heap property, which requires visiting each node once, resulting in O(n) time complexity.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the maximum element in a min heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In a min heap, the maximum element could be any of the leaf nodes, requiring checking all of them, which takes O(n) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of finding the minimum element in a max heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "In a max heap, the minimum element could be any of the leaf nodes, requiring checking all of them, which takes O(n) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of merging two binary heaps into a single heap?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Merging two heaps can be done by creating a new heap from all elements, which takes O(n) time using the heapify approach.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of merging two AVL trees?".to_string(),
            options: vec![
                "O(n)".to_string(),
                "O(n log n)".to_string(),
                "O(m + n)".to_string(),
                "O(m log n)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Merging two AVL trees of sizes m and n can be done in O((m+n) log(m+n)) time by converting to sorted arrays, merging, and rebuilding.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of the 'delete' operation in a hash table with chaining for collision resolution?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(k)".to_string(),
                "O(n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "With chaining, deletion requires finding the element in the appropriate chain, which takes O(k) time where k is the chain length. In the worst case, all elements hash to the same bucket, making k = n.".to_string(),
        },
        Question {
            prompt: "What is the expected time complexity of the 'delete' operation in a hash table with chaining, assuming good hash function?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 1,
            explanation: "With a good hash function and load factor, the expected chain length is constant, making deletion an O(1) operation on average.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of deleting a node from a splay tree?".to_string(),
            options: vec![
                "O(1)".to_string(),
                "O(log n)".to_string(),
                "O(n)".to_string(),
                "O(n log n)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Deletion in a splay tree requires first splaying the node to the root (which takes amortized O(log n) time) and then performing a BST delete operation, resulting in amortized O(log n) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of Dijkstra's algorithm using a binary heap for a graph with V vertices and E edges?".to_string(),
            options: vec![
                "O(V^2)".to_string(),
                "O(E log V)".to_string(),
                "O(V log E)".to_string(),
                "O(E + V)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Dijkstra's with a binary heap runs in O(E log V) time, as each edge may cause a decrease-key operation.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of Dijkstra's algorithm using an array for a graph with V vertices?".to_string(),
            options: vec![
                "O(V^2)".to_string(),
                "O(E log V)".to_string(),
                "O(V log V)".to_string(),
                "O(E + V)".to_string(),
            ],
            correct_answer: 1,
            explanation: "With an array, finding the minimum takes O(V) per vertex, so total time is O(V^2).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of Prim's algorithm using a binary heap for a graph with V vertices and E edges?".to_string(),
            options: vec![
                "O(V^2)".to_string(),
                "O(E log V)".to_string(),
                "O(V log V)".to_string(),
                "O(E + V)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Prim's with a binary heap is O(E log V), similar to Dijkstra's.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of Kruskal's algorithm for a graph with V vertices and E edges?".to_string(),
            options: vec![
                "O(E log V)".to_string(),
                "O(E log E)".to_string(),
                "O(V^2)".to_string(),
                "O(E + V)".to_string(),
            ],
            correct_answer: 2,
            explanation: "Kruskal's algorithm sorts all edges (O(E log E)) and uses union-find (almost linear).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of Bellman-Ford algorithm for a graph with V vertices and E edges?".to_string(),
            options: vec![
                "O(V^2)".to_string(),
                "O(E log V)".to_string(),
                "O(VE)".to_string(),
                "O(E + V)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Bellman-Ford relaxes all E edges for V-1 passes: O(VE).".to_string(),
        },
        Question {
            prompt: "What is the average-case time complexity of the Rabin-Karp string matching algorithm for a text of length n and pattern of length m?".to_string(),
            options: vec![
                "O(n + m)".to_string(),
                "O(nm)".to_string(),
                "O(n)".to_string(),
                "O(m^2)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Rabin-Karp is O(n + m) on average due to efficient rolling hash and rare collisions.".to_string(),
        },
        Question {
            prompt: "What is the worst-case time complexity of the Rabin-Karp string matching algorithm for a text of length n and pattern of length m?".to_string(),
            options: vec![
                "O(n + m)".to_string(),
                "O(nm)".to_string(),
                "O(n)".to_string(),
                "O(m^2)".to_string(),
            ],
            correct_answer: 2,
            explanation: "In the worst case (many hash collisions), Rabin-Karp is O(nm).".to_string(),
        },
        Question {
            prompt: "What is the time complexity of Dijkstra's algorithm using a Fibonacci heap for a graph with V vertices and E edges?".to_string(),
            options: vec![
                "O(E log V)".to_string(),
                "O(V^2)".to_string(),
                "O(E + V log V)".to_string(),
                "O(E + V)".to_string(),
            ],
            correct_answer: 3,
            explanation: "With a Fibonacci heap, Dijkstra's runs in O(E + V log V) time.".to_string(),
        },
        Question {
            prompt: "What is the time complexity of Prim's algorithm using a Fibonacci heap for a graph with V vertices and E edges?".to_string(),
            options: vec![
                "O(E log V)".to_string(),
                "O(V^2)".to_string(),
                "O(E + V log V)".to_string(),
                "O(E + V)".to_string(),
            ],
            correct_answer: 3,
            explanation: "Prim's with a Fibonacci heap is O(E + V log V).".to_string(),
        },
        Question {
            prompt: "What is the key technique that makes Rabin-Karp efficient for multiple pattern matching?".to_string(),
            options: vec![
                "Suffix trees".to_string(),
                "Rolling hash".to_string(),
                "Dynamic programming".to_string(),
                "Trie data structure".to_string(),
            ],
            correct_answer: 2,
            explanation: "Rabin-Karp uses a rolling hash to efficiently compute hashes for all substrings.".to_string(),
        },
    ];

    questions
}