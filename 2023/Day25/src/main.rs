use eframe::{run_native, App, CreationContext};
use egui::Context;
use egui_graphs::{DefaultEdgeShape, DefaultNodeShape, GraphView};
use egui_graphs::{Graph as EGraph, SettingsInteraction};
use petgraph::adj::NodeIndex;
use petgraph::{
    stable_graph::{DefaultIx, StableGraph, StableUnGraph},
    Undirected,
};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}
type NodeId = String;

#[derive(Debug)]
struct Graph {
    nodes: HashMap<NodeId, HashSet<NodeId>>,
}

impl FromStr for Graph {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes: HashMap<NodeId, HashSet<NodeId>> = HashMap::new();
        s.lines().for_each(|l| match l.split_once(": ") {
            Some((node, connected)) => {
                let node_entry = nodes.entry(node.to_string()).or_insert(HashSet::new());
                let to_nodes = connected.trim().split_whitespace().map(|v| v.to_string());
                node_entry.extend(to_nodes.clone());
                to_nodes.for_each(|tn| {
                    nodes
                        .entry(tn)
                        .or_insert(HashSet::new())
                        .insert(node.to_string());
                });
            }
            _ => panic!("Should have been able to parse input line."),
        });
        Ok(Graph { nodes })
    }
}

impl Graph {
    fn partition(&self, root: String, k: usize) -> Option<(usize, usize)> {
        let mut seen: HashSet<NodeId> = HashSet::new();
        let mut next_nodes: Vec<NodeId> = vec![root];
        while next_nodes.len() > 0 {
            seen.extend(next_nodes.clone());
            let tmp_nodes = next_nodes.iter().map(|n| {
                self.nodes[n]
                    .iter()
                    .map(|n| n.clone())
                    .filter(|n| !&seen.contains(n))
                    .collect::<Vec<String>>()
            });
            let leading_edges = tmp_nodes.clone().map(|n| n.len()).sum::<usize>();
            println!("Num Edges: {}", leading_edges);
            if leading_edges == k {
                return Some((seen.len(), self.nodes.len() - seen.len()));
            }

            next_nodes = tmp_nodes.flatten().collect::<Vec<_>>()
        }

        None
    }
    fn find_partition(&self, k: usize) -> Result<usize, String> {
        for node in self.nodes.keys() {
            println!("Checking node: {}", node);
            match self.partition(node.to_string(), k) {
                Some((size1, size2)) => return Ok(size1 * size2),
                None => (),
            }
        }
        Err("Failed to partition".to_string())
    }

    fn print(&self) {
        for (node, neighbors) in &self.nodes {
            println!("{} => {:?}", node, neighbors);
        }
    }

    fn generate_stable_graph(&self) -> StableGraph<String, (), Undirected> {
        let mut g: StableGraph<String, (), Undirected> = StableGraph::default();
        let mut node_map: HashMap<String, NodeIndex<_>> = HashMap::new();

        for node in self.nodes.keys() {
            let graph_node = g.add_node(node.clone());
            node_map.insert(node.to_string(), graph_node);
        }

        for (node, neighbors) in &self.nodes {
            let a = node_map[node];
            for neighbor in neighbors {
                let b = node_map[neighbor];

                if !g.contains_edge(a, b) {
                    g.add_edge(a, b, ());
                }
            }
        }

        g
    }
}

struct GraphApp {
    g: EGraph<String, (), Undirected>,
}

impl App for GraphApp {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let interaction_settings = &SettingsInteraction::new()
                .with_dragging_enabled(true)
                .with_node_clicking_enabled(true)
                .with_node_selection_enabled(true)
                .with_node_selection_multi_enabled(true)
                .with_edge_clicking_enabled(true)
                .with_edge_selection_enabled(true)
                .with_edge_selection_multi_enabled(true);
            ui.add(
                &mut GraphView::<_, _, _, _, DefaultNodeShape, DefaultEdgeShape>::new(&mut self.g)
                    .with_interactions(interaction_settings),
            );
        });
    }
}

impl GraphApp {
    fn new(_: &CreationContext<'_>, gr: StableGraph<String, (), Undirected>) -> Self {
        Self {
            g: EGraph::from(&gr),
        }
    }
}
fn show_graph(graph: StableGraph<String, (), Undirected>) {
    let native_options = eframe::NativeOptions::default();
    run_native(
        "egui_graphs_undirected_demo",
        native_options,
        Box::new(|cc| Box::new(GraphApp::new(cc, graph))),
    )
    .unwrap();
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let graph = input.parse::<Graph>().expect("Graph");
    show_graph(graph.generate_stable_graph());
    // let result = graph.find_partition(3).expect("Couldn't find partition..");

    // println!("Part 1 Result: {}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("example1.txt");
    let result = input;
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    // part2();
}
