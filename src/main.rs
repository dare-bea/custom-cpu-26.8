//! Emulator for the system.

use clap::Parser;
use sdl3::{event::Event, keyboard::Keycode};
use std::{error::Error, fmt::Display};

use cpu3v2::{
    register::{ByteRegister, WordRegister},
    system::{self, System},
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the binary to run.
    program_path: std::path::PathBuf,

    /// Logs each instruction to stderr.
    #[arg(short, long)]
    disassembly: bool,

    /// Halts when attempting to execute an instruction from RAM.
    #[arg(long)]
    no_ram_pc: bool,

    /// Print registers when the program halts.
    #[arg(long)]
    debug_registers: bool,

    /// Logs interrupts to stderr.
    #[arg(long)]
    log_interrupts: bool,

    /// Creates an window for visual output.
    #[arg(short, long)]
    windowed: bool,

    /// Set the initial value of the %H register.
    #[arg(alias = "%h", long = "%H")]
    reg_h: Option<u8>,

    /// Set the initial value of the %A register.
    #[arg(alias = "%a", long = "%A")]
    reg_a: Option<u8>,

    /// Set the initial value of the %B register.
    #[arg(alias = "%b", long = "%B")]
    reg_b: Option<u8>,

    /// Set the initial value of the %C register.
    #[arg(alias = "%c", long = "%C")]
    reg_c: Option<u8>,

    /// Set the initial value of the %X register.
    #[arg(alias = "%x", long = "%X")]
    reg_x: Option<u8>,

    /// Set the initial value of the %L register.
    #[arg(alias = "%l", long = "%L")]
    reg_l: Option<u8>,

    /// Set the initial value of the %M register.
    #[arg(alias = "%m", long = "%M")]
    reg_m: Option<u8>,

    /// Set the initial value of the %N register.
    #[arg(alias = "%n", long = "%N")]
    reg_n: Option<u8>,

    /// Set the initial value of the %HA register pair.
    #[arg(alias = "%ha", long = "%HA")]
    reg_ha: Option<u16>,

    /// Set the initial value of the %BC register pair.
    #[arg(alias = "%bc", long = "%BC")]
    reg_bc: Option<u16>,

    /// Set the initial value of the %XL register pair.
    #[arg(alias = "%xl", long = "%XL")]
    reg_xl: Option<u16>,

    /// Set the initial value of the %MN register pair.
    #[arg(alias = "%mn", long = "%MN")]
    reg_mn: Option<u16>,

    /// Set the initial value of the %R4 register.
    #[arg(alias = "%r4", long = "%R4")]
    reg_r4: Option<u16>,

    /// Set the initial value of the %SP register.
    #[arg(alias = "%sp", long = "%SP")]
    reg_sp: Option<u16>,

    /// Set the initial value of the %FL register.
    #[arg(alias = "%fl", long = "%FL")]
    reg_fl: Option<u16>,

    /// Set the initial value of the %PC register.
    #[arg(alias = "%pc", long = "%PC")]
    reg_pc: Option<u16>,
}

#[derive(Debug, Default, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct RamExecution(u16);

impl Display for RamExecution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "attempt to execute instruction at address {0:#x} in RAM",
            self.0
        )
    }
}

impl Error for RamExecution {}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct CannotInitalizeByteRegisterAndRegisterPair(ByteRegister, WordRegister);

impl Display for CannotInitalizeByteRegisterAndRegisterPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "cannot initialize both the %{0} byte register and the %{1} register pair at the same time",
            self.0, self.1
        )
    }
}

impl Error for CannotInitalizeByteRegisterAndRegisterPair {}

#[allow(clippy::too_many_lines)]
fn system_from_args(rom: &[u8], args: &Args) -> Result<System, Box<dyn Error>> {
    let mut system = System::new(Box::from(rom))?;
    if let Some(value) = args.reg_ha {
        if args.reg_h.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::H,
                WordRegister::HA,
            )
            .into());
        }
        if args.reg_a.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::A,
                WordRegister::HA,
            )
            .into());
        }
        system.set_regw(WordRegister::HA, value);
    } else {
        if let Some(value) = args.reg_h {
            system.set_regb(ByteRegister::H, value);
        }
        if let Some(value) = args.reg_a {
            system.set_regb(ByteRegister::A, value);
        }
    }
    if let Some(value) = args.reg_bc {
        if args.reg_b.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::B,
                WordRegister::BC,
            )
            .into());
        }
        if args.reg_c.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::C,
                WordRegister::BC,
            )
            .into());
        }
        system.set_regw(WordRegister::BC, value);
    } else {
        if let Some(value) = args.reg_b {
            system.set_regb(ByteRegister::B, value);
        }
        if let Some(value) = args.reg_c {
            system.set_regb(ByteRegister::C, value);
        }
    }
    if let Some(value) = args.reg_xl {
        if args.reg_x.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::X,
                WordRegister::XL,
            )
            .into());
        }
        if args.reg_l.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::L,
                WordRegister::XL,
            )
            .into());
        }
        system.set_regw(WordRegister::XL, value);
    } else {
        if let Some(value) = args.reg_x {
            system.set_regb(ByteRegister::X, value);
        }
        if let Some(value) = args.reg_l {
            system.set_regb(ByteRegister::L, value);
        }
    }
    if let Some(value) = args.reg_mn {
        if args.reg_m.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::M,
                WordRegister::MN,
            )
            .into());
        }
        if args.reg_n.is_some() {
            return Err(CannotInitalizeByteRegisterAndRegisterPair(
                ByteRegister::N,
                WordRegister::MN,
            )
            .into());
        }
        system.set_regw(WordRegister::MN, value);
    } else {
        if let Some(value) = args.reg_m {
            system.set_regb(ByteRegister::M, value);
        }
        if let Some(value) = args.reg_n {
            system.set_regb(ByteRegister::N, value);
        }
    }
    if let Some(value) = args.reg_r4 {
        system.set_regw(WordRegister::R4, value);
    }
    if let Some(value) = args.reg_sp {
        system.set_regw(WordRegister::SP, value);
    }
    if let Some(value) = args.reg_fl {
        system.set_regw(WordRegister::FL, value);
    }
    if let Some(value) = args.reg_pc {
        system.set_regw(WordRegister::PC, value);
    }
    Ok(system)
}

const CYCLES_PER_FRAME: u32 = 2_000_000 / 60;

pub(crate) struct WindowOutput {
    pub(crate) _sdl_context: sdl3::Sdl,
    pub(crate) _video_subsystem: sdl3::VideoSubsystem,
    pub(crate) event_pump: sdl3::EventPump,
    pub(crate) canvas: sdl3::render::Canvas<sdl3::video::Window>
}

impl WindowOutput {
    fn new() -> Result<Self, Box<dyn Error>> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;
        let event_pump = sdl_context.event_pump()?;
        let window = video_subsystem
            .window(
                "CPU3v2 Emulator",
                256,
                256)
            .position_centered()
            .build()?;
        video_subsystem.text_input().start_with_options(&window, sdl3::keyboard::TextInputOptions {
            autocorrect: Some(false),
            multiline: Some(true),
            ..Default::default()
        })?;
        let mut canvas = window.into_canvas();
        canvas.set_draw_color((0, 0, 255));
        canvas.clear();
        canvas.present();
        Ok(Self {
            _sdl_context: sdl_context, _video_subsystem: video_subsystem, event_pump, canvas
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let mut winout = match args.windowed {
        true => Some(WindowOutput::new()?),
        false => None,
    };

    let rom = std::fs::read(args.program_path.clone())?;
    let mut system = system_from_args(&rom, &args)?;
    let mut next_frame: u32 = CYCLES_PER_FRAME;

    let ret_val = (|| {
        'running: while !system.is_halted() {
            if let Some(ref mut w) = winout {
                for event in w.event_pump.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running;
                        },
                        Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                            system.input_text("\n")?;
                        },
                        Event::KeyDown { keycode: Some(Keycode::Backspace), .. } => {
                            system.input_text("\x08")?;
                        },
                        Event::TextInput { text, .. } => {
                            system.input_text(&text)?;
                        },
                        _ => {}
                    }
                }
            }
            let pc = system.get_regw(WordRegister::PC);
            if args.no_ram_pc && pc < system::PC_START {
                return Err(RamExecution(pc).into());
            }
            if system.cycles() >= next_frame {
                if let Some(ref mut w) = winout {
                    system.render_sdl(&mut w.canvas)?;
                    w.canvas.present();
                }
                std::thread::sleep(std::time::Duration::from_nanos(1_000_000_000 / 60));
                next_frame = system.cycles() + CYCLES_PER_FRAME;
                system.interrupt(0x7FF0)?;
                if args.log_interrupts {
                    eprintln!("{pc:x}: VBLANK interrupt to {:x}", system.get_regw(WordRegister::PC));
                }
                continue 'running;
            }
            let cycles = system.cycles();
            match system.step() {
                Ok(opcode) => {
                    if args.disassembly {
                        eprintln!("{pc:x}: {:<20} ; {:02x?} ; {} cycles", opcode.to_string(), opcode.to_vec(), system.cycles() - cycles);
                    }
                }
                Err(system::SystemError::Halted) => break,
                Err(e) => {
                    eprintln!(
                        "{:x}: {:x?}",
                        pc,
                        (0..4)
                            .map(|x| system.get_memb(pc.wrapping_add(x)))
                            .collect::<Result<Vec<u8>, _>>()?
                    );
                    return Err(e.into());
                }
            }
            // std::thread::sleep(std::time::Duration::from_millis(500));
        }
        Ok(())
    })();
    if args.debug_registers {
        print!(
            "| HA = {0:5} (0x{0:04x}) |   ",
            system.get_regw(WordRegister::HA)
        );
        print!(
            "H = {0:3} (0x{0:02x})   |   ",
            system.get_regb(ByteRegister::H)
        );
        println!("A = {0:3} (0x{0:02x})   |", system.get_regb(ByteRegister::A));
        print!(
            "| BC = {0:5} (0x{0:04x}) |   ",
            system.get_regw(WordRegister::BC)
        );
        print!(
            "B = {0:3} (0x{0:02x})   |   ",
            system.get_regb(ByteRegister::B)
        );
        println!("C = {0:3} (0x{0:02x})   |", system.get_regb(ByteRegister::C));
        print!(
            "| XL = {0:5} (0x{0:04x}) |   ",
            system.get_regw(WordRegister::XL)
        );
        print!(
            "X = {0:3} (0x{0:02x})   |   ",
            system.get_regb(ByteRegister::X)
        );
        println!("L = {0:3} (0x{0:02x})   |", system.get_regb(ByteRegister::L));
        print!(
            "| MN = {0:5} (0x{0:04x}) |   ",
            system.get_regw(WordRegister::MN)
        );
        print!(
            "M = {0:3} (0x{0:02x})   |   ",
            system.get_regb(ByteRegister::M)
        );
        println!("N = {0:3} (0x{0:02x})   |", system.get_regb(ByteRegister::N));
        print!(
            "| R4 = {0:5} (0x{0:04x}) | ",
            system.get_regw(WordRegister::R4)
        );
        print!("SP = 0x{0:04x} | ", system.get_regw(WordRegister::SP));
        print!("FL = 0x{0:04x} | ", system.get_regw(WordRegister::FL));
        println!("PC = 0x{0:04x} |", system.get_regw(WordRegister::PC));
        println!("Cycles: {}", system.cycles())
    }
    ret_val
}
