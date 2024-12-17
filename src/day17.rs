use either::Either;

macro_rules! literal {
    ($op:expr) => {
        ($op - b'0') as u64
    };
}

// 73ns
pub fn part1(input: &str) -> Either<u64, String> {
    let input = input.as_bytes();
    let (regs, program) = parse_regs(input);
    Either::Right(run_vm(regs, program))
}

pub fn part2(_: &str) -> Either<u64, String> {
    todo!()
}

// Dummy 'D' register for quick indexing
#[derive(Clone, Copy)]
pub struct Registers([u64; 4]);

impl Registers {
    #[inline]
    pub fn combo(&self, operand: u8) -> u64 {
        if operand < b'4' {
            debug_assert!(operand >= b'0' && operand <= b'3');
            (operand - b'0') as u64
        } else {
            debug_assert!(operand >= b'4' && operand < b'7');
            self.0[((operand - b'4') as usize) & 0b11]
        }
    }
}

fn parse_regs(input: &[u8]) -> (Registers, &[u8]) {
    let mut regs = Registers([0; 4]);
    let mut inp = memchr::memchr_iter(b':', input);

    regs.0[0] = {
        let slice = &input[inp.next().unwrap() + 2..];
        slice
            .iter()
            .take_while(|&&v| v != b'\n')
            .fold(0, |acc, v| acc * 10 + (v - b'0') as u64)
    };

    regs.0[1] = {
        let slice = &input[inp.next().unwrap() + 2..];
        slice
            .iter()
            .take_while(|&&v| v != b'\n')
            .fold(0, |acc, v| acc * 10 + (v - b'0') as u64)
    };

    regs.0[2] = {
        let slice = &input[inp.next().unwrap() + 2..];
        slice
            .iter()
            .take_while(|&&v| v != b'\n')
            .fold(0, |acc, v| acc * 10 + (v - b'0') as u64)
    };

    let program = &input[inp.next().unwrap() + 2..];
    (regs, program)
}

fn run_vm(mut regs: Registers, program: &[u8]) -> String {
    let mut cur_program = program;
    let mut out = String::with_capacity(60);
    loop {
        debug_assert!(
            cur_program.is_empty() || (cur_program[1] == b'\n' || cur_program[1] == b',')
        );
        debug_assert!(
            cur_program.is_empty() || (cur_program[3] == b'\n' || cur_program[3] == b',')
        );

        match *cur_program {
            // adv
            [b'0', _, operand, _, ..] => regs.0[0] >>= regs.combo(operand),
            // bxl
            [b'1', _, operand, _, ..] => regs.0[1] ^= literal!(operand),
            // bst
            [b'2', _, operand, _, ..] => regs.0[1] = regs.combo(operand) & 0b111,
            // jnz
            [b'3', _, operand, _, ..] => {
                if regs.0[0] != 0 {
                    cur_program = &program[operand as usize & 0b111..];
                    // Skip the PC increment
                    continue;
                }
            }
            // bxc
            [b'4', _, _, _, ..] => regs.0[1] ^= regs.0[2],
            // out
            [b'5', _, operand, _, ..] => {
                // Prevent growing/alloc failure code paths
                assert!(out.len() + 2 <= out.capacity());
                out.push(((regs.combo(operand) & 0b111) as u8 + b'0') as char);
                out.push(',');
            }
            // bdv
            [b'6', _, operand, _, ..] => regs.0[1] = regs.0[0] >> regs.combo(operand),
            [seven, _, operand, _, ..] => {
                debug_assert_eq!(seven, b'7');
                regs.0[2] = regs.0[0] >> regs.combo(operand)
            }
            _ => break,
        }
        cur_program = &cur_program[4..];
    }
    out.pop();
    out
}
