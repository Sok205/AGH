fn main() {
    println!("HashMaps");
    test3::hash_tables::main_hash();

    println!("HashMap theory: Insertion time is O(1) on average, but can degrade to O(n) in the worst case if many elements hash to the same bucket. This is due to collisions, which can lead to longer search times as elements are stored in a linked list or similar structure within the bucket.");

    println!("Adjacency List");
    test3::graphs::adjacency_list_main();
    println!("Adjacency List theory: An adjacency list is a collection of lists or arrays that represent a graph. Each list corresponds to a vertex and contains the vertices that are adjacent to it. This representation is efficient for sparse graphs, where the number of edges is much less than the square of the number of vertices, as it uses less space compared to an adjacency matrix.");

    println!("Adjacency Matrix");
    test3::graphs::adjaceny_matrix_main();
    println!("Adjacency Matrix theory: An adjacency matrix is a 2D array where the element at row i and column j indicates whether there is an edge between vertex i and vertex j. This representation is efficient for dense graphs, where the number of edges is close to the square of the number of vertices, as it allows for quick edge existence checks but uses more space compared to an adjacency list.");

    println!("Graph");
    test3::graphs::graph_main();
}

