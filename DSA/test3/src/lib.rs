pub mod hash_tables {
    use std::collections::HashMap;

    pub fn main_hash(){
        let keys = vec![4322, 1334, 1471, 9679, 1989, 6171, 6173, 4199];
        let mut hash_table:HashMap<i32, Vec<i32>> = (0..10)
            .map(|i| (i, Vec::new()))
            .collect();

        let mut hash_table_copy = hash_table.clone();

        println!{"Hashing without Linear Probing"}
        compute_hash(&keys, &mut hash_table);

        println!{"Hashing with Linear Probing"}
        double_hashing(&keys, &mut hash_table_copy, 10);
    }
    pub fn compute_hash(hash_values: &[i32], hash_t: &mut HashMap<i32, Vec<i32>>) {

        for &value in hash_values {
            let hash = value % 10;
            insert_into_hash_table(hash_t, hash, value);
        }

        for (key, values) in hash_t.iter() {
            println!("Key: {}, Values: {:?}", key, values);
        }
    }

    pub fn double_hashing(hash_values: &[i32], hash_t: &mut HashMap<i32, Vec<i32>>, m: i32) {
        for &value in hash_values {
            let h1 = value % m;
            let h2 = 1 + (value % (m - 1));
            let mut i = 0;
            loop {
                let hash = (h1 + i * h2) % m;
                if hash_t.get(&hash).map_or(true, |v| v.is_empty()) {
                    insert_into_hash_table(hash_t, hash, value);
                    break;
                }
                i += 1;
            }
        }

        for (key, values) in hash_t.iter() {
            println!("Key: {}, Values: {:?}", key, values);
        }
    }

    pub fn insert_into_hash_table(hash_t: &mut HashMap<i32, Vec<i32>>, key: i32, value: i32) {
        hash_t.entry(key).or_insert(Vec::new()).push(value);
    }

}

pub mod graphs {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::Write;
    use std::process::Command;

    pub struct Node<T> {
        pub value: T,
    }

    pub struct Graph<T> {
        pub nodes: Vec<Node<T>>,
        pub adj: Vec<Vec<usize>>,
        pub edges: Vec<Edge>,
    }
    #[derive(Clone)]
    pub struct Edge {
        pub(crate) src: usize,
        pub(crate) dest: usize,
        pub(crate) weight: i32,
    }
    impl<T: std::fmt::Display> Graph<T>{
        pub fn new() -> Self {
            Graph { nodes: Vec::new(), adj: Vec::new(), edges: vec![] }
        }

        pub fn add_node(&mut self, value: T) {
            self.nodes.push(Node { value });
            self.adj.push(Vec::new());
        }

        pub fn add_edge(&mut self, u: usize, v: usize, weight: i32) {
            self.adj[u].push(v);
            self.adj[v].push(u);
            self.edges.push(Edge { src: u, dest: v, weight });
        }
        
        pub fn add_edge_simple(&mut self, u: usize, v: usize) {
            self.add_edge(u, v, 0);
        }

        pub fn display(&self) {
            for node in &self.nodes {
                println!("{}", node.value);
            }
        }

        pub fn dfs(&self, start: usize) {
            let mut visited = vec![false; self.nodes.len()];
            self.dfs_helper(start, &mut visited);
        }

        fn dfs_helper(&self, node: usize, visited: &mut Vec<bool>) {
            visited[node] = true;
            println!("{}", self.nodes[node].value);
            for &neighbor in &self.adj[node] {
                if !visited[neighbor] {
                    self.dfs_helper(neighbor, visited);
                }
            }
        }

        pub fn bfs(&self, start: usize) {
            let mut visited = vec![false; self.nodes.len()];
            let mut queue = std::collections::VecDeque::new();
            visited[start] = true;
            queue.push_back(start);

            while let Some(node) = queue.pop_front() {
                println!("{}", self.nodes[node].value);
                for &neighbor in &self.adj[node] {
                    if !visited[neighbor] {
                        visited[neighbor] = true;
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    pub fn adjacency_list_main(){
        let mut adjacency_list: HashMap<i32, Vec<i32>> = HashMap::new();

        // Adding edges to the adjacency list
        add_edge(&mut adjacency_list, 1, 2);
        add_edge(&mut adjacency_list, 1, 3);
        add_edge(&mut adjacency_list, 2, 4);
        add_edge(&mut adjacency_list, 3, 5);

        // Displaying the adjacency list
        display_adjacency_list(&adjacency_list);
    }
    pub fn add_edge(adjacency_list: &mut HashMap<i32, Vec<i32>>, u: i32, v: i32) {
        adjacency_list.entry(u).or_insert(Vec::new()).push(v);
        adjacency_list.entry(v).or_insert(Vec::new()).push(u);
    }

    pub fn display_adjacency_list(adjacency_list: &HashMap<i32, Vec<i32>>) {
        for (key, values) in adjacency_list.iter() {
            println!("{}: {:?}", key, values);
        }
    }

    pub fn adjaceny_matrix_main(){
        let mut adjacency_matrix: Vec<Vec<i32>> = vec![vec![0; 5]; 5];

        // Adding edges to the adjacency matrix
        add_edge_matrix(&mut adjacency_matrix, 0, 1);
        add_edge_matrix(&mut adjacency_matrix, 0, 2);
        add_edge_matrix(&mut adjacency_matrix, 1, 3);
        add_edge_matrix(&mut adjacency_matrix, 2, 4);

        // Displaying the adjacency matrix
        display_adjacency_matrix(&adjacency_matrix);
    }

    pub fn add_edge_matrix(adjacency_matrix: &mut Vec<Vec<i32>>, u: i32, v: i32) {
        if u < adjacency_matrix.len() as i32 && v < adjacency_matrix.len() as i32 {
            adjacency_matrix[u as usize][v as usize] = 1;
            adjacency_matrix[v as usize][u as usize] = 1; // For undirected graph
        } else {
            println!("Error: Vertex out of bounds");
        }
    }

    pub fn display_adjacency_matrix(adjacency_matrix: &Vec<Vec<i32>>) {
        for row in adjacency_matrix {
            for value in row {
                print!("{} ", value);
            }
            println!();
        }
    }
    impl<T: std::fmt::Display + Clone> Graph<T> {
        pub fn to_dot(&self) -> String {
            let mut dot = String::from("graph G {\n");

            for (i, node) in self.nodes.iter().enumerate() {
                dot.push_str(&format!("  {} [label=\"{}\"];\n", i, node.value));
            }

            let mut added_edges = std::collections::HashSet::new();
            for (u, neighbors) in self.adj.iter().enumerate() {
                for &v in neighbors {
                    let edge = if u < v { (u, v) } else { (v, u) };
                    if !added_edges.contains(&edge) {
                        dot.push_str(&format!("  {} -- {};\n", u, v));
                        added_edges.insert(edge);
                    }
                }
            }

            dot.push_str("}\n");
            dot
        }

        pub fn visualize(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
            std::fs::create_dir_all("graphs")?;

            let dot_content = self.to_dot();
            let dot_file = format!("graphs/{}.dot", filename);
            let png_file = format!("graphs/{}.png", filename);

            let mut file = File::create(&dot_file)?;
            file.write_all(dot_content.as_bytes())?;

            Command::new("dot")
                .args(&["-Tpng", &dot_file, "-o", &png_file])
                .output()?;

            println!("Graph saved as {}", png_file);
            Ok(())
        }

        pub fn sort_adj(&mut self){
            for neighbors in &mut self.adj {
                neighbors.sort();
            }
        }

        pub fn kruskal(&mut self) -> Self {
            let mut mst = Graph::new();

            for node in &self.nodes {
                mst.add_node(node.value.clone());
            }

            let mut sorted_edges = self.edges.clone();
            sorted_edges.sort_by_key(|e| e.weight);

            let mut parent: Vec<usize> = (0..self.nodes.len()).collect();

            fn find(parent: &mut Vec<usize>, u: usize) -> usize {
                if parent[u] != u {
                    parent[u] = find(parent, parent[u]);
                }
                parent[u]
            }

            fn union(parent: &mut Vec<usize>, u: usize, v: usize) {
                let root_u = find(parent, u);
                let root_v = find(parent, v);
                parent[root_u] = root_v;
            }

            for edge in sorted_edges {
                let u = find(&mut parent, edge.src);
                let v = find(&mut parent, edge.dest);

                if u != v {
                    mst.add_edge(edge.src, edge.dest, edge.weight);
                    union(&mut parent, u, v);
                }
            }
            println!("Minimum Spanning Tree:");
            for edge in &mst.edges {
                println!("{} -- {} (weight: {})", mst.nodes[edge.src].value, mst.nodes[edge.dest].value, edge.weight);
            }

        mst
        }

        pub fn prims(&mut self) -> Self{
            let mut mst = Graph::new();
            for node in &self.nodes {
                mst.add_node(node.value.clone());
            }

            let mut visited = vec![false; self.nodes.len()];
            let mut min_heap = std::collections::BinaryHeap::new();

            visited[0] = true;
            for &neighbor in &self.adj[0] {
                min_heap.push((0, 0, neighbor));
            }

            while let Some((weight, src, dest)) = min_heap.pop() {
                if visited[dest] {
                    continue;
                }
                visited[dest] = true;
                mst.add_edge(src, dest, weight);

                for &next_neighbor in &self.adj[dest] {
                    if !visited[next_neighbor] {
                        min_heap.push((weight + 1, dest, next_neighbor)); // Assuming weight is 1 for simplicity
                    }
                }
            }

            println!("Minimum Spanning Tree using Prim's Algorithm:");
            for edge in &mst.edges {
                println!("{} -- {} (weight: {})", mst.nodes[edge.src].value, mst.nodes[edge.dest].value, edge.weight);
            }

            mst
        }

        pub fn dijkstra(&self, start: usize) -> Vec<i32> {
            use std::collections::BinaryHeap;
            use std::cmp::Reverse;

            let mut distances = vec![i32::MAX; self.nodes.len()];
            distances[start] = 0;

            let mut priority_queue = BinaryHeap::new();
            priority_queue.push(Reverse((0, start)));

            while let Some(Reverse((current_dist, current_vertex))) = priority_queue.pop() {
                if current_dist > distances[current_vertex] {
                    continue;
                }

                for &neighbor in &self.adj[current_vertex] {
                    let edge = self.edges.iter()
                        .find(|e| (e.src == current_vertex && e.dest == neighbor) ||
                            (e.src == neighbor && e.dest == current_vertex))
                        .unwrap();

                    let weight = edge.weight;
                    let distance = current_dist + weight;

                    if distance < distances[neighbor] {
                        distances[neighbor] = distance;
                        priority_queue.push(Reverse((distance, neighbor)));
                    }
                }
            }

            distances
        }

        pub fn bellman_ford(&self, source: usize) -> Result<Vec<i32>, &'static str> {
            let vertices = self.nodes.len();

            let mut distances = vec![i32::MAX; vertices];
            distances[source] = 0;

            for _ in 0..vertices - 1 {
                for edge in &self.edges {
                    if distances[edge.src] == i32::MAX {
                        continue;
                    }

                    let potential_distance = match distances[edge.src].checked_add(edge.weight) {
                        Some(sum) => sum,
                        None => continue,
                    };

                    if potential_distance < distances[edge.dest] {
                        distances[edge.dest] = potential_distance;
                    }
                }
            }

            for edge in &self.edges {
                if distances[edge.src] != i32::MAX
                    && distances[edge.src] + edge.weight < distances[edge.dest] {
                    return Err("Graph contains a negative weight cycle");
                }
            }

            Ok(distances)
        }
    }

    pub fn graph_main(){
        let mut graph = Graph { nodes: Vec::new(), adj: vec![], edges: vec![] };
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);

        graph.add_edge_simple(1, 2);
        graph.add_edge_simple(0, 1);
        graph.add_edge_simple(2, 3);
        graph.add_edge_simple(3, 0);
        graph.add_edge_simple(0, 2);

        println!("DFS Traversal:");
        graph.dfs(0);
        println!("BFS Traversal:");
        graph.bfs(0);

        println!("Displaying Graph Nodes:");
        graph.visualize("graph.dot").unwrap();
    }

}

pub mod mst{
    use crate::graphs::{Graph, Node};
    pub fn mst_main(){
        let mut mstGraph = Graph::new();
        mstGraph.add_node(1);
        mstGraph.add_node(2);
        mstGraph.add_node(3);
        mstGraph.add_node(4);

        mstGraph.add_edge(0, 1, 10);
        mstGraph.add_edge(0, 2, 6);
        mstGraph.add_edge(0, 3, 5);
        mstGraph.add_edge(1, 3, 15);
        mstGraph.add_edge(2, 3, 4);

        let minimum_spanning_tree = mstGraph.kruskal();
        mstGraph.visualize("mst_before.dot").unwrap();
        minimum_spanning_tree.visualize("mst_graph.dot").unwrap();

        let minmum_spanning_tree_prims = mstGraph.prims();
        minmum_spanning_tree_prims.visualize("mst_prims.dot").unwrap();
    }
}

pub mod dijkstra {
    use crate::graphs::{Graph, Node};

    pub fn dijkstra_main() {
        let mut graph = Graph::new();
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(4);

        graph.add_edge(0, 1, 10);
        graph.add_edge(0, 2, 6);
        graph.add_edge(0, 3, 5);
        graph.add_edge(1, 3, 15);
        graph.add_edge(2, 3, 4);

        let distances = graph.dijkstra(0);
        println!("Dijkstra's Algorithm Shortest Distances from Node 0:");
        for (i, distance) in distances.iter().enumerate() {
            println!("Distance from Node 0 to Node {}: {}", i, distance);
        }
    }
}

pub mod bellman_ford {
    use crate::graphs::{Graph, Node};

    pub fn bellman_ford_main() {
        let mut graph = Graph::new();

        for i in 0..6 {
            graph.add_node(i);
        }

        graph.add_edge(0, 1, 5);
        graph.add_edge(0, 2, 7);
        graph.add_edge(1, 2, 3);
        graph.add_edge(1, 3, 4);
        graph.add_edge(1, 4, 6);
        graph.add_edge(3, 4, -1);
        graph.add_edge(3, 5, 2);
        graph.add_edge(4, 5, -3);

        match graph.bellman_ford(0){
            Ok(distances) => {
                println!("Bellman-Ford Algorithm Shortest Distances from Node 0:");
                for (i, distance) in distances.iter().enumerate() {
                    println!("Distance from Node 0 to Node {}: {}", i, distance);
                }
                let mut shortest_paths_graph = Graph::new();
                for i in 0..6 {
                    shortest_paths_graph.add_node(i);
                }
                for edge in &graph.edges {
                    if distances[edge.src] != i32::MAX &&
                        distances[edge.dest] != i32::MAX &&
                        distances[edge.dest] == distances[edge.src] + edge.weight {
                        shortest_paths_graph.add_edge(edge.src, edge.dest, edge.weight);
                    }
                }
                shortest_paths_graph.visualize("bellman_ford_shortest_paths.dot").unwrap();

            },
            Err(err) => println!("{}", err),
        }

        graph.visualize("bellman_ford_graph.dot").unwrap();
    }
}
