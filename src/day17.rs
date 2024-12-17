macro_rules! literal {
    ($op:expr) => {
        ($op - b'0') as u64
    };
}
#[derive(Debug)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    pub fn combo(&self, operand: u8) -> u64 {
        match operand {
            v @ b'0'..=b'3' => (v - b'0') as u64,
            b'4' => self.a,
            b'5' => self.b,
            six => {
                debug_assert_eq!(six, b'6');
                self.c
            }
        }
    }
}

// 159ns
pub fn part1(input: &str) -> u64 {
    let (registers, program) = input.split_once("\n\nProgram: ").unwrap();
    let registers: Vec<u64> = registers
        .split('\n')
        .map(|line| line.rsplit_once(' ').unwrap().1.parse().unwrap())
        .collect();
    let [a, b, c] = registers.try_into().unwrap();
    let mut regs = Registers { a, b, c };
    let mut cur_program = program.as_bytes();
    let mut out = Vec::with_capacity(12);
    loop {
        debug_assert!(
            cur_program.is_empty() || (cur_program[1] == b'\n' || cur_program[1] == b',')
        );
        debug_assert!(
            cur_program.is_empty() || (cur_program[3] == b'\n' || cur_program[3] == b',')
        );

        match *cur_program {
            // adv
            [b'0', _, operand, _, ..] => regs.a /= 2u64.pow(regs.combo(operand) as u32),
            // bxl
            [b'1', _, operand, _, ..] => regs.b ^= literal!(operand),
            // bst
            [b'2', _, operand, _, ..] => regs.b = regs.combo(operand) & 0b111,
            // jnz
            [b'3', _, operand, _, ..] => {
                if regs.a != 0 {
                    cur_program = &program.as_bytes()[operand as usize & 0b111..];
                    // Skip the PC increment
                    continue;
                }
            }
            // bxc
            [b'4', _, _, _, ..] => regs.b ^= regs.c,
            // out
            [b'5', _, operand, _, ..] => out.push(regs.combo(operand) & 0b111),
            // bdv
            [b'6', _, operand, _, ..] => regs.b = regs.a / 2u64.pow(regs.combo(operand) as u32),
            [seven, _, operand, _, ..] => {
                debug_assert_eq!(seven, b'7');
                regs.c = regs.a / 2u64.pow(regs.combo(operand) as u32)
            }
            _ => break,
        }
        cur_program = &cur_program[4..];
    }
    // When benchmarking, don't do string printing
    if !*crate::IS_BENCH {
        let out: Vec<_> = out.into_iter().map(|n| n.to_string()).collect();
        println!("{}", out.join(","));
        0
    } else {
        out.len() as u64
    }
}

pub fn part2(_: &str) -> u64 {
    todo!()
}
