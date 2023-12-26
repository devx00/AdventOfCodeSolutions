use regex::Regex;
use std::{
    cmp::Ordering,
    collections::{HashMap, VecDeque},
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

#[derive(Debug, Clone, Hash)]
enum PartCategory {
    X,
    M,
    A,
    S,
}

impl FromStr for PartCategory {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => PartCategory::X,
            "m" => PartCategory::M,
            "a" => PartCategory::A,
            "s" => PartCategory::S,
            _ => panic!("Invalid category!"),
        })
    }
}

#[derive(Debug, Clone, Hash)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn get(&self, category: PartCategory) -> u64 {
        match category {
            PartCategory::X => self.x,
            PartCategory::M => self.m,
            PartCategory::A => self.a,
            PartCategory::S => self.s,
        }
    }

    fn total(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl FromStr for Part {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\{x=(?<x>\d+),m=(?<m>\d+),a=(?<a>\d+),s=(?<s>\d+)\}").unwrap();
        match re.captures(s) {
            Some(captures) => Ok(Part {
                x: captures["x"].parse::<u64>().unwrap(),
                m: captures["m"].parse::<u64>().unwrap(),
                a: captures["a"].parse::<u64>().unwrap(),
                s: captures["s"].parse::<u64>().unwrap(),
            }),
            None => Err(()),
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct WorkflowOperation {
    category: PartCategory,
    order: std::cmp::Ordering,
    other_value: u64,
    success_workflow: String,
}

impl WorkflowOperation {
    fn inverted_operation(&self) -> (Ordering, u64) {
        match self.order {
            Ordering::Less => (Ordering::Greater, self.other_value.saturating_sub(1)),
            Ordering::Greater => (Ordering::Less, self.other_value + 1),
            Ordering::Equal => panic!("Cannot invert this one."),
        }
    }

    fn inverted(&self) -> Self {
        let inverted_op = self.inverted_operation();
        Self {
            category: self.category.clone(),
            order: inverted_op.0,
            other_value: inverted_op.1,
            success_workflow: "".to_string(),
        }
    }
}

impl FromStr for WorkflowOperation {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(
            r"(?<category>x|m|a|s)(?<op>=|<|>)(?<val>\d+):(?<success_workflow>[a-zAR]+)",
        )
        .unwrap();
        match re.captures(s) {
            Some(captures) => Ok(WorkflowOperation {
                category: captures["category"].parse::<PartCategory>().unwrap(),
                order: match &captures["op"] {
                    ">" => std::cmp::Ordering::Greater,
                    "<" => std::cmp::Ordering::Less,
                    "=" => std::cmp::Ordering::Equal,
                    _ => panic!("Not a valid order operation!"),
                },
                other_value: captures["val"].parse::<u64>().unwrap(),
                success_workflow: captures["success_workflow"].to_string(),
            }),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Hash)]
enum WorkflowStep {
    Operation(WorkflowOperation),
    FallbackWorkflow(String),
}

#[derive(Debug, Clone, Copy, Hash, Eq)]
struct Range {
    min: u64,
    max: u64,
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.min.cmp(&other.min))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        self.min.cmp(&other.min)
    }
}

impl Default for Range {
    fn default() -> Self {
        Self { min: 1, max: 4000 }
    }
}

impl Range {
    fn apply(&self, operation: Ordering, other_value: u64) -> Self {
        match operation {
            Ordering::Less => Self {
                min: self.min,
                max: self.max.min(other_value - 1),
            },
            Ordering::Equal => Self {
                min: other_value,
                max: other_value,
            },
            Ordering::Greater => Self {
                min: self.min.max(other_value + 1),
                max: self.max,
            },
        }
    }

    fn count(&self) -> u64 {
        (self.max - self.min) + 1
    }
}

#[derive(Debug, Clone, Copy, Hash)]
struct AcceptanceConstraints {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl AcceptanceConstraints {
    fn count(&self) -> u64 {
        self.x.count() * self.m.count() * self.a.count() * self.s.count()
    }

    fn apply(&self, operation: &WorkflowOperation) -> Self {
        match operation.category {
            PartCategory::X => Self {
                x: self.x.apply(operation.order, operation.other_value),
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            PartCategory::M => Self {
                x: self.x.clone(),
                m: self.m.apply(operation.order, operation.other_value),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            PartCategory::A => Self {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.apply(operation.order, operation.other_value),
                s: self.s.clone(),
            },
            PartCategory::S => Self {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.apply(operation.order, operation.other_value),
            },
        }
    }
}

impl Default for AcceptanceConstraints {
    fn default() -> Self {
        Self {
            x: Range::default(),
            m: Range::default(),
            a: Range::default(),
            s: Range::default(),
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct Workflow {
    name: String,
    steps: Vec<WorkflowStep>,
}

impl Workflow {
    fn check_part(&self, part: &Part) -> String {
        match self.steps.iter().find_map(|step| match step {
            WorkflowStep::Operation(op) => {
                if part.get(op.category.clone()).cmp(&op.other_value) == op.order {
                    Some(op.success_workflow.clone())
                } else {
                    None
                }
            }
            WorkflowStep::FallbackWorkflow(workflow) => Some(workflow.to_string()),
        }) {
            Some(workflow) => workflow,
            None => panic!("Couldnt find workflow"),
        }
    }

    fn acceptance_constraints(
        &self,
        input_constraints: &AcceptanceConstraints,
    ) -> Vec<(String, AcceptanceConstraints)> {
        let mut workflow_constraints: Vec<(String, AcceptanceConstraints)> = Vec::new();
        let mut next_constraints = input_constraints.clone();
        for step in &self.steps {
            match step {
                WorkflowStep::Operation(op) => {
                    let success_constraints = next_constraints.apply(&op);
                    workflow_constraints.push((op.success_workflow.clone(), success_constraints));
                    next_constraints = next_constraints.apply(&op.inverted());
                }
                WorkflowStep::FallbackWorkflow(workflow) => {
                    workflow_constraints.push((workflow.clone(), next_constraints));
                }
            }
        }
        workflow_constraints
    }
}

impl FromStr for Workflow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"(?<name>[a-z]+)\{(?<operations>[^}]+)\}").unwrap();
        match re.captures(s) {
            Some(captures) => {
                let steps = captures["operations"]
                    .split(",")
                    .map(|ops| match ops.parse::<WorkflowOperation>() {
                        Ok(op) => WorkflowStep::Operation(op),
                        _ => WorkflowStep::FallbackWorkflow(ops.to_string()),
                    })
                    .collect::<Vec<WorkflowStep>>();
                Ok(Workflow {
                    name: captures["name"].to_string(),
                    steps,
                })
            }
            _ => panic!("Failed to parse workflow!"),
        }
    }
}

#[derive(Debug, Clone)]
struct System {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl System {
    fn check_parts(&self) -> u64 {
        self.parts
            .iter()
            .filter(|p| self.check_part(p))
            .fold(0, |acc, part| acc + part.total())
    }

    fn check_part(&self, part: &Part) -> bool {
        let mut next_workflow = "in".to_string();
        loop {
            match self.run_workflow(next_workflow, &part) {
                a if a == "A".to_string() => return true,
                r if r == "R".to_string() => return false,
                other => next_workflow = other,
            }
        }
    }

    fn run_workflow(&self, workflow: String, part: &Part) -> String {
        self.workflows[&workflow].check_part(&part)
    }

    fn count_combinations(&mut self) -> u64 {
        let mut queue: VecDeque<(String, AcceptanceConstraints)> =
            vec![("in".to_string(), AcceptanceConstraints::default())].into();
        let mut count: u64 = 0;

        while let Some((workflow, acceptance_criteria)) = queue.pop_front() {
            match workflow {
                a if a == "A".to_string() => {
                    count += acceptance_criteria.count();
                }
                r if r == "R".to_string() => (),
                _ => queue
                    .extend(self.workflows[&workflow].acceptance_constraints(&acceptance_criteria)),
            }
        }

        count
    }
}

impl FromStr for System {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (workflowss, partss) = s.split_once("\n\n").expect("Expected workflows and parts");
        let workflows = workflowss
            .lines()
            .map(|l| l.parse::<Workflow>().expect("Expected a workflow!"))
            .fold(HashMap::new(), |mut acc, w| {
                acc.insert(w.name.clone(), w);

                acc
            });
        let parts = partss
            .lines()
            .map(|l| l.parse::<Part>().expect("Expected a part"))
            .collect::<Vec<Part>>();
        Ok(System { workflows, parts })
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let system = input.parse::<System>().expect("Expected to have a system.");
    let result = system.check_parts();
    println!("Part 1 Result: {}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("part1.txt");
    let mut system = input.parse::<System>().expect("Expected to have a system.");
    let result = system.count_combinations();
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    part2();
}
