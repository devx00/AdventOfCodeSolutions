use eframe::{run_native, App, CreationContext};
use egui::Context;
use egui_graphs::{DefaultEdgeShape, DefaultNodeShape, GraphView};
use egui_graphs::{Graph as EGraph, SettingsInteraction};
use petgraph::adj::NodeIndex;
use petgraph::algo::tarjan_scc;
use petgraph::{
    stable_graph::{DefaultIx, StableGraph, StableUnGraph},
    Undirected,
};
use std::cell::RefCell;
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

#[derive(Debug)]
enum GraphPartitionGroup {
    GroupA,
    GroupB,
}
struct GraphPartition {
    group_a: HashSet<NodeId>,
    group_b: HashSet<NodeId>,
    graph: RefCell<Graph>,
    num_crossreferences: usize,
}

impl GraphPartition {
    fn new(graph: Graph) -> Self {
        Self {
            group_a: graph
                .nodes
                .keys()
                .into_iter()
                .map(|k| k.clone())
                .collect::<HashSet<NodeId>>(),
            group_b: HashSet::new(),
            graph: RefCell::new(graph),
            num_crossreferences: 0,
        }
    }
    fn whereis(&self, node: &NodeId) -> GraphPartitionGroup {
        if self.group_b.contains(node) {
            return GraphPartitionGroup::GroupA;
        }

        if self.group_a.contains(node) {
            return GraphPartitionGroup::GroupB;
        }

        panic!("Node doesnt exist in either group!");
    }

    fn move_node(
        &mut self,
        node: NodeId,
        cross_inc: usize,
        cross_dec: usize,
        to_group: GraphPartitionGroup,
    ) {
        let (target_group, other_group) = match to_group {
            GraphPartitionGroup::GroupA => (&mut self.group_a, &mut self.group_b),
            GraphPartitionGroup::GroupB => (&mut self.group_b, &mut self.group_a),
        };

        println!(
            "Moving Node {} to {:?} (inc {}, dec {}) {}",
            node, to_group, cross_inc, cross_dec, self.num_crossreferences
        );
        other_group.remove(&node);
        target_group.insert(node);

        self.num_crossreferences += cross_inc;
        self.num_crossreferences -= cross_dec;
    }

    fn cross_references(&self) -> HashMap<NodeId, (isize, isize)> {
        let mut cross_map: HashMap<NodeId, (isize, isize)> = HashMap::new();
        self.group_a
            .iter()
            .map(|n| {
                self.graph.borrow().edges(n).into_iter().fold(
                    (n.to_string(), 0isize, 0isize),
                    |(cn, inc, dec), ne| match self.group_b.contains(&ne) {
                        true => (cn, inc, dec + 1),
                        false => (cn, inc + 1, dec),
                    },
                )
            })
            .for_each(|(n, inc, dec)| {
                cross_map
                    .entry(n)
                    .and_modify(|(e_inc, e_dec)| {
                        *e_inc += inc;
                        *e_dec += dec;
                    })
                    .or_insert((inc, dec));
            });

        println!("Cross Map: {:?}", cross_map);
        cross_map
    }

    fn next_node_to_move(&self) -> Option<(NodeId, (isize, isize))> {
        self.cross_references().into_iter().fold(
            None as Option<(NodeId, (isize, isize))>,
            |acc, (next_node, (next_inc, next_dec))| match acc {
                Some((acc_node, (acc_inc, acc_dec))) => {
                    let net_acc = acc_inc - acc_dec;
                    let net_next = next_inc - next_dec;
                    if net_next < net_acc {
                        Some((next_node, (next_inc, next_dec)))
                    } else {
                        Some((acc_node, (acc_inc, acc_dec)))
                    }
                }
                _ => Some((next_node, (next_inc, next_dec))),
            },
        )
    }

    fn find_partitions(&mut self) -> Result<usize, String> {
        let first = self
            .graph
            .borrow()
            .nodes
            .iter()
            .map(|(n, e)| (n.to_string(), e.len()))
            .reduce(
                |(curr_n, curr_e), (n, e)| {
                    if e > curr_e {
                        (n, e)
                    } else {
                        (curr_n, curr_e)
                    }
                },
            )
            .expect("Node to exist");

        self.move_node(first.0.to_string(), first.1, 0, GraphPartitionGroup::GroupB);
        let init_edges = self.graph.borrow().edges(&first.0);
        for node in init_edges.iter() {
            self.move_node(node.to_string(), GraphPartitionGroup::GroupB);
        }
        println!("{:?}\n{:?}", self.group_a, self.group_b);

        while let Some((next_node, (next_inc, next_dec))) = self.next_node_to_move() {
            self.move_node(
                next_node,
                next_inc as usize,
                next_dec as usize,
                GraphPartitionGroup::GroupB,
            );

            if self.num_crossreferences == 3 {
                break;
            }
        }

        match self.num_crossreferences {
            3 => Ok(self.group_a.len() * self.group_b.len()),
            _ => Err("Couldnt find valid partition...".to_string()),
        }
    }
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
    fn edges(&self, node: &NodeId) -> Vec<NodeId> {
        self.nodes[node].clone().into_iter().collect()
    }

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
    let input = load_input("example1.txt");
    let graph = input.parse::<Graph>().expect("Graph");
    let mut partition = GraphPartition::new(graph);
    match partition.find_partitions() {
        Ok(result) => println!("Part 1 result: {}", result),
        Err(e) => println!("Part 1 failed: {}", e),
    }
    // show_graph(graph.generate_stable_graph());
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
