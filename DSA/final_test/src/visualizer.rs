use std::fmt;
use std::thread;
use std::time::Duration;
use std::io;
const RESET: &str = "\x1b[0m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const MAGENTA: &str = "\x1b[35m";
const CYAN: &str = "\x1b[36m";
const BOLD: char = '\x1B';

pub enum SortingAlgorithm {
    Selection,
    Insertion,
    Bubble,
    Merge,
    Quick,
    Bucket,
}

pub enum DataStructures{
    Heap,
}

pub struct Visualizer {
    delay_ms: u64,
    show_steps: bool,
    algorithm: SortingAlgorithm,
    structure: DataStructures,
}

impl Visualizer {

    pub fn new(delay_ms: u64, show_steps: bool, algorithm: SortingAlgorithm) -> Self {
        Visualizer {
            delay_ms,
            show_steps,
            algorithm,
            structure: DataStructures::Heap
        }
    }

    pub fn new_structure(delay_ms: u64, show_steps: bool, structure: DataStructures) -> Self {
        Visualizer {
            delay_ms,
            show_steps,
            algorithm: SortingAlgorithm::Selection,
            structure
        }
    }

    pub fn visualize<F>(&self, array: &[i32], sort_fn: F)
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let len = array.len();

        println!("{}Starting visualization: {}{}", CYAN, self.algorithm_name(), RESET);
        println!("Initial array: {}", Self::format_array(array, &[], &[]));

        let result = if self.show_steps {
            self.visualize_with_steps(array, &sort_fn)
        } else {
            sort_fn(array, len)
        };

        println!("\n{}Final result:{}", CYAN, RESET);
        println!("Before: {}", Self::format_array(array, &[], &[]));
        println!("After:  {}", Self::format_array(&result, &[], &[]));

        println!("\n{}Visualization complete!{}", GREEN, RESET);
    }

    fn algorithm_name(&self) -> String {
        match self.algorithm {
            SortingAlgorithm::Selection => "Selection Sort".to_string(),
            SortingAlgorithm::Insertion => "Insertion Sort".to_string(),
            SortingAlgorithm::Bubble => "Bubble Sort".to_string(),
            SortingAlgorithm::Merge => "Merge Sort".to_string(),
            SortingAlgorithm::Quick => "Quick Sort".to_string(),
            SortingAlgorithm::Bucket => "Bucket Sort".to_string(),
        }
    }

    fn visualize_with_steps<F>(&self, array: &[i32], sort_fn: &F) -> Vec<i32>
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let mut result = array.to_vec();
        let len = array.len();
        let mut prev_result = array.to_vec();

        match self.algorithm {
            SortingAlgorithm::Selection => self.visualize_selection(array, sort_fn),
            SortingAlgorithm::Insertion => self.visualize_insertion(array, sort_fn),
            SortingAlgorithm::Bubble => self.visualize_bubble(array, sort_fn),
            SortingAlgorithm::Merge => self.visualize_merge(array, sort_fn),
            SortingAlgorithm::Quick => self.visualize_quick(array, sort_fn),
            SortingAlgorithm::Bucket => self.visualize_bucket(array, sort_fn),
        }
    }

    fn visualize_selection<F>(&self, array: &[i32], sort_fn: &F) -> Vec<i32>
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let len = array.len();
        let mut result = array.to_vec();

        for i in 0..len {
            let partial_sort = sort_fn(&array[0..i+1], i+1);

            let mut comparing = Vec::new();
            for j in i+1..len {
                comparing.push(j);
            }

            for j in 0..i+1 {
                result[j] = partial_sort[j];
            }

            let mut highlights = vec![i];

            print!("\r\x1B[K");
            print!("Step {}/{}: ", i+1, len);
            println!("{}", Self::format_array(&result, &highlights, &comparing));

            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        result
    }

    fn visualize_insertion<F>(&self, array: &[i32], sort_fn: &F) -> Vec<i32>
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let len = array.len();
        let mut result = array.to_vec();

        for i in 1..=len {
            let partial_sort = sort_fn(&array[0..i], i);

            for j in 0..i {
                result[j] = partial_sort[j];
            }

            let highlights = vec![i-1];

            let mut comparing = Vec::new();
            for j in 0..i-1 {
                if j < i-1 && partial_sort[j] > array[i-1] {
                    comparing.push(j);
                }
            }

            print!("\r\x1B[K");
            print!("Step {}/{}: ", i, len);
            println!("{}", Self::format_array(&result, &highlights, &comparing));

            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        result
    }

    fn visualize_bubble<F>(&self, array: &[i32], sort_fn: &F) -> Vec<i32>
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let len = array.len();
        let mut result = array.to_vec();
        let mut prev_result = array.to_vec();

        for i in 1..=len {
            let partial_sort = sort_fn(&array[0..i], i);

            let mut swapped = Vec::new();
            for j in 0..i {
                if j < prev_result.len() && partial_sort[j] != prev_result[j] {
                    swapped.push(j);
                    if j+1 < i && !swapped.contains(&(j+1)) {
                        swapped.push(j+1);
                    }
                }
            }

            let mut highlights = vec![i-1];

            for j in 0..i {
                result[j] = partial_sort[j];
            }

            print!("\r\x1B[K");
            print!("Step {}/{}: ", i, len);
            println!("{}", Self::format_array(&result, &highlights, &swapped));

            prev_result = result.clone();
            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        result
    }

    fn visualize_merge<F>(&self, array: &[i32], sort_fn: &F) -> Vec<i32>
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let len = array.len();
        let mut result = array.to_vec();
        let mut prev_result = array.to_vec();

        let mut width = 1;
        let mut step = 1;

        while width < len {
            println!("\n{}Merging subarrays of size {}{}", BLUE, width, RESET);

            let mut i = 0;
            while i < len {
                let left = i;
                let mid = (i + width).min(len);
                let right = (i + 2 * width).min(len);

                if mid < right {
                    let partial_sort = sort_fn(&array[0..right], right);

                    let mut merged = Vec::new();
                    for j in left..right {
                        if j < prev_result.len() && partial_sort[j] != prev_result[j] {
                            merged.push(j);
                        }
                        result[j] = partial_sort[j];
                    }

                    let highlights = vec![left, mid-1, right-1];

                    print!("\r\x1B[K");
                    print!("Step {}: Merging [{}, {}] and [{}, {}]: ",
                           step, left, mid-1, mid, right-1);
                    println!("{}", Self::format_array(&result, &highlights, &merged));

                    prev_result = result.clone();
                    step += 1;
                    thread::sleep(Duration::from_millis(self.delay_ms));
                }

                i += 2 * width;
            }

            width *= 2;
        }

        result
    }

    fn visualize_quick<F>(&self, array: &[i32], sort_fn: &F) -> Vec<i32>
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let mut result = array.to_vec();
        let len = array.len();

        println!("Initial array: {}", Self::format_array(&result, &[], &[]));

        self.quick_sort_visualize(&mut result, 0, len - 1, 1);
        result
    }

    fn quick_sort_visualize(&self, array: &mut Vec<i32>, low: usize, high: usize, mut step: usize) -> usize {
        if low < high {
            let pivot_index = self.partition_visualize(array, low, high, step);
            step += 1;

            if pivot_index > 0 && low < pivot_index {
                step = self.quick_sort_visualize(array, low, pivot_index - 1, step);
            }

            step = self.quick_sort_visualize(array, pivot_index + 1, high, step);
        }

        step
    }

    fn visualize_bucket<F>(&self, array: &[i32], sort_fn: &F) -> Vec<i32>
    where
        F: Fn(&[i32], usize) -> Vec<i32>,
    {
        let len = array.len();
        if len <= 1 {
            return array.to_vec();
        }

        let max_val = *array.iter().max().unwrap();
        let min_val = *array.iter().min().unwrap();
        let range = (max_val - min_val) as f64;

        let mut bucket = vec![vec![]; len];

        println!("\n{}Starting Bucket Sort{}", CYAN, RESET);
        println!("Initial array: {}", Self::format_array(array, &[], &[]));
        println!("Min value: {}, Max value: {}, Range: {}", min_val, max_val, range);
        thread::sleep(Duration::from_millis(self.delay_ms));

        println!("\n{}Step 1: Distributing elements into buckets{}", BLUE, RESET);
        for (i, &num) in array.iter().enumerate() {
            let index = ((num - min_val) as f64 / (range + 1.0) * (len as f64 - 1.0)) as usize;
            println!("Element {} ({}) goes to bucket {}", i, num, index);
            bucket[index].push(num);

            self.display_buckets(&bucket);
            thread::sleep(Duration::from_millis(self.delay_ms));
        }

        println!("\n{}Step 2: Sorting individual buckets{}", BLUE, RESET);
        for i in 0..len {
            if !bucket[i].is_empty() {
                println!("Sorting bucket {}: {:?}", i, bucket[i]);

                let bucket_len = bucket[i].len();
                for j in 1..bucket_len {
                    let key = bucket[i][j];
                    let mut k = j;
                    while k > 0 && bucket[i][k - 1] > key {
                        bucket[i][k] = bucket[i][k - 1];
                        k -= 1;
                    }
                    bucket[i][k] = key;

                    println!("After insertion step {}: {:?}", j, bucket[i]);
                    self.display_buckets(&bucket);
                    thread::sleep(Duration::from_millis(self.delay_ms));
                }
            }
        }

        println!("\n{}Step 3: Concatenating buckets{}", BLUE, RESET);
        let mut result = Vec::new();
        for i in 0..len {
            if !bucket[i].is_empty() {
                println!("Adding bucket {} to result: {:?}", i, bucket[i]);
                result.extend(bucket[i].iter());

                println!("Current result: {}", Self::format_array(&result, &[], &[]));
                thread::sleep(Duration::from_millis(self.delay_ms));
            }
        }

        println!("\n{}Bucket Sort complete!{}", GREEN, RESET);
        result
    }

    fn display_buckets(&self, buckets: &[Vec<i32>]) {
        println!("Current buckets:");
        for (i, bucket) in buckets.iter().enumerate() {
            if !bucket.is_empty() {
                println!("  Bucket {}: {:?}", i, bucket);
            } else {
                println!("  Bucket {}: []", i);
            }
        }
        println!();
        thread::sleep(Duration::from_millis(self.delay_ms / 2));
    }

    fn partition_visualize(&self, array: &mut Vec<i32>, low: usize, high: usize, step: usize) -> usize {
        let pivot = array[high];
        let mut i = low;

        print!("\r\x1B[K");
        print!("Step {}: Partitioning [{}, {}], pivot value {}: ",
               step, low, high, pivot);
        println!("{}", Self::format_array(array, &[high], &[]));

        thread::sleep(Duration::from_millis(self.delay_ms));

        for j in low..high {
            if array[j] <= pivot {
                print!("\r\x1B[K");
                print!("Step {}.{}: Comparing {} <= {}: ",
                       step, j-low+1, array[j], pivot);
                println!("{}", Self::format_array(array, &[high], &[j, i]));

                thread::sleep(Duration::from_millis(self.delay_ms));

                if i != j {
                    array.swap(i, j);

                    print!("\r\x1B[K");
                    print!("Step {}.{}: Swapping {} and {}: ",
                           step, j-low+1, array[i], array[j]);
                    println!("{}", Self::format_array(array, &[high], &[i, j]));

                    thread::sleep(Duration::from_millis(self.delay_ms));
                }

                i += 1;
            }
        }

        print!("\r\x1B[K");
        print!("Step {}.{}: Placing pivot in final position: ", step, high-low+1);
        println!("{}", Self::format_array(array, &[high, i], &[]));

        array.swap(i, high);

        print!("\r\x1B[K");
        print!("Step {}.{}: After partition: ", step, high-low+2);
        println!("{}", Self::format_array(array, &[i], &[]));

        thread::sleep(Duration::from_millis(self.delay_ms));

        i
    }

    pub fn visualize_heap<T: fmt::Display + Ord>(&self, heap: &mut crate::Heap<T, impl Fn(&T, &T) -> bool>) {
        println!("{}Starting Heap visualization{}", CYAN, RESET);
        self.display_heap(heap, None, None);
    }

    pub fn visualize_heap_push<T: fmt::Display + Ord + Clone>(
        &self,
        heap: &mut crate::Heap<T, impl Fn(&T, &T) -> bool>,
        value: T
    ) {
        println!("\n{}Pushing value: {}{}", CYAN, value, RESET);
        println!("Before push:");
        self.display_heap(heap, None, None);

        let original_len = heap.len();

        heap.push(value.clone());

        println!("\nAfter push:");
        self.display_heap(heap, Some(heap.len() - 1), None);

        self.after_heap_operation(&heap.data, "Push");
    }

    pub fn visualize_heap_pop<T: fmt::Display + Ord + Clone>(
        &self,
        heap: &mut crate::Heap<T, impl Fn(&T, &T) -> bool>
    ) -> Option<T> {
        println!("\n{}Popping value from heap{}", CYAN, RESET);
        println!("Before pop:");
        self.display_heap(heap, None, Some(0));

        let popped = heap.pop();

        match &popped {
            Some(value) => println!("\n{}Popped value: {}{}", YELLOW, value, RESET),
            None => println!("\n{}Heap is empty{}", RED, RESET),
        }

        println!("\nAfter pop:");
        self.display_heap(heap, None, None);

        println!("\n{}Pop operation complete!{}", GREEN, RESET);

        popped
    }

    fn display_heap<T: fmt::Display + Ord>(
        &self,
        heap: &crate::Heap<T, impl Fn(&T, &T) -> bool>,
        highlight_index: Option<usize>,
        highlight_root: Option<usize>
    ) {
        if heap.is_empty() {
            println!("{}Heap is empty{}", YELLOW, RESET);
            return;
        }

        let data = &heap.data;

        let depth = (data.len() as f64).log2().floor() as usize + 1;

        self.display_heap_as_tree(data, depth, highlight_index, highlight_root);

        println!("\nHeap array representation:");
        print!("[");
        for (i, item) in data.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }

            if Some(i) == highlight_index {
                print!("{}{}{}", GREEN, item, RESET);
            } else if Some(i) == highlight_root {
                print!("{}{}{}", MAGENTA, item, RESET);
            } else {
                print!("{}", item);
            }
        }
        println!("]");

        thread::sleep(Duration::from_millis(self.delay_ms));
    }

    fn display_heap_as_tree<T: fmt::Display>(
        &self,
        data: &[T],
        depth: usize,
        highlight_index: Option<usize>,
        highlight_root: Option<usize>
    ) {
        println!("\nHeap tree representation (horizontal):");

        if data.is_empty() {
            return;
        }

        self.print_node_horizontal(data, 0, 0, highlight_index, highlight_root);
    }

    fn print_node_horizontal<T: fmt::Display>(
        &self,
        data: &[T],
        idx: usize,
        level: usize,
        highlight_index: Option<usize>,
        highlight_root: Option<usize>
    ) {
        if idx >= data.len() {
            return;
        }

        let right_child = 2 * idx + 2;
        if right_child < data.len() {
            self.print_node_horizontal(data, right_child, level + 1, highlight_index, highlight_root);
        }

        for _ in 0..level {
            print!("    ");
        }

        if level > 0 {
            print!("└── ");
        }

        if Some(idx) == highlight_index {
            print!("{}{}{}\n", GREEN, data[idx], RESET);
        } else if Some(idx) == highlight_root {
            print!("{}{}{}\n", MAGENTA, data[idx], RESET);
        } else {
            print!("{}\n", data[idx]);
        }

        let left_child = 2 * idx + 1;
        if left_child < data.len() {
            self.print_node_horizontal(data, left_child, level + 1, highlight_index, highlight_root);
        }
    }

    fn format_array(arr: &[i32], highlighted: &[usize], comparing: &[usize]) -> String {
        let mut result = String::from("[");
        for (i, &num) in arr.iter().enumerate() {
            if i > 0 {
                result.push_str(", ");
            }

            if highlighted.contains(&i) {
                result.push_str(&format!("{}{}{}", GREEN, num, RESET));
            } else if comparing.contains(&i) {
                result.push_str(&format!("{}{}{}", YELLOW, num, RESET));
            } else {
                result.push_str(&num.to_string());
            }
        }
        result.push(']');
        result
    }

    fn after_heap_operation<T: fmt::Display>(&self, data: &[T], operation_name: &str) {
        println!("\n{}Heap array representation:{}", BOLD, RESET);

        print!("[");
        for (i, item) in data.iter().enumerate() {
            if i > 0 {
                print!(", ");
            }
            print!("{}", item);
        }
        println!("]");

        println!("\n{}{} operation complete!{}", GREEN, operation_name, RESET);

        println!("\nWould you like to learn more about heap data structures?");
        println!("1. Yes - Show heap theory");
        println!("2. No - Continue with operations");

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        match choice.trim() {
            "1" => {
                crate::theories::heap::display_heap_theory();
            },
            _ => {
                println!("Continuing with heap operations...");
            }
        }
    }
}

fn fill_grid<'a, T: fmt::Display>(
    data: &'a [T],
    idx: usize,
    col: usize,
    row: usize,
    grid: &mut Vec<Vec<Option<(usize, &'a T)>>>
) {
    if idx >= data.len() || col >= grid.len() || row >= grid[0].len() || row < 2 {
        return;
    }

    grid[col][row] = Some((idx, &data[idx]));

    let left_idx = 2 * idx + 1;
    let right_idx = 2 * idx + 2;

    if left_idx < data.len() && col+1 < grid.len() && row >= 2 {
        if col+1 < grid.len() && row-1 < grid[0].len() {
            grid[col+1][row-1] = None;
        }
        if col+2 < grid.len() && row-2 < grid[0].len() {
            fill_grid(data, left_idx, col+2, row-2, grid);
        }
    }

    if right_idx < data.len() && col+1 < grid.len() && row+1 < grid[0].len() {
        if col+1 < grid.len() && row+1 < grid[0].len() {
            grid[col+1][row+1] = None;
        }
        if col+2 < grid.len() && row+2 < grid[0].len() {
            fill_grid(data, right_idx, col+2, row+2, grid);
        }
    }
}

fn has_connection<T: fmt::Display>(
    data: &[T],
    row: usize,
    col: usize,
    grid: &Vec<Vec<Option<(usize, &T)>>>
) -> bool {
    if col + 1 >= grid.len() {
        return false;
    }

    let has_node_before = col > 0 && grid[col-1][row-1].is_some() || grid[col-1][row+1].is_some();
    let has_node_after = grid[col+1][row-1].is_some() || grid[col+1][row+1].is_some();

    has_node_before && has_node_after
}

