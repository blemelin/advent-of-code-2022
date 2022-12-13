use util::{FromLine, FromLines, read, run};

mod util;

fn main() {
    let (t0, input) = run(|| read::<Input, _>("inputs/day10.txt"));
    let (t1, (p1, p2)) = run(|| input.execute());

    println!("Part 1 :\n{}", p1);
    println!("Part 2 :\n{}", p2);
    println!("Time : {} ns", (t0 + t1).as_nanos());
}

#[derive(Debug)]
struct Input {
    program: Program,
}

impl Input {
    fn execute(&self) -> (i64, String) {
        let mut cpu = Cpu::new();
        let mut crt = Crt::<40, 6>::new();
        let mut process = Process::new(&self.program);

        let mut signal = 0;
        let mut cycle = 1;
        loop {
            if (cycle) % 40 == 20 { signal += cycle * cpu.x; }
            cycle += 1;

            crt.tick(&cpu);
            if !process.tick(&mut cpu) { break; }; // Update Cpu last to end a cycle.
        }

        (signal, crt.screen.iter().map(|it| it.iter().collect::<String>()).collect::<Vec<String>>().join("\n"))
    }
}

#[derive(Debug)]
struct Cpu {
    // X register.
    x: i64,
    // Code segment (Current instruction and remaining cycles)
    cs: Option<(Instruction, u64)>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x: 1,
            cs: None,
        }
    }

    fn tick(&mut self, instruction: &Instruction) -> bool {
        self.cs = self.cs.or_else(|| {
            // No current instruction. Set it.
            match instruction {
                Instruction::Noop => Some((*instruction, 0)),
                Instruction::AddX(_) => Some((*instruction, 1)),
            }
        }).and_then(|code_segment| {
            // Has current instruction. Run it.
            match code_segment {
                (Instruction::Noop, 0) => {
                    // Noop instruction ended. Nothing to do.
                    None
                }
                (Instruction::AddX(value), 0) => {
                    // AddX instruction ended. Apply it.
                    self.x += value;
                    None
                }
                (instruction, cycles) => {
                    // Instruction still running. Decrease remaining cycles.
                    Some((instruction, cycles - 1))
                }
            }
        });

        self.cs.is_some()
    }
}

#[derive(Debug)]
struct Process<'a> {
    program: &'a Program,
    program_counter: usize,
}

impl<'a> Process<'a> {
    fn new(program: &'a Program) -> Self {
        Self {
            program,
            program_counter: 0,
        }
    }

    fn tick(&mut self, cpu: &mut Cpu) -> bool {
        if self.program_counter < self.program.len() {
            if !cpu.tick(&self.program[self.program_counter]) {
                self.program_counter += 1;
            }
        }
        self.program_counter < self.program.len()
    }
}

#[derive(Debug)]
struct Crt<const W: usize, const H: usize> {
    // Screen raster.
    screen: [[char; W]; H],
    // Position in the raster.
    position: usize,
}

impl<const W: usize, const H: usize> Crt<W, H> {
    fn new() -> Self {
        Self {
            screen: [['░'; W]; H],
            position: 0,
        }
    }

    fn tick(&mut self, cpu: &Cpu) {
        const SPRITE_LEN: i64 = 3;
        const SPRITE_OFFSET: i64 = SPRITE_LEN / 2;

        let (ray_x, ray_y) = (self.position % W, self.position / W);

        let sprite_x = cpu.x as i64;
        let sprite_range = sprite_x - SPRITE_OFFSET..=sprite_x + SPRITE_OFFSET;
        self.screen[ray_y][ray_x] = if sprite_range.contains(&(ray_x as i64)) {
            '▓'
        } else {
            '░'
        };

        self.position += 1;
    }
}

type Program = Vec<Instruction>;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl FromLines for Input {
    fn from_lines(lines: &[&str]) -> Self {
        let program = lines.iter().map(line_to!(Instruction)).collect();

        Self {
            program
        }
    }
}

impl FromLine for Instruction {
    fn from_line(line: &str) -> Self {
        let parts: Vec<&str> = line.split(' ').collect();
        match parts[..] {
            ["noop"] => Self::Noop,
            ["addx", value] => Self::AddX(i64::from_line(value)),
            _ => panic!("{line} is not a valid instruction")
        }
    }
}