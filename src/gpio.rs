#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    gpio000: Gpio000,
    gpio004: Gpio004,
    gpio008: Gpio008,
    gpio00c: Gpio00c,
    gpio010: Gpio010,
    gpio014: Gpio014,
    gpio018: Gpio018,
    gpio01c: Gpio01c,
    gpio020: Gpio020,
    gpio024: Gpio024,
    gpio028: Gpio028,
    gpio02c: Gpio02c,
    gpio030: Gpio030,
    gpio034: Gpio034,
    gpio038: Gpio038,
    gpio03c: Gpio03c,
    gpio040: Gpio040,
    gpio044: Gpio044,
    gpio048: Gpio048,
    gpio04c: Gpio04c,
    gpio050: Gpio050,
    gpio054: Gpio054,
    gpio058: Gpio058,
    _reserved23: [u8; 0x04],
    gpio060: Gpio060,
    gpio064: Gpio064,
    gpio068: Gpio068,
    gpio06c: Gpio06c,
    gpio070: Gpio070,
    gpio074: Gpio074,
    gpio078: Gpio078,
    gpio07c: Gpio07c,
    gpio080: Gpio080,
    gpio084: Gpio084,
    gpio088: Gpio088,
    gpio08c: Gpio08c,
    gpio090: Gpio090,
    gpio094: Gpio094,
    gpio098: Gpio098,
    gpio09c: Gpio09c,
    gpio0a0: Gpio0a0,
    gpio0a4: Gpio0a4,
    gpio0a8: Gpio0a8,
    gpio0ac: Gpio0ac,
    gpio0b0: Gpio0b0,
    gpio0b4: Gpio0b4,
    gpio0b8: Gpio0b8,
    _reserved46: [u8; 0x04],
    gpio0c0: Gpio0c0,
    gpio0c4: Gpio0c4,
    gpio0c8: Gpio0c8,
    gpio0cc: Gpio0cc,
    gpio0d0: Gpio0d0,
    gpio0d4: Gpio0d4,
    _reserved52: [u8; 0x08],
    gpio0e0: Gpio0e0,
    gpio0e4: Gpio0e4,
    gpio0e8: Gpio0e8,
    gpio0ec: Gpio0ec,
    gpio0f0: Gpio0f0,
    gpio0f4: Gpio0f4,
    gpio0f8: Gpio0f8,
    gpio0fc: Gpio0fc,
    gpio100: Gpio100,
    gpio104: Gpio104,
    gpio108: Gpio108,
    _reserved63: [u8; 0x04],
    gpio110: Gpio110,
    gpio114: Gpio114,
    gpio118: Gpio118,
    gpio11c: Gpio11c,
    gpio120: Gpio120,
    gpio124: Gpio124,
    gpio128: Gpio128,
    gpio12c: Gpio12c,
    gpio130: Gpio130,
    gpio134: Gpio134,
    gpio138: Gpio138,
    _reserved74: [u8; 0x04],
    gpio140: Gpio140,
    gpio144: Gpio144,
    gpio148: Gpio148,
    gpio14c: Gpio14c,
    gpio150: Gpio150,
    gpio154: Gpio154,
    gpio158: Gpio158,
    gpio15c: Gpio15c,
    gpio160: Gpio160,
    gpio164: Gpio164,
    gpio168: Gpio168,
    _reserved85: [u8; 0x64],
    gpio1d0: Gpio1d0,
    gpio1d4: Gpio1d4,
    _reserved87: [u8; 0xd4],
    gpio2ac: Gpio2ac,
    _reserved88: [u8; 0x0250],
    gpio500: Gpio500,
}
impl RegisterBlock {
    #[doc = "0x00 - GPIO\\_A/B/C/D Data Value Register"]
    #[inline(always)]
    pub const fn gpio000(&self) -> &Gpio000 {
        &self.gpio000
    }
    #[doc = "0x04 - GPIO\\_A/B/C/D Direction Register"]
    #[inline(always)]
    pub const fn gpio004(&self) -> &Gpio004 {
        &self.gpio004
    }
    #[doc = "0x08 - GPIO\\_A/B/C/D Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio008(&self) -> &Gpio008 {
        &self.gpio008
    }
    #[doc = "0x0c - GPIO\\_A/B/C/D Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio00c(&self) -> &Gpio00c {
        &self.gpio00c
    }
    #[doc = "0x10 - GPIO\\_A/B/C/D Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio010(&self) -> &Gpio010 {
        &self.gpio010
    }
    #[doc = "0x14 - GPIO\\_A/B/C/D Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio014(&self) -> &Gpio014 {
        &self.gpio014
    }
    #[doc = "0x18 - GPIO\\_A/B/C/D Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio018(&self) -> &Gpio018 {
        &self.gpio018
    }
    #[doc = "0x1c - GPIO\\_A/B/C/D Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio01c(&self) -> &Gpio01c {
        &self.gpio01c
    }
    #[doc = "0x20 - GPIO\\_E/F/G/H Data Value Register"]
    #[inline(always)]
    pub const fn gpio020(&self) -> &Gpio020 {
        &self.gpio020
    }
    #[doc = "0x24 - GPIO\\_E/F/G/H Direction Register"]
    #[inline(always)]
    pub const fn gpio024(&self) -> &Gpio024 {
        &self.gpio024
    }
    #[doc = "0x28 - GPIO\\_E/F/G/H Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio028(&self) -> &Gpio028 {
        &self.gpio028
    }
    #[doc = "0x2c - GPIO\\_E/F/G/H Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio02c(&self) -> &Gpio02c {
        &self.gpio02c
    }
    #[doc = "0x30 - GPIO\\_E/F/G/H Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio030(&self) -> &Gpio030 {
        &self.gpio030
    }
    #[doc = "0x34 - GPIO\\_E/F/G/H Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio034(&self) -> &Gpio034 {
        &self.gpio034
    }
    #[doc = "0x38 - GPIO\\_E/F/G/H Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio038(&self) -> &Gpio038 {
        &self.gpio038
    }
    #[doc = "0x3c - GPIO\\_E/F/G/H Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio03c(&self) -> &Gpio03c {
        &self.gpio03c
    }
    #[doc = "0x40 - GPIO\\_A/B/C/D Debounce Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio040(&self) -> &Gpio040 {
        &self.gpio040
    }
    #[doc = "0x44 - GPIO\\_A/B/C/D Debounce Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio044(&self) -> &Gpio044 {
        &self.gpio044
    }
    #[doc = "0x48 - GPIO\\_E/F/G/H Debounce Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio048(&self) -> &Gpio048 {
        &self.gpio048
    }
    #[doc = "0x4c - GPIO\\_E/F/G/H Debounce Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio04c(&self) -> &Gpio04c {
        &self.gpio04c
    }
    #[doc = "0x50 - Debounce Timer Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio050(&self) -> &Gpio050 {
        &self.gpio050
    }
    #[doc = "0x54 - Debounce Timer Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio054(&self) -> &Gpio054 {
        &self.gpio054
    }
    #[doc = "0x58 - Debounce Timer Setting Register \\#3"]
    #[inline(always)]
    pub const fn gpio058(&self) -> &Gpio058 {
        &self.gpio058
    }
    #[doc = "0x60 - GPIO\\_A/B/C/D Command Source 0"]
    #[inline(always)]
    pub const fn gpio060(&self) -> &Gpio060 {
        &self.gpio060
    }
    #[doc = "0x64 - GPIO\\_A/B/C/D Command Source 1"]
    #[inline(always)]
    pub const fn gpio064(&self) -> &Gpio064 {
        &self.gpio064
    }
    #[doc = "0x68 - GPIO\\_E/F/G/H Command Source 0"]
    #[inline(always)]
    pub const fn gpio068(&self) -> &Gpio068 {
        &self.gpio068
    }
    #[doc = "0x6c - GPIO\\_E/F/G/H Command Source 1"]
    #[inline(always)]
    pub const fn gpio06c(&self) -> &Gpio06c {
        &self.gpio06c
    }
    #[doc = "0x70 - GPIO\\_I/J/K/L Data Value Register"]
    #[inline(always)]
    pub const fn gpio070(&self) -> &Gpio070 {
        &self.gpio070
    }
    #[doc = "0x74 - GPIO\\_I/J/K/L Direction Register"]
    #[inline(always)]
    pub const fn gpio074(&self) -> &Gpio074 {
        &self.gpio074
    }
    #[doc = "0x78 - GPIO\\_M/N/O/P Data Value Register"]
    #[inline(always)]
    pub const fn gpio078(&self) -> &Gpio078 {
        &self.gpio078
    }
    #[doc = "0x7c - GPIO\\_M/N/O/P Direction Register"]
    #[inline(always)]
    pub const fn gpio07c(&self) -> &Gpio07c {
        &self.gpio07c
    }
    #[doc = "0x80 - GPIO\\_Q/R/S/T Data Value Register"]
    #[inline(always)]
    pub const fn gpio080(&self) -> &Gpio080 {
        &self.gpio080
    }
    #[doc = "0x84 - GPIO\\_Q/R/S/T Direction Register"]
    #[inline(always)]
    pub const fn gpio084(&self) -> &Gpio084 {
        &self.gpio084
    }
    #[doc = "0x88 - GPIO\\_U Data Value Register"]
    #[inline(always)]
    pub const fn gpio088(&self) -> &Gpio088 {
        &self.gpio088
    }
    #[doc = "0x8c - GPIO\\_U Direction Register"]
    #[inline(always)]
    pub const fn gpio08c(&self) -> &Gpio08c {
        &self.gpio08c
    }
    #[doc = "0x90 - GPIO\\_I/J/K/L Command Source 0"]
    #[inline(always)]
    pub const fn gpio090(&self) -> &Gpio090 {
        &self.gpio090
    }
    #[doc = "0x94 - GPIO\\_I/J/K/L Command Source 1"]
    #[inline(always)]
    pub const fn gpio094(&self) -> &Gpio094 {
        &self.gpio094
    }
    #[doc = "0x98 - GPIO\\_I/J/K/L Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio098(&self) -> &Gpio098 {
        &self.gpio098
    }
    #[doc = "0x9c - GPIO\\_I/J/K/L Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio09c(&self) -> &Gpio09c {
        &self.gpio09c
    }
    #[doc = "0xa0 - GPIO\\_I/J/K/L Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio0a0(&self) -> &Gpio0a0 {
        &self.gpio0a0
    }
    #[doc = "0xa4 - GPIO\\_I/J/K/L Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio0a4(&self) -> &Gpio0a4 {
        &self.gpio0a4
    }
    #[doc = "0xa8 - GPIO\\_I/J/K/L Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio0a8(&self) -> &Gpio0a8 {
        &self.gpio0a8
    }
    #[doc = "0xac - GPIO\\_I/J/K/L Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio0ac(&self) -> &Gpio0ac {
        &self.gpio0ac
    }
    #[doc = "0xb0 - GPIO\\_I/J/K/L Debounce Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio0b0(&self) -> &Gpio0b0 {
        &self.gpio0b0
    }
    #[doc = "0xb4 - GPIO\\_I/J/K/L Debounce Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio0b4(&self) -> &Gpio0b4 {
        &self.gpio0b4
    }
    #[doc = "0xb8 - GPIO\\_I/J/K/L Input Mask Register"]
    #[inline(always)]
    pub const fn gpio0b8(&self) -> &Gpio0b8 {
        &self.gpio0b8
    }
    #[doc = "0xc0 - GPIO\\_A/B/C/D Data Read Register"]
    #[inline(always)]
    pub const fn gpio0c0(&self) -> &Gpio0c0 {
        &self.gpio0c0
    }
    #[doc = "0xc4 - GPIO\\_E/F/G/H Data Read Register"]
    #[inline(always)]
    pub const fn gpio0c4(&self) -> &Gpio0c4 {
        &self.gpio0c4
    }
    #[doc = "0xc8 - GPIO\\_I/J/K/L Data Read Register"]
    #[inline(always)]
    pub const fn gpio0c8(&self) -> &Gpio0c8 {
        &self.gpio0c8
    }
    #[doc = "0xcc - GPIO\\_M/N/O/P Data Read Register"]
    #[inline(always)]
    pub const fn gpio0cc(&self) -> &Gpio0cc {
        &self.gpio0cc
    }
    #[doc = "0xd0 - GPIO\\_Q/R/S/T Data Read Register"]
    #[inline(always)]
    pub const fn gpio0d0(&self) -> &Gpio0d0 {
        &self.gpio0d0
    }
    #[doc = "0xd4 - GPIO\\_U Data Read Register"]
    #[inline(always)]
    pub const fn gpio0d4(&self) -> &Gpio0d4 {
        &self.gpio0d4
    }
    #[doc = "0xe0 - GPIO\\_M/N/O/P Command Source 0"]
    #[inline(always)]
    pub const fn gpio0e0(&self) -> &Gpio0e0 {
        &self.gpio0e0
    }
    #[doc = "0xe4 - GPIO\\_M/N/O/P Command Source 1"]
    #[inline(always)]
    pub const fn gpio0e4(&self) -> &Gpio0e4 {
        &self.gpio0e4
    }
    #[doc = "0xe8 - GPIO\\_M/N/O/P Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio0e8(&self) -> &Gpio0e8 {
        &self.gpio0e8
    }
    #[doc = "0xec - GPIO\\_M/N/O/P Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio0ec(&self) -> &Gpio0ec {
        &self.gpio0ec
    }
    #[doc = "0xf0 - GPIO\\_M/N/O/P Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio0f0(&self) -> &Gpio0f0 {
        &self.gpio0f0
    }
    #[doc = "0xf4 - GPIO\\_M/N/O/P Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio0f4(&self) -> &Gpio0f4 {
        &self.gpio0f4
    }
    #[doc = "0xf8 - GPIO\\_M/N/O/P Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio0f8(&self) -> &Gpio0f8 {
        &self.gpio0f8
    }
    #[doc = "0xfc - GPIO\\_M/N/O/P Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio0fc(&self) -> &Gpio0fc {
        &self.gpio0fc
    }
    #[doc = "0x100 - GPIO\\_M/N/O/P Debounce Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio100(&self) -> &Gpio100 {
        &self.gpio100
    }
    #[doc = "0x104 - GPIO\\_M/N/O/P Debounce Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio104(&self) -> &Gpio104 {
        &self.gpio104
    }
    #[doc = "0x108 - GPIO\\_M/N/O/P Input Mask Register"]
    #[inline(always)]
    pub const fn gpio108(&self) -> &Gpio108 {
        &self.gpio108
    }
    #[doc = "0x110 - GPIO\\_Q/R/S/T Command Source 0"]
    #[inline(always)]
    pub const fn gpio110(&self) -> &Gpio110 {
        &self.gpio110
    }
    #[doc = "0x114 - GPIO\\_Q/R/S/T Command Source 1"]
    #[inline(always)]
    pub const fn gpio114(&self) -> &Gpio114 {
        &self.gpio114
    }
    #[doc = "0x118 - GPIO\\_Q/R/S/T Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio118(&self) -> &Gpio118 {
        &self.gpio118
    }
    #[doc = "0x11c - GPIO\\_Q/R/S/T Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio11c(&self) -> &Gpio11c {
        &self.gpio11c
    }
    #[doc = "0x120 - GPIO\\_Q/R/S/T Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio120(&self) -> &Gpio120 {
        &self.gpio120
    }
    #[doc = "0x124 - GPIO\\_Q/R/S/T Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio124(&self) -> &Gpio124 {
        &self.gpio124
    }
    #[doc = "0x128 - GPIO\\_Q/R/S/T Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio128(&self) -> &Gpio128 {
        &self.gpio128
    }
    #[doc = "0x12c - GPIO\\_Q/R/S/T Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio12c(&self) -> &Gpio12c {
        &self.gpio12c
    }
    #[doc = "0x130 - GPIO\\_Q/R/S/T Debounce Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio130(&self) -> &Gpio130 {
        &self.gpio130
    }
    #[doc = "0x134 - GPIO\\_Q/R/S/T Debounce Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio134(&self) -> &Gpio134 {
        &self.gpio134
    }
    #[doc = "0x138 - GPIO\\_Q/R/S/T Input Mask Register"]
    #[inline(always)]
    pub const fn gpio138(&self) -> &Gpio138 {
        &self.gpio138
    }
    #[doc = "0x140 - GPIO\\_U Command Source 0"]
    #[inline(always)]
    pub const fn gpio140(&self) -> &Gpio140 {
        &self.gpio140
    }
    #[doc = "0x144 - GPIO\\_U Command Source 1"]
    #[inline(always)]
    pub const fn gpio144(&self) -> &Gpio144 {
        &self.gpio144
    }
    #[doc = "0x148 - GPIO\\_U Interrupt Enable Register"]
    #[inline(always)]
    pub const fn gpio148(&self) -> &Gpio148 {
        &self.gpio148
    }
    #[doc = "0x14c - GPIO\\_U Interrupt Sensitivity Type 0 Register"]
    #[inline(always)]
    pub const fn gpio14c(&self) -> &Gpio14c {
        &self.gpio14c
    }
    #[doc = "0x150 - GPIO\\_U Interrupt Sensitivity Type 1 Register"]
    #[inline(always)]
    pub const fn gpio150(&self) -> &Gpio150 {
        &self.gpio150
    }
    #[doc = "0x154 - GPIO\\_U Interrupt Sensitivity Type 2 Register"]
    #[inline(always)]
    pub const fn gpio154(&self) -> &Gpio154 {
        &self.gpio154
    }
    #[doc = "0x158 - GPIO\\_U Interrupt Status Register"]
    #[inline(always)]
    pub const fn gpio158(&self) -> &Gpio158 {
        &self.gpio158
    }
    #[doc = "0x15c - GPIO\\_U Reset Tolerant Register"]
    #[inline(always)]
    pub const fn gpio15c(&self) -> &Gpio15c {
        &self.gpio15c
    }
    #[doc = "0x160 - GPIO\\_U Debounce Setting Register \\#1"]
    #[inline(always)]
    pub const fn gpio160(&self) -> &Gpio160 {
        &self.gpio160
    }
    #[doc = "0x164 - GPIO\\_U Debounce Setting Register \\#2"]
    #[inline(always)]
    pub const fn gpio164(&self) -> &Gpio164 {
        &self.gpio164
    }
    #[doc = "0x168 - GPIO\\_U Input Mask Register"]
    #[inline(always)]
    pub const fn gpio168(&self) -> &Gpio168 {
        &self.gpio168
    }
    #[doc = "0x1d0 - GPIO\\_A/B/C/D Input Mask Register"]
    #[inline(always)]
    pub const fn gpio1d0(&self) -> &Gpio1d0 {
        &self.gpio1d0
    }
    #[doc = "0x1d4 - GPIO\\_E/F/G/H Input Mask Register"]
    #[inline(always)]
    pub const fn gpio1d4(&self) -> &Gpio1d4 {
        &self.gpio1d4
    }
    #[doc = "0x2ac - GPIO Index Register"]
    #[inline(always)]
    pub const fn gpio2ac(&self) -> &Gpio2ac {
        &self.gpio2ac
    }
    #[doc = "0x500 - Serial GPIO\\_A/B/C/D 1 Data Value Register"]
    #[inline(always)]
    pub const fn gpio500(&self) -> &Gpio500 {
        &self.gpio500
    }
}
#[doc = "GPIO000 (rw) register accessor: GPIO\\_A/B/C/D Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio000`] module"]
#[doc(alias = "GPIO000")]
pub type Gpio000 = crate::Reg<gpio000::Gpio000Spec>;
#[doc = "GPIO\\_A/B/C/D Data Value Register"]
pub mod gpio000;
#[doc = "GPIO004 (rw) register accessor: GPIO\\_A/B/C/D Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio004`] module"]
#[doc(alias = "GPIO004")]
pub type Gpio004 = crate::Reg<gpio004::Gpio004Spec>;
#[doc = "GPIO\\_A/B/C/D Direction Register"]
pub mod gpio004;
#[doc = "GPIO008 (rw) register accessor: GPIO\\_A/B/C/D Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio008`] module"]
#[doc(alias = "GPIO008")]
pub type Gpio008 = crate::Reg<gpio008::Gpio008Spec>;
#[doc = "GPIO\\_A/B/C/D Interrupt Enable Register"]
pub mod gpio008;
#[doc = "GPIO00C (rw) register accessor: GPIO\\_A/B/C/D Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio00c`] module"]
#[doc(alias = "GPIO00C")]
pub type Gpio00c = crate::Reg<gpio00c::Gpio00cSpec>;
#[doc = "GPIO\\_A/B/C/D Interrupt Sensitivity Type 0 Register"]
pub mod gpio00c;
#[doc = "GPIO010 (rw) register accessor: GPIO\\_A/B/C/D Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio010`] module"]
#[doc(alias = "GPIO010")]
pub type Gpio010 = crate::Reg<gpio010::Gpio010Spec>;
#[doc = "GPIO\\_A/B/C/D Interrupt Sensitivity Type 1 Register"]
pub mod gpio010;
#[doc = "GPIO014 (rw) register accessor: GPIO\\_A/B/C/D Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio014`] module"]
#[doc(alias = "GPIO014")]
pub type Gpio014 = crate::Reg<gpio014::Gpio014Spec>;
#[doc = "GPIO\\_A/B/C/D Interrupt Sensitivity Type 2 Register"]
pub mod gpio014;
#[doc = "GPIO018 (rw) register accessor: GPIO\\_A/B/C/D Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio018`] module"]
#[doc(alias = "GPIO018")]
pub type Gpio018 = crate::Reg<gpio018::Gpio018Spec>;
#[doc = "GPIO\\_A/B/C/D Interrupt Status Register"]
pub mod gpio018;
#[doc = "GPIO01C (rw) register accessor: GPIO\\_A/B/C/D Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio01c`] module"]
#[doc(alias = "GPIO01C")]
pub type Gpio01c = crate::Reg<gpio01c::Gpio01cSpec>;
#[doc = "GPIO\\_A/B/C/D Reset Tolerant Register"]
pub mod gpio01c;
#[doc = "GPIO020 (rw) register accessor: GPIO\\_E/F/G/H Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio020`] module"]
#[doc(alias = "GPIO020")]
pub type Gpio020 = crate::Reg<gpio020::Gpio020Spec>;
#[doc = "GPIO\\_E/F/G/H Data Value Register"]
pub mod gpio020;
#[doc = "GPIO024 (rw) register accessor: GPIO\\_E/F/G/H Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio024`] module"]
#[doc(alias = "GPIO024")]
pub type Gpio024 = crate::Reg<gpio024::Gpio024Spec>;
#[doc = "GPIO\\_E/F/G/H Direction Register"]
pub mod gpio024;
#[doc = "GPIO028 (rw) register accessor: GPIO\\_E/F/G/H Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio028::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio028::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio028`] module"]
#[doc(alias = "GPIO028")]
pub type Gpio028 = crate::Reg<gpio028::Gpio028Spec>;
#[doc = "GPIO\\_E/F/G/H Interrupt Enable Register"]
pub mod gpio028;
#[doc = "GPIO02C (rw) register accessor: GPIO\\_E/F/G/H Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio02c`] module"]
#[doc(alias = "GPIO02C")]
pub type Gpio02c = crate::Reg<gpio02c::Gpio02cSpec>;
#[doc = "GPIO\\_E/F/G/H Interrupt Sensitivity Type 0 Register"]
pub mod gpio02c;
#[doc = "GPIO030 (rw) register accessor: GPIO\\_E/F/G/H Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio030`] module"]
#[doc(alias = "GPIO030")]
pub type Gpio030 = crate::Reg<gpio030::Gpio030Spec>;
#[doc = "GPIO\\_E/F/G/H Interrupt Sensitivity Type 1 Register"]
pub mod gpio030;
#[doc = "GPIO034 (rw) register accessor: GPIO\\_E/F/G/H Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio034`] module"]
#[doc(alias = "GPIO034")]
pub type Gpio034 = crate::Reg<gpio034::Gpio034Spec>;
#[doc = "GPIO\\_E/F/G/H Interrupt Sensitivity Type 2 Register"]
pub mod gpio034;
#[doc = "GPIO038 (rw) register accessor: GPIO\\_E/F/G/H Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio038`] module"]
#[doc(alias = "GPIO038")]
pub type Gpio038 = crate::Reg<gpio038::Gpio038Spec>;
#[doc = "GPIO\\_E/F/G/H Interrupt Status Register"]
pub mod gpio038;
#[doc = "GPIO03C (rw) register accessor: GPIO\\_E/F/G/H Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio03c`] module"]
#[doc(alias = "GPIO03C")]
pub type Gpio03c = crate::Reg<gpio03c::Gpio03cSpec>;
#[doc = "GPIO\\_E/F/G/H Reset Tolerant Register"]
pub mod gpio03c;
#[doc = "GPIO040 (rw) register accessor: GPIO\\_A/B/C/D Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio040`] module"]
#[doc(alias = "GPIO040")]
pub type Gpio040 = crate::Reg<gpio040::Gpio040Spec>;
#[doc = "GPIO\\_A/B/C/D Debounce Setting Register \\#1"]
pub mod gpio040;
#[doc = "GPIO044 (rw) register accessor: GPIO\\_A/B/C/D Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio044`] module"]
#[doc(alias = "GPIO044")]
pub type Gpio044 = crate::Reg<gpio044::Gpio044Spec>;
#[doc = "GPIO\\_A/B/C/D Debounce Setting Register \\#2"]
pub mod gpio044;
#[doc = "GPIO048 (rw) register accessor: GPIO\\_E/F/G/H Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio048`] module"]
#[doc(alias = "GPIO048")]
pub type Gpio048 = crate::Reg<gpio048::Gpio048Spec>;
#[doc = "GPIO\\_E/F/G/H Debounce Setting Register \\#1"]
pub mod gpio048;
#[doc = "GPIO04C (rw) register accessor: GPIO\\_E/F/G/H Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio04c`] module"]
#[doc(alias = "GPIO04C")]
pub type Gpio04c = crate::Reg<gpio04c::Gpio04cSpec>;
#[doc = "GPIO\\_E/F/G/H Debounce Setting Register \\#2"]
pub mod gpio04c;
#[doc = "GPIO050 (rw) register accessor: Debounce Timer Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio050`] module"]
#[doc(alias = "GPIO050")]
pub type Gpio050 = crate::Reg<gpio050::Gpio050Spec>;
#[doc = "Debounce Timer Setting Register \\#1"]
pub mod gpio050;
#[doc = "GPIO054 (rw) register accessor: Debounce Timer Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio054`] module"]
#[doc(alias = "GPIO054")]
pub type Gpio054 = crate::Reg<gpio054::Gpio054Spec>;
#[doc = "Debounce Timer Setting Register \\#2"]
pub mod gpio054;
#[doc = "GPIO058 (rw) register accessor: Debounce Timer Setting Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio058`] module"]
#[doc(alias = "GPIO058")]
pub type Gpio058 = crate::Reg<gpio058::Gpio058Spec>;
#[doc = "Debounce Timer Setting Register \\#3"]
pub mod gpio058;
#[doc = "GPIO060 (rw) register accessor: GPIO\\_A/B/C/D Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio060`] module"]
#[doc(alias = "GPIO060")]
pub type Gpio060 = crate::Reg<gpio060::Gpio060Spec>;
#[doc = "GPIO\\_A/B/C/D Command Source 0"]
pub mod gpio060;
#[doc = "GPIO064 (rw) register accessor: GPIO\\_A/B/C/D Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio064`] module"]
#[doc(alias = "GPIO064")]
pub type Gpio064 = crate::Reg<gpio064::Gpio064Spec>;
#[doc = "GPIO\\_A/B/C/D Command Source 1"]
pub mod gpio064;
#[doc = "GPIO068 (rw) register accessor: GPIO\\_E/F/G/H Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio068`] module"]
#[doc(alias = "GPIO068")]
pub type Gpio068 = crate::Reg<gpio068::Gpio068Spec>;
#[doc = "GPIO\\_E/F/G/H Command Source 0"]
pub mod gpio068;
#[doc = "GPIO06C (rw) register accessor: GPIO\\_E/F/G/H Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio06c`] module"]
#[doc(alias = "GPIO06C")]
pub type Gpio06c = crate::Reg<gpio06c::Gpio06cSpec>;
#[doc = "GPIO\\_E/F/G/H Command Source 1"]
pub mod gpio06c;
#[doc = "GPIO070 (rw) register accessor: GPIO\\_I/J/K/L Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio070`] module"]
#[doc(alias = "GPIO070")]
pub type Gpio070 = crate::Reg<gpio070::Gpio070Spec>;
#[doc = "GPIO\\_I/J/K/L Data Value Register"]
pub mod gpio070;
#[doc = "GPIO074 (rw) register accessor: GPIO\\_I/J/K/L Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio074`] module"]
#[doc(alias = "GPIO074")]
pub type Gpio074 = crate::Reg<gpio074::Gpio074Spec>;
#[doc = "GPIO\\_I/J/K/L Direction Register"]
pub mod gpio074;
#[doc = "GPIO078 (rw) register accessor: GPIO\\_M/N/O/P Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio078`] module"]
#[doc(alias = "GPIO078")]
pub type Gpio078 = crate::Reg<gpio078::Gpio078Spec>;
#[doc = "GPIO\\_M/N/O/P Data Value Register"]
pub mod gpio078;
#[doc = "GPIO07C (rw) register accessor: GPIO\\_M/N/O/P Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio07c`] module"]
#[doc(alias = "GPIO07C")]
pub type Gpio07c = crate::Reg<gpio07c::Gpio07cSpec>;
#[doc = "GPIO\\_M/N/O/P Direction Register"]
pub mod gpio07c;
#[doc = "GPIO080 (rw) register accessor: GPIO\\_Q/R/S/T Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio080`] module"]
#[doc(alias = "GPIO080")]
pub type Gpio080 = crate::Reg<gpio080::Gpio080Spec>;
#[doc = "GPIO\\_Q/R/S/T Data Value Register"]
pub mod gpio080;
#[doc = "GPIO084 (rw) register accessor: GPIO\\_Q/R/S/T Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio084`] module"]
#[doc(alias = "GPIO084")]
pub type Gpio084 = crate::Reg<gpio084::Gpio084Spec>;
#[doc = "GPIO\\_Q/R/S/T Direction Register"]
pub mod gpio084;
#[doc = "GPIO088 (rw) register accessor: GPIO\\_U Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio088`] module"]
#[doc(alias = "GPIO088")]
pub type Gpio088 = crate::Reg<gpio088::Gpio088Spec>;
#[doc = "GPIO\\_U Data Value Register"]
pub mod gpio088;
#[doc = "GPIO08C (rw) register accessor: GPIO\\_U Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio08c`] module"]
#[doc(alias = "GPIO08C")]
pub type Gpio08c = crate::Reg<gpio08c::Gpio08cSpec>;
#[doc = "GPIO\\_U Direction Register"]
pub mod gpio08c;
#[doc = "GPIO090 (rw) register accessor: GPIO\\_I/J/K/L Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio090`] module"]
#[doc(alias = "GPIO090")]
pub type Gpio090 = crate::Reg<gpio090::Gpio090Spec>;
#[doc = "GPIO\\_I/J/K/L Command Source 0"]
pub mod gpio090;
#[doc = "GPIO094 (rw) register accessor: GPIO\\_I/J/K/L Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio094`] module"]
#[doc(alias = "GPIO094")]
pub type Gpio094 = crate::Reg<gpio094::Gpio094Spec>;
#[doc = "GPIO\\_I/J/K/L Command Source 1"]
pub mod gpio094;
#[doc = "GPIO098 (rw) register accessor: GPIO\\_I/J/K/L Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio098`] module"]
#[doc(alias = "GPIO098")]
pub type Gpio098 = crate::Reg<gpio098::Gpio098Spec>;
#[doc = "GPIO\\_I/J/K/L Interrupt Enable Register"]
pub mod gpio098;
#[doc = "GPIO09C (rw) register accessor: GPIO\\_I/J/K/L Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio09c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio09c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio09c`] module"]
#[doc(alias = "GPIO09C")]
pub type Gpio09c = crate::Reg<gpio09c::Gpio09cSpec>;
#[doc = "GPIO\\_I/J/K/L Interrupt Sensitivity Type 0 Register"]
pub mod gpio09c;
#[doc = "GPIO0A0 (rw) register accessor: GPIO\\_I/J/K/L Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0a0`] module"]
#[doc(alias = "GPIO0A0")]
pub type Gpio0a0 = crate::Reg<gpio0a0::Gpio0a0Spec>;
#[doc = "GPIO\\_I/J/K/L Interrupt Sensitivity Type 1 Register"]
pub mod gpio0a0;
#[doc = "GPIO0A4 (rw) register accessor: GPIO\\_I/J/K/L Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0a4`] module"]
#[doc(alias = "GPIO0A4")]
pub type Gpio0a4 = crate::Reg<gpio0a4::Gpio0a4Spec>;
#[doc = "GPIO\\_I/J/K/L Interrupt Sensitivity Type 2 Register"]
pub mod gpio0a4;
#[doc = "GPIO0A8 (rw) register accessor: GPIO\\_I/J/K/L Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0a8`] module"]
#[doc(alias = "GPIO0A8")]
pub type Gpio0a8 = crate::Reg<gpio0a8::Gpio0a8Spec>;
#[doc = "GPIO\\_I/J/K/L Interrupt Status Register"]
pub mod gpio0a8;
#[doc = "GPIO0AC (rw) register accessor: GPIO\\_I/J/K/L Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0ac`] module"]
#[doc(alias = "GPIO0AC")]
pub type Gpio0ac = crate::Reg<gpio0ac::Gpio0acSpec>;
#[doc = "GPIO\\_I/J/K/L Reset Tolerant Register"]
pub mod gpio0ac;
#[doc = "GPIO0B0 (rw) register accessor: GPIO\\_I/J/K/L Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0b0`] module"]
#[doc(alias = "GPIO0B0")]
pub type Gpio0b0 = crate::Reg<gpio0b0::Gpio0b0Spec>;
#[doc = "GPIO\\_I/J/K/L Debounce Setting Register \\#1"]
pub mod gpio0b0;
#[doc = "GPIO0B4 (rw) register accessor: GPIO\\_I/J/K/L Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0b4`] module"]
#[doc(alias = "GPIO0B4")]
pub type Gpio0b4 = crate::Reg<gpio0b4::Gpio0b4Spec>;
#[doc = "GPIO\\_I/J/K/L Debounce Setting Register \\#2"]
pub mod gpio0b4;
#[doc = "GPIO0B8 (rw) register accessor: GPIO\\_I/J/K/L Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0b8`] module"]
#[doc(alias = "GPIO0B8")]
pub type Gpio0b8 = crate::Reg<gpio0b8::Gpio0b8Spec>;
#[doc = "GPIO\\_I/J/K/L Input Mask Register"]
pub mod gpio0b8;
#[doc = "GPIO0C0 (rw) register accessor: GPIO\\_A/B/C/D Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0c0`] module"]
#[doc(alias = "GPIO0C0")]
pub type Gpio0c0 = crate::Reg<gpio0c0::Gpio0c0Spec>;
#[doc = "GPIO\\_A/B/C/D Data Read Register"]
pub mod gpio0c0;
#[doc = "GPIO0C4 (rw) register accessor: GPIO\\_E/F/G/H Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0c4`] module"]
#[doc(alias = "GPIO0C4")]
pub type Gpio0c4 = crate::Reg<gpio0c4::Gpio0c4Spec>;
#[doc = "GPIO\\_E/F/G/H Data Read Register"]
pub mod gpio0c4;
#[doc = "GPIO0C8 (rw) register accessor: GPIO\\_I/J/K/L Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0c8`] module"]
#[doc(alias = "GPIO0C8")]
pub type Gpio0c8 = crate::Reg<gpio0c8::Gpio0c8Spec>;
#[doc = "GPIO\\_I/J/K/L Data Read Register"]
pub mod gpio0c8;
#[doc = "GPIO0CC (rw) register accessor: GPIO\\_M/N/O/P Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0cc`] module"]
#[doc(alias = "GPIO0CC")]
pub type Gpio0cc = crate::Reg<gpio0cc::Gpio0ccSpec>;
#[doc = "GPIO\\_M/N/O/P Data Read Register"]
pub mod gpio0cc;
#[doc = "GPIO0D0 (rw) register accessor: GPIO\\_Q/R/S/T Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0d0`] module"]
#[doc(alias = "GPIO0D0")]
pub type Gpio0d0 = crate::Reg<gpio0d0::Gpio0d0Spec>;
#[doc = "GPIO\\_Q/R/S/T Data Read Register"]
pub mod gpio0d0;
#[doc = "GPIO0D4 (rw) register accessor: GPIO\\_U Data Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0d4`] module"]
#[doc(alias = "GPIO0D4")]
pub type Gpio0d4 = crate::Reg<gpio0d4::Gpio0d4Spec>;
#[doc = "GPIO\\_U Data Read Register"]
pub mod gpio0d4;
#[doc = "GPIO0E0 (rw) register accessor: GPIO\\_M/N/O/P Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0e0`] module"]
#[doc(alias = "GPIO0E0")]
pub type Gpio0e0 = crate::Reg<gpio0e0::Gpio0e0Spec>;
#[doc = "GPIO\\_M/N/O/P Command Source 0"]
pub mod gpio0e0;
#[doc = "GPIO0E4 (rw) register accessor: GPIO\\_M/N/O/P Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0e4`] module"]
#[doc(alias = "GPIO0E4")]
pub type Gpio0e4 = crate::Reg<gpio0e4::Gpio0e4Spec>;
#[doc = "GPIO\\_M/N/O/P Command Source 1"]
pub mod gpio0e4;
#[doc = "GPIO0E8 (rw) register accessor: GPIO\\_M/N/O/P Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0e8`] module"]
#[doc(alias = "GPIO0E8")]
pub type Gpio0e8 = crate::Reg<gpio0e8::Gpio0e8Spec>;
#[doc = "GPIO\\_M/N/O/P Interrupt Enable Register"]
pub mod gpio0e8;
#[doc = "GPIO0EC (rw) register accessor: GPIO\\_M/N/O/P Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0ec`] module"]
#[doc(alias = "GPIO0EC")]
pub type Gpio0ec = crate::Reg<gpio0ec::Gpio0ecSpec>;
#[doc = "GPIO\\_M/N/O/P Interrupt Sensitivity Type 0 Register"]
pub mod gpio0ec;
#[doc = "GPIO0F0 (rw) register accessor: GPIO\\_M/N/O/P Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0f0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0f0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0f0`] module"]
#[doc(alias = "GPIO0F0")]
pub type Gpio0f0 = crate::Reg<gpio0f0::Gpio0f0Spec>;
#[doc = "GPIO\\_M/N/O/P Interrupt Sensitivity Type 1 Register"]
pub mod gpio0f0;
#[doc = "GPIO0F4 (rw) register accessor: GPIO\\_M/N/O/P Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0f4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0f4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0f4`] module"]
#[doc(alias = "GPIO0F4")]
pub type Gpio0f4 = crate::Reg<gpio0f4::Gpio0f4Spec>;
#[doc = "GPIO\\_M/N/O/P Interrupt Sensitivity Type 2 Register"]
pub mod gpio0f4;
#[doc = "GPIO0F8 (rw) register accessor: GPIO\\_M/N/O/P Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0f8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0f8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0f8`] module"]
#[doc(alias = "GPIO0F8")]
pub type Gpio0f8 = crate::Reg<gpio0f8::Gpio0f8Spec>;
#[doc = "GPIO\\_M/N/O/P Interrupt Status Register"]
pub mod gpio0f8;
#[doc = "GPIO0FC (rw) register accessor: GPIO\\_M/N/O/P Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0fc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0fc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio0fc`] module"]
#[doc(alias = "GPIO0FC")]
pub type Gpio0fc = crate::Reg<gpio0fc::Gpio0fcSpec>;
#[doc = "GPIO\\_M/N/O/P Reset Tolerant Register"]
pub mod gpio0fc;
#[doc = "GPIO100 (rw) register accessor: GPIO\\_M/N/O/P Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio100`] module"]
#[doc(alias = "GPIO100")]
pub type Gpio100 = crate::Reg<gpio100::Gpio100Spec>;
#[doc = "GPIO\\_M/N/O/P Debounce Setting Register \\#1"]
pub mod gpio100;
#[doc = "GPIO104 (rw) register accessor: GPIO\\_M/N/O/P Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio104`] module"]
#[doc(alias = "GPIO104")]
pub type Gpio104 = crate::Reg<gpio104::Gpio104Spec>;
#[doc = "GPIO\\_M/N/O/P Debounce Setting Register \\#2"]
pub mod gpio104;
#[doc = "GPIO108 (rw) register accessor: GPIO\\_M/N/O/P Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio108`] module"]
#[doc(alias = "GPIO108")]
pub type Gpio108 = crate::Reg<gpio108::Gpio108Spec>;
#[doc = "GPIO\\_M/N/O/P Input Mask Register"]
pub mod gpio108;
#[doc = "GPIO110 (rw) register accessor: GPIO\\_Q/R/S/T Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio110`] module"]
#[doc(alias = "GPIO110")]
pub type Gpio110 = crate::Reg<gpio110::Gpio110Spec>;
#[doc = "GPIO\\_Q/R/S/T Command Source 0"]
pub mod gpio110;
#[doc = "GPIO114 (rw) register accessor: GPIO\\_Q/R/S/T Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio114`] module"]
#[doc(alias = "GPIO114")]
pub type Gpio114 = crate::Reg<gpio114::Gpio114Spec>;
#[doc = "GPIO\\_Q/R/S/T Command Source 1"]
pub mod gpio114;
#[doc = "GPIO118 (rw) register accessor: GPIO\\_Q/R/S/T Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio118`] module"]
#[doc(alias = "GPIO118")]
pub type Gpio118 = crate::Reg<gpio118::Gpio118Spec>;
#[doc = "GPIO\\_Q/R/S/T Interrupt Enable Register"]
pub mod gpio118;
#[doc = "GPIO11C (rw) register accessor: GPIO\\_Q/R/S/T Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio11c`] module"]
#[doc(alias = "GPIO11C")]
pub type Gpio11c = crate::Reg<gpio11c::Gpio11cSpec>;
#[doc = "GPIO\\_Q/R/S/T Interrupt Sensitivity Type 0 Register"]
pub mod gpio11c;
#[doc = "GPIO120 (rw) register accessor: GPIO\\_Q/R/S/T Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio120`] module"]
#[doc(alias = "GPIO120")]
pub type Gpio120 = crate::Reg<gpio120::Gpio120Spec>;
#[doc = "GPIO\\_Q/R/S/T Interrupt Sensitivity Type 1 Register"]
pub mod gpio120;
#[doc = "GPIO124 (rw) register accessor: GPIO\\_Q/R/S/T Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio124`] module"]
#[doc(alias = "GPIO124")]
pub type Gpio124 = crate::Reg<gpio124::Gpio124Spec>;
#[doc = "GPIO\\_Q/R/S/T Interrupt Sensitivity Type 2 Register"]
pub mod gpio124;
#[doc = "GPIO128 (rw) register accessor: GPIO\\_Q/R/S/T Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio128`] module"]
#[doc(alias = "GPIO128")]
pub type Gpio128 = crate::Reg<gpio128::Gpio128Spec>;
#[doc = "GPIO\\_Q/R/S/T Interrupt Status Register"]
pub mod gpio128;
#[doc = "GPIO12C (rw) register accessor: GPIO\\_Q/R/S/T Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio12c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio12c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio12c`] module"]
#[doc(alias = "GPIO12C")]
pub type Gpio12c = crate::Reg<gpio12c::Gpio12cSpec>;
#[doc = "GPIO\\_Q/R/S/T Reset Tolerant Register"]
pub mod gpio12c;
#[doc = "GPIO130 (rw) register accessor: GPIO\\_Q/R/S/T Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio130`] module"]
#[doc(alias = "GPIO130")]
pub type Gpio130 = crate::Reg<gpio130::Gpio130Spec>;
#[doc = "GPIO\\_Q/R/S/T Debounce Setting Register \\#1"]
pub mod gpio130;
#[doc = "GPIO134 (rw) register accessor: GPIO\\_Q/R/S/T Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio134`] module"]
#[doc(alias = "GPIO134")]
pub type Gpio134 = crate::Reg<gpio134::Gpio134Spec>;
#[doc = "GPIO\\_Q/R/S/T Debounce Setting Register \\#2"]
pub mod gpio134;
#[doc = "GPIO138 (rw) register accessor: GPIO\\_Q/R/S/T Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio138`] module"]
#[doc(alias = "GPIO138")]
pub type Gpio138 = crate::Reg<gpio138::Gpio138Spec>;
#[doc = "GPIO\\_Q/R/S/T Input Mask Register"]
pub mod gpio138;
#[doc = "GPIO140 (rw) register accessor: GPIO\\_U Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio140`] module"]
#[doc(alias = "GPIO140")]
pub type Gpio140 = crate::Reg<gpio140::Gpio140Spec>;
#[doc = "GPIO\\_U Command Source 0"]
pub mod gpio140;
#[doc = "GPIO144 (rw) register accessor: GPIO\\_U Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio144`] module"]
#[doc(alias = "GPIO144")]
pub type Gpio144 = crate::Reg<gpio144::Gpio144Spec>;
#[doc = "GPIO\\_U Command Source 1"]
pub mod gpio144;
#[doc = "GPIO148 (rw) register accessor: GPIO\\_U Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio148`] module"]
#[doc(alias = "GPIO148")]
pub type Gpio148 = crate::Reg<gpio148::Gpio148Spec>;
#[doc = "GPIO\\_U Interrupt Enable Register"]
pub mod gpio148;
#[doc = "GPIO14C (rw) register accessor: GPIO\\_U Interrupt Sensitivity Type 0 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio14c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio14c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio14c`] module"]
#[doc(alias = "GPIO14C")]
pub type Gpio14c = crate::Reg<gpio14c::Gpio14cSpec>;
#[doc = "GPIO\\_U Interrupt Sensitivity Type 0 Register"]
pub mod gpio14c;
#[doc = "GPIO150 (rw) register accessor: GPIO\\_U Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio150`] module"]
#[doc(alias = "GPIO150")]
pub type Gpio150 = crate::Reg<gpio150::Gpio150Spec>;
#[doc = "GPIO\\_U Interrupt Sensitivity Type 1 Register"]
pub mod gpio150;
#[doc = "GPIO154 (rw) register accessor: GPIO\\_U Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio154`] module"]
#[doc(alias = "GPIO154")]
pub type Gpio154 = crate::Reg<gpio154::Gpio154Spec>;
#[doc = "GPIO\\_U Interrupt Sensitivity Type 2 Register"]
pub mod gpio154;
#[doc = "GPIO158 (rw) register accessor: GPIO\\_U Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio158`] module"]
#[doc(alias = "GPIO158")]
pub type Gpio158 = crate::Reg<gpio158::Gpio158Spec>;
#[doc = "GPIO\\_U Interrupt Status Register"]
pub mod gpio158;
#[doc = "GPIO15C (rw) register accessor: GPIO\\_U Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio15c`] module"]
#[doc(alias = "GPIO15C")]
pub type Gpio15c = crate::Reg<gpio15c::Gpio15cSpec>;
#[doc = "GPIO\\_U Reset Tolerant Register"]
pub mod gpio15c;
#[doc = "GPIO160 (rw) register accessor: GPIO\\_U Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio160`] module"]
#[doc(alias = "GPIO160")]
pub type Gpio160 = crate::Reg<gpio160::Gpio160Spec>;
#[doc = "GPIO\\_U Debounce Setting Register \\#1"]
pub mod gpio160;
#[doc = "GPIO164 (rw) register accessor: GPIO\\_U Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio164`] module"]
#[doc(alias = "GPIO164")]
pub type Gpio164 = crate::Reg<gpio164::Gpio164Spec>;
#[doc = "GPIO\\_U Debounce Setting Register \\#2"]
pub mod gpio164;
#[doc = "GPIO168 (rw) register accessor: GPIO\\_U Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio168`] module"]
#[doc(alias = "GPIO168")]
pub type Gpio168 = crate::Reg<gpio168::Gpio168Spec>;
#[doc = "GPIO\\_U Input Mask Register"]
pub mod gpio168;
#[doc = "GPIO1D0 (rw) register accessor: GPIO\\_A/B/C/D Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d0`] module"]
#[doc(alias = "GPIO1D0")]
pub type Gpio1d0 = crate::Reg<gpio1d0::Gpio1d0Spec>;
#[doc = "GPIO\\_A/B/C/D Input Mask Register"]
pub mod gpio1d0;
#[doc = "GPIO1D4 (rw) register accessor: GPIO\\_E/F/G/H Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio1d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio1d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio1d4`] module"]
#[doc(alias = "GPIO1D4")]
pub type Gpio1d4 = crate::Reg<gpio1d4::Gpio1d4Spec>;
#[doc = "GPIO\\_E/F/G/H Input Mask Register"]
pub mod gpio1d4;
#[doc = "GPIO2AC (rw) register accessor: GPIO Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio2ac`] module"]
#[doc(alias = "GPIO2AC")]
pub type Gpio2ac = crate::Reg<gpio2ac::Gpio2acSpec>;
#[doc = "GPIO Index Register"]
pub mod gpio2ac;
#[doc = "GPIO500 (rw) register accessor: Serial GPIO\\_A/B/C/D 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio500::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio500::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio500`] module"]
#[doc(alias = "GPIO500")]
pub type Gpio500 = crate::Reg<gpio500::Gpio500Spec>;
#[doc = "Serial GPIO\\_A/B/C/D 1 Data Value Register"]
pub mod gpio500;
