// This file will not be used for now!
// Jesus, who thought NES ROMs could be so complicated!
// For now, this file will not be attached to the project,
// and the emulator itself will use a *very* simplified iNES implementation.

// I believe NES 2.0 is simply a super-set of the iNES file format?
// Thus, writing a separate implementation for iNES is likely not needed.

use std::num::NonZeroU16;

use super::ROM;

pub enum iNESVersion {
    /// AKA: iNES/.NES
    Ver1,
    /// AKA: NES 2.0
    Ver2,
}

pub fn check_ver(header: [u8; 16]) -> Option<iNESVersion> {
    // if the header starts with NES<EOF>:
    if header[0] == b'N' && header[1] == b'E' && header[2] == b'S' && header[3] == 0x1A {
        // if the byte at offset 7 has bit 2 clear and bit 3 set:
        if (header[7] & 0x0C) == 0x08 {
            return Some(iNESVersion::Ver2);
        }
        return Some(iNESVersion::Ver1);
    }
    None
}

pub enum ConsoleType {
    NES,
    VsSystem,
    Playchoice10,
    DecimalModeFamiclone,
    PlugthroughOrEPSM,
    VT01STN,
    VT02,
    VT03,
    VT32,
    VT369,
    UM6578,
    FamicomNetworkSystem 
}

pub enum VsPPUType {
    RP2C03B,
    RP2C03G,
    RP2C04_0001,
    RP2C04_0002,
    RP2C04_0003,
    RP2C04_0004,
    RC2C03B,
    RC2C03C,
    RC2C05_01,
    RC2C05_02,
    RC2C05_03,
    RC2C05_04,
    RC2C05_05
}

pub enum VsHardwareType {
    Unisystem,
    UnisystemRBI,
    UnisystemTKO,
    UnisystemSuperXevious,
    UnisystemIceClimber,
    Dual,
    DualRaid
}

pub struct iNESInfo {
    version: iNESVersion,
    // Can you specify the ROM size(s) in a more concise way? YES.
    // The format uses only 12-bits for each.
    // Am I going to bother with floating numbers or a separate implementation
    // for decoding exponent-multiplier notation? NO.
    prg_rom_size: u128, 
    chr_rom_size: u128, 

    /// Hard-wired nametable mirroring type:
    /// 
    /// 0: Horizontal (vertical arrangement) or mapper-controlled
    /// 
    /// 1: Vertical (horizontal arrangement)
    hardwired_nametable_mirroring: bool,
    nonvolatile_memory: bool,
    /// 512-byte Trainer
    /// 
    /// 0: Not present
    /// 
    /// 1: Present between Header and PRG-ROM data
    has_trainer: bool,
    /// Not enough IQ to understand this one.
    hardwired_fourscreen_mode: bool,
    console_type: ConsoleType,
    // Similarly to the ROM sizes, these technically can't reach this value in the NES 2.0 spec.
    // All of them are also represented as (64 << 4_bit_shift_count), thus taking a nibble each.
    // I chose to represent the size as Option<NonZeroU16>, since it conveys the meaning better,
    // and takes up the same amount of space as a u16.
    // Due to the bit-shift encoding in the NES 2.0 specification,
    // only valid numbers are the powers of two between 2^6 and 2^13.
    prg_ram_size: Option<NonZeroU16>,
    prg_nvram_size: Option<NonZeroU16>,
    chr_ram_size: Option<NonZeroU16>,
    chr_nvram_size: Option<NonZeroU16>,
    // misc_roms: ??,
    // default_expansion_device: DefaultExpansionDevice
}

impl ROM {
    pub fn load_ines() {
        
    }
}
