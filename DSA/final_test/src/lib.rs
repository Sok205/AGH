pub mod sorting_algorithms {
    pub fn selection_sort(array: &[i32], len_array: usize) -> Vec<i32> {
        let mut result = array.to_vec();

        for i in 0..len_array {
            let mut min_index = i;
            for j in i + 1..len_array {
                if result[j] < result[min_index] {
                    min_index = j
                }
            }
            let temp = result[min_index];
            result[min_index] = result[i];
            result[i] = temp;
        }
        result
    }

    pub fn insertion_sort(array: &[i32], len_array: usize) -> Vec<i32> {
        let mut result = array.to_vec();

        for i in 1..len_array {
            let mut key = result[i];
            let mut j = i;
            while j > 0 && result[j - 1] > key {
                result[j] = result[j - 1];
                j = j - 1;
            }
            result[j] = key;
        }
        result
    }

    pub fn bubble_sort(array: &[i32], len_array: usize) -> Vec<i32> {
        let mut result = array.to_vec();
        for i in 0..len_array {
            for j in 0..len_array - 1 {
                if result[j] > result[j + 1] {
                    let temp = result[j];
                    result[j] = result[j + 1];
                    result[j + 1] = temp;
                }
            }
        }
        result
    }

    pub fn merge_sort(array: &[i32], len: usize) -> Vec<i32> {
        if len <= 1 {
            return array[0..len].to_vec();
        }

        let mut result = array[0..len].to_vec();

        let mut width = 1;
        while width < len {
            let mut i = 0;
            while i < len {
                let left = i;
                let mid = (i + width).min(len);
                let right = (i + 2 * width).min(len);

                if mid < right {
                    merge_subarrays(&mut result, left, mid, right);
                }

                i += 2 * width;
            }
            width *= 2;
        }

        result
    }

    fn merge_subarrays(array: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
        let left_part = array[left..mid].to_vec();
        let right_part = array[mid..right].to_vec();

        let mut i = 0;
        let mut j = 0;
        let mut k = left;

        while i < left_part.len() && j < right_part.len() {
            if left_part[i] <= right_part[j] {
                array[k] = left_part[i];
                i += 1;
            } else {
                array[k] = right_part[j];
                j += 1;
            }
            k += 1;
        }

        while i < left_part.len() {
            array[k] = left_part[i];
            i += 1;
            k += 1;
        }

        while j < right_part.len() {
            array[k] = right_part[j];
            j += 1;
            k += 1;
        }
    }

    pub fn quick_sort(array: &[i32], len_array: usize) -> Vec<i32> {
        let mut result = array.to_vec();
        if len_array <= 1 {
            return result;
        }

        quick_sort_helper(&mut result, 0, len_array - 1);
        result
    }

    fn quick_sort_helper(array: &mut Vec<i32>, low: usize, high: usize) {
        if low < high {
            let pivot = partition(array, low, high);

            if pivot > 0 && low < pivot {
                quick_sort_helper(array, low, pivot - 1);
            }

            quick_sort_helper(array, pivot + 1, high);
        }
    }

    fn partition(array: &mut Vec<i32>, low: usize, high: usize) -> usize {
        let pivot = array[high];
        let mut i = low;

        for j in low..high {
            if array[j] <= pivot {
                array.swap(i, j);
                i += 1;
            }
        }

        array.swap(i, high);
        i
    }
}
pub struct Heap<T, F>
where F: Fn(&T, &T) -> bool,
{
    pub(crate) data: Vec<T>,
    cmp: F,
}

impl<T, F> Heap<T, F>
where
    T: Ord,
    F: Fn(&T, &T) -> bool,
{
    pub fn new(cmp: F) -> Self {
        Self{
            data: Vec::new(),
            cmp,
        }
    }

    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.bubble_up(self.data.len() - 1);
    }

    pub fn peak(&self) -> Option<&T> {
        self.data.get(0)
    }

    fn bubble_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if !(self.cmp)(&self.data[idx], &self.data[parent]) {
                break;
            }
            self.data.swap(idx, parent);
            idx = parent;
        }
    }

    fn heapify_down(&mut self, idx: usize) {
        if self.data.is_empty() {
            return;
        }

        let len = self.data.len();
        let mut root = idx;

        loop {
            let left = 2 * root + 1;
            let right = 2 * root + 2;
            let mut largest = root;

            if left < len && (self.cmp)(&self.data[left], &self.data[largest]) {
                largest = left;
            }

            if right < len && (self.cmp)(&self.data[right], &self.data[largest]) {
                largest = right;
            }

            if largest == root {
                break;
            }

            self.data.swap(root, largest);
            root = largest;
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let len = self.data.len();
        self.data.swap(0, len - 1);
        let result = self.data.pop();

        if !self.data.is_empty() {
            self.heapify_down(0);
        }

        result
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

}

pub struct PriorityQueue<T>
where
    T: Ord,
{
    heap: Heap<T, fn(&T, &T) -> bool>,
}

impl<T> PriorityQueue<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self {
            heap: Heap::new(|a, b| a > b),
        }
    }

    pub fn new_min() -> Self {
        Self {
            heap: Heap::new(|a, b| a < b),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.heap.push(item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.heap.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.heap.peak()
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

pub struct BST<T>
where
    T: Ord
{
    root: Option<Box<Node<T>>>,
}

struct Node<T>
where
    T: Ord,
{
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> BST<T>
where
    T: Ord,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, value: T) {
        let root = &mut self.root;

        if root.is_none() {
            *root = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
            return;
        }

        Self::insert_recursive(root, value);
    }

    fn insert_recursive(node: &mut Option<Box<Node<T>>>, value: T) {
        if let Some(n) = node {
            if value < n.value {
                Self::insert_recursive(&mut n.left, value);
            } else if value > n.value {
                Self::insert_recursive(&mut n.right, value);
            }
        } else {
            *node = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
        }
    }

    fn contains_recursive(node: &Option<Box<Node<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(n) => {
                if value < &n.value {
                    Self::contains_recursive(&n.left, value)
                } else if value > &n.value {
                    Self::contains_recursive(&n.right, value)
                } else {
                    true
                }
            }
        }
    }
    
    pub fn contains(&self, value: &T) -> bool {
        Self::contains_recursive(&self.root, value)
    }

    fn find_min_value(node: &Option<Box<Node<T>>>) -> T
    where
        T: Clone
    {
        let mut current = node.as_ref().unwrap();
        let mut min_value = current.value.clone();

        while let Some(left) = &current.left {
            min_value = left.value.clone();
            current = left;
        }

        min_value
    }

    fn find_max_value(node: &Option<Box<Node<T>>>) -> T
    where
        T: Clone
    {
        let mut current = node.as_ref().unwrap();
        let mut min_value = current.value.clone();

        while let Some(right) = &current.right {
            min_value = right.value.clone();
            current = right;
        }

        min_value
    }
}
pub mod visualizer;
pub mod display;

pub mod handlers {
    pub mod algo;
    pub mod strct;
    pub mod time_complexity;
}

pub mod theories{
    pub mod heap;
    pub mod priority_queue;
    pub mod time_complexity;
}

pub mod interactive {
    pub mod priority_queue;
}