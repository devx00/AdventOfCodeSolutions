use std::{
    borrow::BorrowMut,
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufReader, Read},
    ops::{Deref, DerefMut},
    str::FromStr,
};

fn load_input(input_name: &str) -> String {
    let input_file = File::open(format!("inputs/{}", input_name)).unwrap();

    let mut data: String = String::new();
    let _ = BufReader::new(input_file).read_to_string(&mut data);

    data
}

trait CommunicationModule {
    fn name(&self) -> String;
    fn as_emitter(&mut self) -> Option<&mut dyn Emitter> {
        None
    }

    fn as_receiver(&mut self) -> Option<&mut dyn Receiver> {
        None
    }

    fn as_broadcaster(&mut self) -> Option<&mut Broadcaster> {
        None
    }

    fn as_sink(&mut self) -> Option<&mut OutputSink> {
        None
    }

    fn still_processing(&mut self) -> bool {
        let emitter_processing = match self.as_emitter() {
            Some(emitter) => emitter.output_core().output_queue.len() > 0,
            _ => false,
        };
        let receiver_processing = match self.as_receiver() {
            Some(receiver) => receiver.receiver_processing(),
            _ => false,
        };

        emitter_processing || receiver_processing
    }
}

trait Receiver {
    fn input_core(&mut self) -> &mut InputCore;
    fn add_input(&mut self, input_module: String) {
        self.input_core().inputs.push(input_module);
    }
    fn receive_pulse(&mut self, pulse: DirectedPulse) {
        self.input_core().input_queue.push_back(pulse);
    }

    fn receiver_processing(&mut self) -> bool {
        // println!("Receiver Processing: {:?}", self.input_core().input_queue);
        self.input_core().input_queue.len() > 0
    }

    fn process_inputs(&mut self);
}

trait Emitter {
    fn output_core(&mut self) -> &mut OutputCore;
    fn send_pulse_to_all(&mut self, src: String, pulse: Pulse) {
        let pulses = self
            .output_core()
            .outputs
            .iter()
            .map(|out| DirectedPulse::new(src.clone(), out.to_string(), pulse.clone()))
            .collect::<Vec<DirectedPulse>>();
        self.output_core().output_queue.extend(pulses);
    }
    fn add_output(&mut self, output_module: String) {
        self.output_core().outputs.push(output_module);
    }

    fn add_outputs(&mut self, outputs: Vec<String>) {
        for output in outputs {
            self.add_output(output);
        }
    }
    fn emitter_processing(&mut self) -> bool {
        println!("Emitter Processing: {:?}", self.output_core().output_queue);
        self.output_core().output_queue.len() > 0
    }

    fn output_pulses(&mut self) -> Vec<DirectedPulse> {
        self.output_core()
            .output_queue
            .drain(..)
            .collect::<Vec<DirectedPulse>>()
    }
}

#[derive(Debug, Clone, Hash)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, Hash)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug)]
struct InputCore {
    input_queue: VecDeque<DirectedPulse>,
    inputs: Vec<String>,
}

impl InputCore {
    fn new() -> InputCore {
        Self {
            input_queue: VecDeque::new(),
            inputs: vec![],
        }
    }
}

#[derive(Debug)]
struct OutputCore {
    output_queue: VecDeque<DirectedPulse>,
    outputs: Vec<String>,
}
impl OutputCore {
    fn new() -> OutputCore {
        Self {
            output_queue: VecDeque::new(),
            outputs: vec![],
        }
    }
}

#[derive(Debug)]
struct FlipFlop {
    name: String,
    state: FlipFlopState,
    input: InputCore,
    output: OutputCore,
}

impl FlipFlop {
    fn new(name: String) -> FlipFlop {
        Self {
            name,
            state: FlipFlopState::default(),
            input: InputCore::new(),
            output: OutputCore::new(),
        }
    }
}

impl FlipFlopState {
    fn flip(&self) -> FlipFlopState {
        match self {
            Self::Off => Self::On,
            Self::On => Self::Off,
        }
    }
}

impl Default for FlipFlopState {
    fn default() -> Self {
        FlipFlopState::Off
    }
}

impl CommunicationModule for FlipFlop {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn as_emitter(&mut self) -> Option<&mut dyn Emitter> {
        Some(self)
    }

    fn as_receiver(&mut self) -> Option<&mut dyn Receiver> {
        Some(self)
    }
}

impl Receiver for FlipFlop {
    fn input_core(&mut self) -> &mut InputCore {
        &mut self.input
    }

    fn process_inputs(&mut self) {
        while let Some(directed_pulse) = self.input.input_queue.pop_front() {
            match directed_pulse.pulse {
                Pulse::High => (),
                Pulse::Low => {
                    self.state = self.state.flip();
                    let pulse = match self.state {
                        FlipFlopState::On => Pulse::High,
                        FlipFlopState::Off => Pulse::Low,
                    };

                    self.send_pulse_to_all(self.name.clone(), pulse);
                }
            }
        }
    }
}

impl Emitter for FlipFlop {
    fn output_core(&mut self) -> &mut OutputCore {
        &mut self.output
    }
}

#[derive(Debug)]
struct Conjunction {
    name: String,
    state: HashMap<String, PulseState>,
    input: InputCore,
    output: OutputCore,
}

impl Conjunction {
    fn new(name: String) -> Self {
        Self {
            name,
            state: HashMap::new(),
            input: InputCore::new(),
            output: OutputCore::new(),
        }
    }
    fn check_and_emit(&mut self) {
        // println!("Conjunction Inputs: {:?}", self.input.inputs);
        let pulse = match self.state.values().all(|v| v.is_high()) {
            true => Pulse::Low,
            _ => Pulse::High,
        };

        self.send_pulse_to_all(self.name.clone(), pulse);
    }
}

impl Receiver for Conjunction {
    fn add_input(&mut self, input_module: String) {
        // println!("Adding Input {} To {}", input_module, self.name);
        self.input.inputs.push(input_module.clone());
        self.state
            .insert(input_module.clone(), PulseState::new(input_module));
    }
    fn input_core(&mut self) -> &mut InputCore {
        &mut self.input
    }

    fn process_inputs(&mut self) {
        while let Some(dir_pulse) = self.input.input_queue.pop_front() {
            let source = dir_pulse.source.clone();
            let pulse_state = self
                .state
                .entry(source.clone())
                .or_insert(PulseState::new(source.clone()));
            pulse_state.state = dir_pulse.pulse;
            self.check_and_emit();
        }
    }
}

impl Emitter for Conjunction {
    fn output_core(&mut self) -> &mut OutputCore {
        &mut self.output
    }
}

impl CommunicationModule for Conjunction {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn as_emitter(&mut self) -> Option<&mut dyn Emitter> {
        Some(self)
    }

    fn as_receiver(&mut self) -> Option<&mut dyn Receiver> {
        Some(self)
    }
}

impl Pulse {
    fn flip(&self) -> Pulse {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High,
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct PulseState {
    source: String,
    state: Pulse,
}

impl PulseState {
    fn new(module: String) -> Self {
        Self {
            source: module,
            state: Pulse::Low,
        }
    }

    fn is_high(&self) -> bool {
        match self.state {
            Pulse::Low => false,
            Pulse::High => true,
        }
    }
}

#[derive(Debug, Clone, Hash)]
struct DirectedPulse {
    source: String,
    destination: String,
    pulse: Pulse,
}

impl DirectedPulse {
    fn new(source: String, destination: String, pulse: Pulse) -> Self {
        Self {
            source,
            destination,
            pulse,
        }
    }
}

#[derive(Debug)]
struct Broadcaster {
    presses_left: u32,
    output: OutputCore,
    ready: bool,
    num_presses: u32,
}

impl Broadcaster {
    fn new() -> Self {
        Self {
            presses_left: 0,
            output: OutputCore::new(),
            ready: true,
            num_presses: 0,
        }
    }

    fn new_with_presses(presses: u32) -> Self {
        Self {
            presses_left: presses,
            output: OutputCore::new(),
            ready: true,
            num_presses: 0,
        }
    }

    fn press_button(&mut self, times: u32) {
        self.presses_left += times;
    }

    fn set_ready(&mut self) {
        self.ready = true;
    }
}

impl Emitter for Broadcaster {
    fn output_core(&mut self) -> &mut OutputCore {
        &mut self.output
    }

    fn output_pulses(&mut self) -> Vec<DirectedPulse> {
        if self.presses_left > 0 && self.ready {
            self.ready = false;
            self.presses_left -= 1;
            self.num_presses += 1;
            self.output
                .outputs
                .iter()
                .map(|modname| DirectedPulse::new(self.name().clone(), modname.clone(), Pulse::Low))
                .collect::<Vec<DirectedPulse>>()
        } else {
            vec![]
        }
    }
}

impl CommunicationModule for Broadcaster {
    fn name(&self) -> String {
        "Broadcaster".to_string()
    }

    fn as_broadcaster(&mut self) -> Option<&mut Broadcaster> {
        Some(self)
    }

    fn as_emitter(&mut self) -> Option<&mut dyn Emitter> {
        Some(self)
    }

    fn still_processing(&mut self) -> bool {
        self.presses_left > 0 || self.output_core().output_queue.len() > 0
    }
}

#[derive(Debug)]
struct OutputSink {
    input: InputCore,
    name: Option<String>,
    seen_low: bool,
}
impl OutputSink {
    fn new() -> Self {
        Self {
            input: InputCore::new(),
            name: None,
            seen_low: false,
        }
    }

    fn named(name: &String) -> Self {
        Self {
            input: InputCore::new(),
            name: Some(name.to_string()),
            seen_low: false,
        }
    }
}
impl Receiver for OutputSink {
    fn input_core(&mut self) -> &mut InputCore {
        &mut self.input
    }
    fn process_inputs(&mut self) {
        self.input
            .input_queue
            .drain(..)
            .for_each(|dir_pulse| match dir_pulse.pulse {
                Pulse::Low => self.seen_low = true,
                _ => (),
            });
    }
}

impl CommunicationModule for OutputSink {
    fn name(&self) -> String {
        match &self.name {
            Some(name) => name.to_string(),
            _ => "output".to_string(),
        }
    }

    fn as_receiver(&mut self) -> Option<&mut dyn Receiver> {
        Some(self)
    }

    fn as_sink(&mut self) -> Option<&mut OutputSink> {
        Some(self)
    }
}

struct Circuit {
    io_modules: HashMap<String, Box<dyn CommunicationModule>>,
    low_pulse_count: u32,
    high_pulse_count: u32,
}

impl Circuit {
    fn print(&self) {
        self.io_modules.iter().for_each(|(k, v)| {
            println!("Module: {}, {}", k, v.name());
        })
    }

    fn press_button(&mut self, times: u32) {
        self.low_pulse_count += times;
        match self
            .io_modules
            .get_mut("Broadcaster")
            .expect("Expected to get broadcaster")
            .as_broadcaster()
        {
            Some(broadcaster) => broadcaster.press_button(times),
            _ => panic!("Failed to get broadcaster."),
        }
    }

    fn receivers_still_processing(&mut self) -> bool {
        for io_mod in self.io_modules.values_mut() {
            if let Some(receiver) = io_mod.as_receiver() {
                return receiver.receiver_processing();
            } else {
                return false;
            }
        }
        false
    }
    fn emitters_still_processing(&mut self) -> bool {
        for io_mod in self.io_modules.values_mut() {
            if let Some(emitter) = io_mod.as_emitter() {
                return emitter.emitter_processing();
            } else {
                return false;
            }
        }
        false
    }

    fn still_processing(&mut self) -> bool {
        for io_mod in self.io_modules.values_mut() {
            if io_mod.still_processing() {
                // println!("{} is still processing!", io_mod.name());
                return true;
            }
        }
        false
    }

    fn hookup_inputs(&mut self) {
        let mut inputs_to_destinations: Vec<(String, String)> = vec![];
        for val in self.io_modules.values_mut() {
            let name = val.name();
            if let Some(emitter) = val.as_mut().as_emitter() {
                inputs_to_destinations.extend(
                    emitter
                        .output_core()
                        .outputs
                        .iter()
                        .map(|out| (name.clone(), out.clone()))
                        .collect::<Vec<(String, String)>>(),
                )
            }
        }
        for (emitter_name, receiver_name) in inputs_to_destinations {
            if let Some(receiver) = self
                .io_modules
                .entry(receiver_name.clone())
                .or_insert(Box::new(OutputSink::named(&receiver_name)))
                .as_receiver()
            {
                receiver.add_input(emitter_name);
            }
        }
    }

    fn broadcast_ready(&mut self) {
        if let Some(broadcast) = self
            .io_modules
            .get_mut("Broadcaster")
            .unwrap()
            .as_broadcaster()
        {
            broadcast.set_ready();
        }
    }

    fn clock(&mut self) {
        while self.still_processing() {
            self.tick();
            if self.check_low("rx".to_string()) {
                println!("Finished After {} Presses: ", self.get_num_presses());
            }
        }
    }

    fn tick(&mut self) {
        self.output_tick();
        self.input_tick();
    }

    fn input_tick(&mut self) {
        for val in self.io_modules.values_mut() {
            if let Some(receiver) = val.as_mut().as_receiver() {
                receiver.process_inputs();
            }
        }
    }

    fn count_pulse(&mut self, pulse: &Pulse) {
        match pulse {
            Pulse::High => self.high_pulse_count += 1,
            Pulse::Low => self.low_pulse_count = self.low_pulse_count.wrapping_add(1),
        }
    }

    fn output_tick(&mut self) {
        let pulses = self
            .io_modules
            .values_mut()
            .filter_map(|val| val.as_mut().as_emitter())
            .map(|emitter| emitter.output_pulses())
            .flatten()
            .collect::<Vec<DirectedPulse>>();

        // println!("Pulses: {:?}", pulses);
        if pulses.len() == 0 {
            self.broadcast_ready();
        }

        pulses.into_iter().for_each(|pulse| {
            self.count_pulse(&pulse.pulse);
            // println!("Pulse: {:?}", pulse);
            // self.print();
            if let Some(receiver) = self
                .io_modules
                .get_mut(&pulse.destination)
                .expect("Should be able to find destination")
                .as_receiver()
            {
                receiver.receive_pulse(pulse)
            }
        })
    }

    fn get_num_presses(&mut self) -> u32 {
        if let Some(broadcast) = self
            .io_modules
            .get_mut("Broadcaster")
            .unwrap()
            .as_broadcaster()
        {
            return broadcast.num_presses;
        } else {
            panic!("Should be able to access broadcaster!");
        }
    }
    fn check_low(&mut self, name: String) -> bool {
        if let Some(sink) = self
            .io_modules
            .entry(name.clone())
            .or_insert(Box::new(OutputSink::named(&name)))
            .as_sink()
        {
            return sink.seen_low;
        } else {
            false
        }
    }
}

impl FromStr for Circuit {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let io_modules: Vec<Box<dyn CommunicationModule>> = s
            .lines()
            .map(|l| {
                let (name, outputs) = l.split_once(" -> ").unwrap();
                let mut io_module: Box<dyn CommunicationModule> = match name.trim() {
                    "broadcaster" => Box::new(Broadcaster::new()),
                    "output" => Box::new(OutputSink::new()),
                    module_name => match module_name.chars().nth(0).unwrap() {
                        '&' => Box::new(Conjunction::new(
                            module_name.chars().skip(1).collect::<String>().clone(),
                        )),
                        '%' => Box::new(FlipFlop::new(
                            module_name.chars().skip(1).collect::<String>().clone(),
                        )),
                        _ => {
                            println!("Unknown Module: {}", module_name);
                            panic!("Unknown IO Module")
                        }
                    },
                };

                match io_module.as_emitter() {
                    Some(emitter) => emitter.add_outputs(
                        outputs
                            .split(", ")
                            .map(|v| v.trim().to_string())
                            .collect::<Vec<String>>(),
                    ),
                    _ => (),
                }

                io_module
            })
            .collect::<Vec<Box<dyn CommunicationModule>>>();

        let mut modules_map = HashMap::new();
        for module in io_modules {
            modules_map.insert(module.name(), module);
        }

        modules_map.insert("output".to_string(), Box::new(OutputSink::new()));
        Ok(Circuit {
            io_modules: modules_map,
            high_pulse_count: 0,
            low_pulse_count: 0,
        })
    }
}

#[allow(dead_code)]
fn part1() {
    let input = load_input("part1.txt");
    let mut circuit = input
        .parse::<Circuit>()
        .expect("Expected to be able to parse circuit.");
    circuit.hookup_inputs();
    circuit.press_button(1000);
    // circuit.print();
    circuit.clock();

    // circuit.clock();
    println!(
        "High: {}, Low: {}",
        circuit.high_pulse_count, circuit.low_pulse_count
    );
    let result = circuit.high_pulse_count * circuit.low_pulse_count;

    println!("Part 1 Result: {}", result);
}

#[allow(dead_code)]
fn part2() {
    let input = load_input("part1.txt");
    let mut circuit = input
        .parse::<Circuit>()
        .expect("Expected to be able to parse circuit.");
    circuit.hookup_inputs();
    circuit.press_button(1);
    // circuit.print();
    circuit.clock();

    // circuit.clock();
    println!(
        "High: {}, Low: {}",
        circuit.high_pulse_count, circuit.low_pulse_count
    );
    let result = circuit.high_pulse_count * circuit.low_pulse_count;
    println!("Part 2 Result: {}", result);
}

fn main() {
    part1();
    part2();
}
