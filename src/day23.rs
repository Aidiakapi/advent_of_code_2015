day!(
    day23,
    "https://adventofcode.com/2015/day/23/input",
    part1,
    part2
);

fn access(regs: &mut [usize; 2], reg: Register) -> &mut usize {
    match reg {
        Register::A => &mut regs[0],
        Register::B => &mut regs[1],
    }
}

fn solve(input: String, start_a: usize) -> Result<usize> {
    let instructions = parse_input(&input)?;
    let mut ip = 0isize;
    let regs = &mut [start_a, 0];

    while ip >= 0 && (ip as usize) < instructions.len() {
        // print!("[{:>6 } {:>6 }] {:>20 }", regs[0], regs[1], format!("{:?}", instructions[ip as usize]));
        match &instructions[ip as usize] {
            Instruction::Half(reg) => {
                *access(regs, *reg) /= 2;
                ip += 1;
            }
            Instruction::Tripple(reg) => {
                *access(regs, *reg) *= 3;
                ip += 1;
            }
            Instruction::Increment(reg) => {
                *access(regs, *reg) += 1;
                ip += 1;
            }
            Instruction::Jump(offset) => ip += offset,
            Instruction::JumpIfEven(reg, offset) => {
                if *access(regs, *reg) % 2 == 0 {
                    ip += offset;
                } else {
                    ip += 1;
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                if *access(regs, *reg) == 1 {
                    ip += offset;
                } else {
                    ip += 1;
                }
            }
        }
        // println!("     [{:>6 } {:>6 }]", regs[0], regs[1]);
    }
    Ok(regs[1])
}

fn part1(input: String) -> Result<usize> {
    solve(input, 0)
}
fn part2(input: String) -> Result<usize> {
    solve(input, 1)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Register {
    A,
    B,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Half(Register),
    Tripple(Register),
    Increment(Register),
    Jump(isize),
    JumpIfEven(Register, isize),
    JumpIfOne(Register, isize),
}

fn parse_input(input: &str) -> Result<Vec<Instruction>> {
    fn parse_register(s: &str) -> Result<Register> {
        if s.len() == 1 {
            match s.as_bytes()[0] {
                b'a' => return Ok(Register::A),
                b'b' => return Ok(Register::B),
                _ => {}
            }
        }
        return Err(Error::Input("invalid register"));
    }
    input
        .split('\n')
        .map(|line| {
            if line.len() < 5 {
                return Err(Error::Input("invalid instruction length"));
            }
            let constructor = match &line[0..4] {
                "hlf " => return Ok(Instruction::Half(parse_register(&line[4..])?)),
                "tpl " => return Ok(Instruction::Tripple(parse_register(&line[4..])?)),
                "inc " => return Ok(Instruction::Increment(parse_register(&line[4..])?)),
                "jmp " => return Ok(Instruction::Jump(line[4..].parse()?)),
                "jie " => Instruction::JumpIfEven,
                "jio " => Instruction::JumpIfOne,
                _ => return Err(Error::Input("invalid instruction")),
            };
            let mut parts = line[4..].split(", ");
            let reg = parse_register(parts.next().unwrap())?;
            let offset = parts
                .next()
                .ok_or(Error::Input("expected offset"))?
                .parse()?;
            Ok(constructor(reg, offset))
        })
        .collect::<Result<_>>()
}
