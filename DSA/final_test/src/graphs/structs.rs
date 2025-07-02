pub struct Node {
    pub value: i32,
    pub neighbors: Vec<(usize, i32)>,
}

pub struct Graph {
    pub nodes: Vec<Node>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Node { value, neighbors: Vec::new() }
    }
}

impl Graph {
    pub fn new() -> Self {
        Graph { nodes: Vec::new() }
    }

    pub fn add_node(&mut self, value: i32) -> usize {
        let index = self.nodes.len();
        self.nodes.push(Node::new(value));
        index
    }

    pub fn add_edge(&mut self, from: usize, to: usize, weight: i32) {
        if from < self.nodes.len() && to < self.nodes.len() {
            self.nodes[from].neighbors.push((to, weight));
        }
    }

    pub fn dfs(&self, start: usize, visited: &mut Vec<usize>) {
        visited.push(start);

        for &(neighbor, _) in &self.nodes[start].neighbors {
            if !visited.contains(&neighbor) {
                self.dfs(neighbor, visited);
            }
        }
    }

    pub fn bfs(&self, start: usize) -> Vec<usize> {
        let mut visited = Vec::new();
        let mut queue = std::collections::VecDeque::new();

        visited.push(start);
        queue.push_back(start);

        while let Some(current) = queue.pop_front() {
            for &(neighbor, _) in &self.nodes[current].neighbors {
                if !visited.contains(&neighbor) {
                    visited.push(neighbor);
                    queue.push_back(neighbor);
                }
            }
        }

        visited
    }

    pub fn dijkstra(&self, start: usize) -> Vec<usize, i32> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let mut distances: Vec<Option<32>> = vec![None; self.nodes.len()];
        let mut previous: Vec<Option<usize>> = vec![None; self.nodes.len()];

        distances[start] = Some(0);

        let mut priority_queue = BinaryHeap::new();
        priority_queue.push(Reverse((0, start)));

        while let Some(Reverse((distance, current_node))) = priority_queue.pop() {
            if let Some(dist) = distances[current_node]{
                if current_distance > dist{
                    continue;
                }
            }

            for &(neighbor, weight) in &self.nodes[current_node].neighbors {
                let new_distance = current_distance + weight;

                if distances[neighbor].is_none() || new_distance < distances[neighbor].unwrap() {
                    distances[neighbor] = Some(new_distance);
                    previous[neighbor] = Some(current_node);
                    priority_queue.push(Reverse((new_distance, neighbor)));
                }
            }

            previous.iter().zip(distance.iter())
                .map(|(&prev, &dist) |{
                if let Some(distance) = dist{
                    Some((prev.unwrap_or(start), distance))
                }
                else{
                    None
                }
            })
                .collect()
        }
    }

}