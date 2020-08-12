use apu2a03::APU2A03;
use cartridge::{Cartridge, CartridgeError};
use common::{Bus, Device};
use controller::{Controller, StandardNESControllerState};
use cpu6502::CPU6502;
use display::TV;
use ppu2c02::{Palette, VRam, PPU2C02};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::UiProvider;

// NES TV size
// TODO: should be included in "tv" crate
pub const TV_WIDTH: u32 = 256;
pub const TV_HEIGHT: u32 = 240;

struct PPUBus {
    cartridge: Rc<RefCell<Cartridge>>,
    vram: VRam,
    palettes: Palette,
}

struct CPUBus {
    cartridge: Rc<RefCell<Cartridge>>,
    ram: [u8; 0x800],
    ppu: Rc<RefCell<dyn Bus>>,
    apu: Rc<RefCell<dyn Bus>>,
    contoller: Controller,
}

impl CPUBus {
    pub fn new(
        cartridge: Rc<RefCell<Cartridge>>,
        ppu: Rc<RefCell<dyn Bus>>,
        apu: Rc<RefCell<dyn Bus>>,
        contoller: Controller,
    ) -> Self {
        CPUBus {
            cartridge,
            ram: [0; 0x800],
            ppu,
            apu,
            contoller,
        }
    }
}

impl PPUBus {
    pub fn new(cartridge: Rc<RefCell<Cartridge>>) -> Self {
        PPUBus {
            cartridge: cartridge.clone(),
            vram: VRam::new(cartridge.clone()),
            palettes: Palette::new(),
        }
    }
}

impl Bus for PPUBus {
    fn read(&self, address: u16, device: Device) -> u8 {
        match address {
            0x0000..=0x1FFF => self.cartridge.borrow().read(address, device),
            0x2000..=0x3EFF => self.vram.read(address & 0x2FFF, device),
            0x3F00..=0x3FFF => self.palettes.read(address, device),
            // mirror
            0x4000..=0xFFFF => self.read(address & 0x3FFF, device),
        }
    }
    fn write(&mut self, address: u16, data: u8, device: Device) {
        match address {
            0x0000..=0x1FFF => self.cartridge.borrow_mut().write(address, data, device),
            0x2000..=0x3EFF => self.vram.write(address & 0x2FFF, data, device),
            0x3F00..=0x3FFF => self.palettes.write(address, data, device),
            // mirror
            0x4000..=0xFFFF => self.write(address & 0x3FFF, data, device),
        }
    }
}

impl Bus for CPUBus {
    fn read(&self, address: u16, device: Device) -> u8 {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x7FF) as usize],
            0x2000..=0x3FFF => self.ppu.borrow().read(0x2000 | (address & 0x7), device),
            0x4000..=0x4013 => self.apu.borrow().read(address, device),
            0x4014 => self.ppu.borrow().read(address, device),
            0x4015 => self.apu.borrow().read(address, device),
            0x4016 => self.contoller.read(address, device),
            0x4017 => self.apu.borrow().read(address, device),
            0x6000..=0xFFFF => self.cartridge.borrow().read(address, device),
            _ => {
                // println!("unimplemented read cpu from {:04X}", address);
                0
            }
        }
    }
    fn write(&mut self, address: u16, data: u8, device: Device) {
        match address {
            0x0000..=0x1FFF => self.ram[(address & 0x7FF) as usize] = data,
            0x2000..=0x3FFF => self
                .ppu
                .borrow_mut()
                .write(0x2000 | (address & 0x7), data, device),
            0x4000..=0x4013 => self.apu.borrow_mut().write(address, data, device),
            0x4014 => self.ppu.borrow_mut().write(address, data, device),
            0x4015 => self.apu.borrow_mut().write(address, data, device),
            0x4016 => self.contoller.write(address, data, device),
            0x4017 => self.apu.borrow_mut().write(address, data, device),
            0x6000..=0xFFFF => self
                .cartridge
                .borrow_mut()
                .write(address, data, Device::CPU),
            _ => {} // println!("unimplemented write cpu to {:04X}", address),
        };
    }
}

pub struct NES<P: UiProvider + Send + 'static> {
    cpu: CPU6502<CPUBus>,
    ppu: Rc<RefCell<PPU2C02<PPUBus>>>,
    apu: Rc<RefCell<APU2A03>>,
    image: Arc<Mutex<Vec<u8>>>,
    ctrl_state: Arc<Mutex<StandardNESControllerState>>,

    ui: Option<P>, // just to hold the UI object (it will be taken in the main loop)
}

impl<P: UiProvider + Send + 'static> NES<P> {
    pub fn new(filename: &str, ui: P) -> Result<Self, CartridgeError> {
        let cartridge = Cartridge::from_file(filename)?;
        let cartridge = Rc::new(RefCell::new(cartridge));
        let ppubus = PPUBus::new(cartridge.clone());

        let tv = TV::new(TV_WIDTH, TV_HEIGHT, P::get_tv_color_converter());
        let image = tv.get_image_clone();

        let ppu = PPU2C02::new(ppubus, tv);

        let ppu = Rc::new(RefCell::new(ppu));

        let apu = Rc::new(RefCell::new(APU2A03::new()));

        let ctrl = Controller::new();
        let ctrl_state = ctrl.get_primary_controller_state();

        let cpubus = CPUBus::new(cartridge.clone(), ppu.clone(), apu.clone(), ctrl);

        let mut cpu = CPU6502::new(Rc::new(RefCell::new(cpubus)), ppu.clone());
        cpu.add_irq_provider(cartridge.clone());
        cpu.add_irq_provider(apu.clone());

        Ok(Self {
            cpu,
            ppu,
            apu,
            image,
            ctrl_state,
            ui: Some(ui),
        })
    }

    /// calculate a new view based on the window size
    pub fn run(&mut self) {
        let image = self.image.clone();
        let ctrl_state = self.ctrl_state.clone();

        let (tx, rx) = std::sync::mpsc::channel::<bool>();

        let mut ui = self.ui.take().unwrap();

        std::thread::spawn(move || {
            ui.run_ui_loop(image, ctrl_state);
            tx.send(true).unwrap();
        });

        self.cpu.reset();
        // Run the sound thread
        self.apu.borrow().play();

        let mut last = std::time::Instant::now();
        const CPU_FREQ: f64 = 1.789773 * 1E6;
        const N: usize = 2000; // number of CPU cycles per loop, lower is smoother
        const CPU_PER_CYCLE_NANOS: f64 = 1E9 / CPU_FREQ;
        let mut apu_clock = false;

        // run the emulator loop
        while let Err(_) = rx.try_recv() {
            for _ in 0..N {
                self.cpu.run_next();
                if apu_clock {
                    self.apu.borrow_mut().clock();
                }
                apu_clock = !apu_clock;

                let mut ppu = self.ppu.borrow_mut();
                ppu.clock();
                ppu.clock();
                ppu.clock();
            }

            if let Some(d) =
                std::time::Duration::from_nanos((CPU_PER_CYCLE_NANOS * N as f64) as u64)
                    .checked_sub(last.elapsed())
            {
                std::thread::sleep(d);
            }

            last = std::time::Instant::now();
        }
    }
}