use std::{
    fmt,
    io::{self, Read},
    ops::{Index, IndexMut},
};

#[derive(Debug, PartialEq, Clone)]
struct Registers(Vec<u32>);

impl Registers {
    const REGISTERS_COUNT: u32 = 6;

    fn new() -> Self {
        Self(vec![0; Self::REGISTERS_COUNT as usize])
    }
}

impl Index<u32> for Registers {
    type Output = u32;
    fn index(&self, reg: u32) -> &Self::Output {
        assert!((0..Self::REGISTERS_COUNT).contains(&reg));
        &self.0[reg as usize]
    }
}

impl IndexMut<u32> for Registers {
    fn index_mut(&mut self, reg: u32) -> &mut Self::Output {
        assert!((0..Self::REGISTERS_COUNT).contains(&reg));
        &mut self.0[reg as usize]
    }
}

#[derive(Debug)]
struct Instruction {
    opcode: String,
    a: u32,
    b: u32,
    c: u32,
}

impl Instruction {
    fn build(s: &str) -> Self {
        let p: Vec<_> = s.split(' ').collect();
        Self {
            opcode: p[0].to_string(),
            a: p[1].parse().unwrap(),
            b: p[2].parse().unwrap(),
            c: p[3].parse().unwrap(),
        }
    }

    fn exec(&self, regs: &mut Registers) {
        match self.opcode.as_str() {
            "addr" => regs[self.c] = regs[self.a] + regs[self.b],
            "addi" => regs[self.c] = regs[self.a] + self.b,
            "mulr" => regs[self.c] = regs[self.a] * regs[self.b],
            "muli" => regs[self.c] = regs[self.a] * self.b,
            "banr" => regs[self.c] = regs[self.a] & regs[self.b],
            "bani" => regs[self.c] = regs[self.a] & self.b,
            "borr" => regs[self.c] = regs[self.a] | regs[self.b],
            "bori" => regs[self.c] = regs[self.a] | self.b,
            "setr" => regs[self.c] = regs[self.a],
            "seti" => regs[self.c] = self.a,
            "gtir" => regs[self.c] = u32::from(self.a > regs[self.b]),
            "gtri" => regs[self.c] = u32::from(regs[self.a] > self.b),
            "gtrr" => regs[self.c] = u32::from(regs[self.a] > regs[self.b]),
            "eqit" => regs[self.c] = u32::from(self.a == regs[self.b]),
            "eqri" => regs[self.c] = u32::from(regs[self.a] == self.b),
            "eqrr" => regs[self.c] = u32::from(regs[self.a] == regs[self.b]),
            _ => panic!("Invalid opcode {}", self.opcode),
        }
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {} {}", self.opcode, self.a, self.b, self.c)
    }
}

fn build(input: &str) -> (u32, Vec<Instruction>) {
    let mut it = input.lines();
    let ip_binding = it
        .next()
        .unwrap()
        .strip_prefix("#ip ")
        .unwrap()
        .parse()
        .unwrap();
    let mut instructions = Vec::new();
    for line in it {
        instructions.push(Instruction::build(line));
    }
    (ip_binding, instructions)
}

fn exec(
    ip_binding: u32,
    instructions: &[Instruction],
    regs: &mut Registers,
    ip: &mut u32,
    step: usize,
) {
    const DEBUG: bool = false;

    let ins = &instructions[*ip as usize];

    regs[ip_binding] = *ip;
    if DEBUG {
        print!("[{}] ip={} {} {:?}", step, ip, ins, regs);
    }

    ins.exec(regs);

    *ip = regs[ip_binding];
    *ip += 1;
    if DEBUG {
        println!(" {:?}", regs);
    }
}

// All the code above until here is the same (or almost) as day 19.

// Executes the specified instructions, with the given IP binding and registers.
// If set, max_step gives a maximum amount of instructions to execute.
// If the program finishes normally, it will return the number of instructions executed.
fn exec_all<const ENABLE_OPTIMIZATIONS: bool>(
    ip_binding: u32,
    instructions: &[Instruction],
    regs: &mut Registers,
    max_instructions: Option<usize>,
) -> Option<usize> {
    let mut ip: u32 = 0;
    let mut steps_count = 0;

    while ip < instructions.len() as u32 {
        if ENABLE_OPTIMIZATIONS {
            // Optimization of lines 18-25
            if ip == 18 {
                regs[5] = regs[4] / 256;
                ip = 26;
            }
            // Note: Such an optimization can be validated first with following:
            // if ip == 26 {
            //     assert_eq!(regs[5], regs[4] / 256);
            // }
        }

        exec(ip_binding, instructions, regs, &mut ip, steps_count);
        steps_count += 1;

        if let Some(max) = max_instructions {
            if steps_count > max {
                return None;
            }
        }
    }
    Some(steps_count)
}

fn lowest_reg0_causing_halt_fewest_ins(ip_binding: u32, instructions: &[Instruction]) -> u32 {
    // Number of instructions to execute before giving up.
    // This number was found by looking after how many instructions the program stops normally,
    // and then decreasing it so that it runs quickly.
    const MAX_STEPS: Option<usize> = Some(40);
    // Max value of r0 to check.
    const MAX_R0_TO_CHECK: u32 = 10_000_000;
    // To set those two constants, we started with 20_000 ; 100_000, then 2_000 ; 1_000_000
    // then 1_000; 10_000_000 and we could settle with the small 40_000 ; 10_000_000.

    let mut candidate: (u32, usize) = (0, usize::MAX);

    let mut r0 = 1;
    while r0 < MAX_R0_TO_CHECK {
        let mut regs = Registers::new();
        regs[0] = r0;
        if let Some(step_count) = exec_all::<true>(ip_binding, instructions, &mut regs, MAX_STEPS) {
            // normal exit, so found something
            if step_count < candidate.1 {
                candidate = (r0, step_count);
                // println!("Candidate {}; {} steps", candidate.0, candidate.1);
            }
        }

        // if r0 % 100_000 == 0 {
        //     println!("{}: current candidate {; {} steps", r0, candidate.0, candidate.1);
        // }
        r0 += 1;
    }
    // println!("Selected candidate {} use {} steps", candidate.0, candidate.1);

    candidate.0
}

fn part2(ip_binding: u32, instructions: &[Instruction]) -> i64 {
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let (ip_binding, instructions) = build(&input);

    println!(
        "Part 1: {}",
        lowest_reg0_causing_halt_fewest_ins(ip_binding, &instructions)
    );
    println!("Part 2: {}", part2(ip_binding, &instructions));
}
