#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

// for easier building
macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

// got these color data from
// http://www.thealmightyguru.com/Games/Hacking/Wiki/index.php?title=NES_Palette
#[allow(dead_code)]
const NEW_COLORS: [Color; 0x40] = [
    color!(0x7C, 0x7C, 0x7C),
    color!(0x00, 0x00, 0xFC),
    color!(0x00, 0x00, 0xBC),
    color!(0x44, 0x28, 0xBC),
    color!(0x94, 0x00, 0x84),
    color!(0xA8, 0x00, 0x20),
    color!(0xA8, 0x10, 0x00),
    color!(0x88, 0x14, 0x00),
    color!(0x50, 0x30, 0x00),
    color!(0x00, 0x78, 0x00),
    color!(0x00, 0x68, 0x00),
    color!(0x00, 0x58, 0x00),
    color!(0x00, 0x40, 0x58),
    color!(0x00, 0x00, 0x00),
    color!(0x00, 0x00, 0x00),
    color!(0x00, 0x00, 0x00),
    color!(0xBC, 0xBC, 0xBC),
    color!(0x00, 0x78, 0xF8),
    color!(0x00, 0x58, 0xF8),
    color!(0x68, 0x44, 0xFC),
    color!(0xD8, 0x00, 0xCC),
    color!(0xE4, 0x00, 0x58),
    color!(0xF8, 0x38, 0x00),
    color!(0xE4, 0x5C, 0x10),
    color!(0xAC, 0x7C, 0x00),
    color!(0x00, 0xB8, 0x00),
    color!(0x00, 0xA8, 0x00),
    color!(0x00, 0xA8, 0x44),
    color!(0x00, 0x88, 0x88),
    color!(0x00, 0x00, 0x00),
    color!(0x00, 0x00, 0x00),
    color!(0x00, 0x00, 0x00),
    color!(0xF8, 0xF8, 0xF8),
    color!(0x3C, 0xBC, 0xFC),
    color!(0x68, 0x88, 0xFC),
    color!(0x98, 0x78, 0xF8),
    color!(0xF8, 0x78, 0xF8),
    color!(0xF8, 0x58, 0x98),
    color!(0xF8, 0x78, 0x58),
    color!(0xFC, 0xA0, 0x44),
    color!(0xF8, 0xB8, 0x00),
    color!(0xB8, 0xF8, 0x18),
    color!(0x58, 0xD8, 0x54),
    color!(0x58, 0xF8, 0x98),
    color!(0x00, 0xE8, 0xD8),
    color!(0x78, 0x78, 0x78),
    color!(0x00, 0x00, 0x00),
    color!(0x00, 0x00, 0x00),
    color!(0xFC, 0xFC, 0xFC),
    color!(0xA4, 0xE4, 0xFC),
    color!(0xB8, 0xB8, 0xF8),
    color!(0xD8, 0xB8, 0xF8),
    color!(0xF8, 0xB8, 0xF8),
    color!(0xF8, 0xA4, 0xC0),
    color!(0xF0, 0xD0, 0xB0),
    color!(0xFC, 0xE0, 0xA8),
    color!(0xF8, 0xD8, 0x78),
    color!(0xD8, 0xF8, 0x78),
    color!(0xB8, 0xF8, 0xB8),
    color!(0xB8, 0xF8, 0xD8),
    color!(0x00, 0xFC, 0xFC),
    color!(0xF8, 0xD8, 0xF8),
    color!(0x00, 0x00, 0x00),
    color!(0x00, 0x00, 0x00),
];

#[allow(dead_code)]
const DEFAULT_COLORS: [Color; 0x40] = [
    color!(0x54, 0x54, 0x54),
    color!(0x0, 0x1e, 0x74),
    color!(0x8, 0x10, 0x90),
    color!(0x30, 0x0, 0x88),
    color!(0x44, 0x0, 0x64),
    color!(0x5c, 0x0, 0x30),
    color!(0x54, 0x4, 0x0),
    color!(0x3c, 0x18, 0x0),
    color!(0x20, 0x2a, 0x0),
    color!(0x8, 0x3a, 0x0),
    color!(0x0, 0x40, 0x0),
    color!(0x0, 0x3c, 0x0),
    color!(0x0, 0x32, 0x3c),
    color!(0x0, 0x0, 0x0),
    color!(0x0, 0x0, 0x0),
    color!(0x0, 0x0, 0x0),
    color!(0x98, 0x96, 0x98),
    color!(0x8, 0x4c, 0xc4),
    color!(0x30, 0x32, 0xec),
    color!(0x5c, 0x1e, 0xe4),
    color!(0x88, 0x14, 0xb0),
    color!(0xa0, 0x14, 0x64),
    color!(0x98, 0x22, 0x20),
    color!(0x78, 0x3c, 0x0),
    color!(0x54, 0x5a, 0x0),
    color!(0x28, 0x72, 0x0),
    color!(0x8, 0x7c, 0x0),
    color!(0x0, 0x76, 0x28),
    color!(0x0, 0x66, 0x78),
    color!(0x0, 0x0, 0x0),
    color!(0x0, 0x0, 0x0),
    color!(0x0, 0x0, 0x0),
    color!(0xec, 0xee, 0xec),
    color!(0x4c, 0x9a, 0xec),
    color!(0x78, 0x7c, 0xec),
    color!(0xb0, 0x62, 0xec),
    color!(0xe4, 0x54, 0xec),
    color!(0xec, 0x58, 0xb4),
    color!(0xec, 0x6a, 0x64),
    color!(0xd4, 0x88, 0x20),
    color!(0xa0, 0xaa, 0x0),
    color!(0x74, 0xc4, 0x0),
    color!(0x4c, 0xd0, 0x20),
    color!(0x38, 0xcc, 0x6c),
    color!(0x38, 0xb4, 0xcc),
    color!(0x3c, 0x3c, 0x3c),
    color!(0x0, 0x0, 0x0),
    color!(0x0, 0x0, 0x0),
    color!(0xec, 0xee, 0xec),
    color!(0xa8, 0xcc, 0xec),
    color!(0xbc, 0xbc, 0xec),
    color!(0xd4, 0xb2, 0xec),
    color!(0xec, 0xae, 0xec),
    color!(0xec, 0xae, 0xd4),
    color!(0xec, 0xb4, 0xb0),
    color!(0xe4, 0xc4, 0x90),
    color!(0xcc, 0xd2, 0x78),
    color!(0xb4, 0xde, 0x78),
    color!(0xa8, 0xe2, 0x90),
    color!(0x98, 0xe2, 0xb4),
    color!(0xa0, 0xd6, 0xe4),
    color!(0xa0, 0xa2, 0xa0),
    color!(0x0, 0x0, 0x0),
    color!(0x0, 0x0, 0x0),
];

/// Selects which of the two colors to use
pub const COLORS: [Color; 0x40] = NEW_COLORS;
