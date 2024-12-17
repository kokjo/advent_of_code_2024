use z3::ast::{Ast, BV};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CPU {
    program: Vec<u8>,
    pc: usize,
    a: u64,
    b: u64,
    c: u64,
}

fn parse(input: &str) -> CPU {
    let mut cpu = CPU {
        program: vec![],
        pc: 0,
        a: 0,
        b: 0,
        c: 0,
    };

    for line in input.lines() {
        if let Some((thing, stuff)) = line.trim().split_once(':') {
            match thing {
                "Register A" => {
                    cpu.a = stuff.trim().parse().unwrap();
                },
                "Register B" => {
                    cpu.b = stuff.trim().parse().unwrap();
                },
                "Register C" => {
                    cpu.c = stuff.trim().parse().unwrap();
                },
                "Program" => {
                    cpu.program = stuff.trim().split(',').map(|n| n.parse().unwrap()).collect::<Vec<_>>();
                }
                _ => continue
            }
        }
    }

    cpu
}

impl CPU {
    fn opcode(&self) -> u8 {
        self.program[self.pc + 0]
    }

    fn literal(&self) -> u64 {
        self.program[self.pc + 1] as u64
    }

    fn combo(&self) -> u64 {
        match self.program[self.pc + 1] {
            0 => 0,
            1 => 1,
            2 => 2,
            3 => 3,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("invalid combo")
        }
    }
}


fn part_1(mut cpu: CPU) -> String {
    let mut output = vec![];

    while cpu.pc < cpu.program.len() {
        match cpu.opcode() {
            0 => { cpu.a = cpu.a >> cpu.combo(); },
            1 => { cpu.b = cpu.b ^ cpu.literal(); },
            2 => { cpu.b = cpu.combo() & 7; },
            3 => if cpu.a != 0 {
                cpu.pc = cpu.literal() as usize;
                continue;
            },
            4 => { cpu.b = cpu.b ^ cpu.c; },
            5 => { output.push(cpu.combo() & 7); },
            6 => { cpu.b = cpu.a >> cpu.combo(); },
            7 => { cpu.c = cpu.a >> cpu.combo(); },
            _ => { panic!("invalid opcode"); }
        }
        cpu.pc += 2;
    }

    output.into_iter().map(|o| o.to_string()).collect::<Vec<String>>().join(",")
}

#[derive(Debug, Clone)]
struct SymbCpu<'z3> {
    program: Vec<u8>,
    pc: usize,
    ctx: &'z3 z3::Context,
    slv: z3::Solver<'z3>,
    a: BV<'z3>,
    b: BV<'z3>,
    c: BV<'z3>,
    output: Vec<BV<'z3>>,
}

impl<'z3> SymbCpu<'z3> {
    fn new(ctx: &'z3 z3::Context, program: Vec<u8>) -> Self {
        SymbCpu {
            program: program,
            pc: 0,
            ctx: ctx,
            slv: z3::Solver::new(&ctx),
            a: BV::fresh_const(ctx, "a", 64),
            b: BV::fresh_const(ctx, "a", 64),
            c: BV::fresh_const(ctx, "a", 64),
            output: vec![]
        }
    }

    fn opcode(&self) -> u8 {
        self.program[self.pc + 0]
    }

    fn literal(&self) -> BV<'z3> {
        BV::from_u64(self.ctx, self.program[self.pc + 1] as u64, 64)
    }

    fn combo(&self) -> BV<'z3> {
        match self.program[self.pc + 1] {
            0 => BV::from_u64(self.ctx, 0, 64),
            1 => BV::from_u64(self.ctx, 1, 64),
            2 => BV::from_u64(self.ctx, 2, 64),
            3 => BV::from_u64(self.ctx, 3, 64),
            4 => self.a.clone(),
            5 => self.b.clone(),
            6 => self.c.clone(),
            _ => panic!("invalid combo")
        }
    }
}

fn part_2(cpu: CPU) -> u64 {
    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);

    let cpu = SymbCpu::new(&ctx, cpu.program);

    let a = cpu.a.clone();
    let mut queue = vec![cpu];
    while let Some(mut cpu) = queue.pop() {
        if cpu.output.len() > cpu.program.len() {
            continue;
        }
        if cpu.pc >= cpu.program.len() {
            for (p, o) in cpu.program.iter().zip(cpu.output) {
                cpu.slv.assert(&BV::from_u64(&ctx, *p as u64, 64)._eq(&o));
            }
            if cpu.slv.check() != z3::SatResult::Sat {
                continue;
            }
            let model = cpu.slv.get_model().unwrap();
            let a_value = model.eval(&a, true).unwrap();
            return a_value.as_u64().unwrap();
        }
        match cpu.opcode() {
            0 => { cpu.a = cpu.a.bvlshr(&cpu.combo()); },
            1 => { cpu.b = cpu.b.clone() ^ cpu.literal(); },
            2 => { cpu.b = cpu.combo() & BV::from_u64(&ctx, 7, 64); },
            3 => {
                let cond = cpu.a._eq(&BV::from_u64(cpu.ctx, 0, 64));
                
                let mut not_taken = cpu.clone();
                not_taken.slv.assert(&cond);
                not_taken.pc += 2;
                queue.push(not_taken);

                let mut taken = cpu.clone();
                taken.slv.assert(&cond.not());
                taken.pc = cpu.program[cpu.pc + 1] as usize;
                queue.push(taken);
                continue;
            },
            4 => { cpu.b = cpu.b ^ cpu.c.clone(); },
            5 => { cpu.output.push(cpu.combo() & BV::from_u64(&ctx, 7, 64)); },
            6 => { cpu.b = cpu.a.bvlshr(&cpu.combo()); },
            7 => { cpu.c = cpu.a.bvlshr(&cpu.combo()); },
            _ => { panic!("invalid opcode"); }
        }
        cpu.pc += 2;
        queue.push(cpu);
    }

    panic!("could not find solution for A");
}

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let cpu = parse(&input);
    println!("part_1 = {}", part_1(cpu.clone()));
    println!("part_2 = {}", part_2(cpu.clone()));
}
