use util::{FromLine, FromLines, read};

mod util;

fn main() {
    // Read data
    let Data(program) = read("inputs/day10.txt");
    let mut cpu = Cpu::new();
    let mut crt = Crt::<40, 6>::new();
    let mut process = Process::new(&program);

    // Part 1
    let mut signal = 0;
    let mut cycle = 1i64;
    loop {
        if (cycle - 20) % 40 == 0 { signal += cycle * cpu.x; }

        crt.tick(&cpu); // Important! Update Crt before Cpu.
        let finished = !process.tick(&mut cpu);

        cycle += 1;
        if finished { break; }
    }
    println!("Part 1 :\n{}", signal);

    // Part 2
    println!("Part 2 :");
    for line in crt.screen {
        println!("{}", line.iter().collect::<String>());
    }
}

#[derive(Debug)]
struct Data(Program);

impl FromLines for Data {
    fn from_lines(lines: &[&str]) -> Self {
        let program = lines.iter().map(line_to!(Instruction)).collect();

        Self(program)
    }
}

type Program = Vec<Instruction>;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    AddX(i64),
}

impl FromLine for Instruction {
    fn from_line(line: &str) -> Self {
        let mut parts = line.split(' ');
        let name = parts.next().expect("instruction should have a name");

        match name {
            "noop" => {
                Self::Noop
            }
            "addx" => {
                let value = parts.next().expect("addx instruction should have a value");
                let value = i64::from_line(value);
                Self::AddX(value)
            }
            _ => {
                panic!("{name} is not a valid instruction")
            }
        }
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

        let ray_x = self.position % W;
        let ray_y = self.position / W;

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