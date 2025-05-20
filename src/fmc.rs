#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fmc000: Fmc000,
    fmc004: Fmc004,
    fmc008: Fmc008,
    fmc00c: Fmc00c,
    fmc010: Fmc010,
    fmc014: Fmc014,
    _reserved6: [u8; 0x18],
    fmc030: Fmc030,
    fmc034: Fmc034,
    _reserved8: [u8; 0x18],
    fmc050: Fmc050,
    fmc054: Fmc054,
    _reserved10: [u8; 0x08],
    fmc060: Fmc060,
    fmc064: Fmc064,
    fmc068: Fmc068,
    fmc06c: Fmc06c,
    _reserved14: [u8; 0x0c],
    fmc07c: Fmc07c,
    fmc080: Fmc080,
    fmc084: Fmc084,
    fmc088: Fmc088,
    fmc08c: Fmc08c,
    fmc090: Fmc090,
    fmc094: Fmc094,
    fmc098: Fmc098,
    _reserved22: [u8; 0x04],
    fmc0a0: Fmc0a0,
    fmc0a4: Fmc0a4,
    fmc0a8: Fmc0a8,
    fmc0ac: Fmc0ac,
    fmc0b0: Fmc0b0,
    fmc0b4: Fmc0b4,
    fmc0b8: Fmc0b8,
    fmc0bc: Fmc0bc,
    fmc0c0: Fmc0c0,
    fmc0c4: Fmc0c4,
    fmc0c8: Fmc0c8,
    fmc0cc: Fmc0cc,
    _reserved34: [u8; 0x30],
    fmc100: Fmc100,
    fmc104: Fmc104,
    fmc108: Fmc108,
    fmc10c: Fmc10c,
    fmc110: Fmc110,
    fmc114: Fmc114,
    fmc118: Fmc118,
    fmc11c: Fmc11c,
    fmcfqcd: [Fmcfqcd; 12],
    fmc150: Fmc150,
    fmc154: Fmc154,
    fmc158: Fmc158,
    fmc15c: Fmc15c,
    fmc160: Fmc160,
    fmc164: Fmc164,
    fmc168: Fmc168,
    fmc16c: Fmc16c,
    fmc170: Fmc170,
    fmc174: Fmc174,
    fmc178: Fmc178,
    fmc17c: Fmc17c,
    _reserved55: [u8; 0x80],
    fmcbuf: [Fmcbuf; 64],
}
impl RegisterBlock {
    #[doc = "0x00 - CE Type Setting Register"]
    #[inline(always)]
    pub const fn fmc000(&self) -> &Fmc000 {
        &self.fmc000
    }
    #[doc = "0x04 - CE Control Register"]
    #[inline(always)]
    pub const fn fmc004(&self) -> &Fmc004 {
        &self.fmc004
    }
    #[doc = "0x08 - Interrupt Control and Status Register"]
    #[inline(always)]
    pub const fn fmc008(&self) -> &Fmc008 {
        &self.fmc008
    }
    #[doc = "0x0c - Command Control Register"]
    #[inline(always)]
    pub const fn fmc00c(&self) -> &Fmc00c {
        &self.fmc00c
    }
    #[doc = "0x10 - SPI CE0 Control Register"]
    #[inline(always)]
    pub const fn fmc010(&self) -> &Fmc010 {
        &self.fmc010
    }
    #[doc = "0x14 - SPI CE1 Control Register"]
    #[inline(always)]
    pub const fn fmc014(&self) -> &Fmc014 {
        &self.fmc014
    }
    #[doc = "0x30 - CE0 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn fmc030(&self) -> &Fmc030 {
        &self.fmc030
    }
    #[doc = "0x34 - CE1 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn fmc034(&self) -> &Fmc034 {
        &self.fmc034
    }
    #[doc = "0x50 - Auto Soft-Reset Command Control"]
    #[inline(always)]
    pub const fn fmc050(&self) -> &Fmc050 {
        &self.fmc050
    }
    #[doc = "0x54 - SPI Dummy Cycle Data Register"]
    #[inline(always)]
    pub const fn fmc054(&self) -> &Fmc054 {
        &self.fmc054
    }
    #[doc = "0x60 - FMC\\_WDT1 Control/Status Register for Address Mode Detection"]
    #[inline(always)]
    pub const fn fmc060(&self) -> &Fmc060 {
        &self.fmc060
    }
    #[doc = "0x64 - FMC\\_WDT2 Control/Status Register for Alternate Boot"]
    #[inline(always)]
    pub const fn fmc064(&self) -> &Fmc064 {
        &self.fmc064
    }
    #[doc = "0x68 - FMC\\_WDT2 Timer Reload Value Register"]
    #[inline(always)]
    pub const fn fmc068(&self) -> &Fmc068 {
        &self.fmc068
    }
    #[doc = "0x6c - FMC\\_WDT2 Timer Restart Register"]
    #[inline(always)]
    pub const fn fmc06c(&self) -> &Fmc06c {
        &self.fmc06c
    }
    #[doc = "0x7c - DMA Buffer Mode Length Register"]
    #[inline(always)]
    pub const fn fmc07c(&self) -> &Fmc07c {
        &self.fmc07c
    }
    #[doc = "0x80 - DMA Control/Status Register"]
    #[inline(always)]
    pub const fn fmc080(&self) -> &Fmc080 {
        &self.fmc080
    }
    #[doc = "0x84 - DMA Flash Side Address"]
    #[inline(always)]
    pub const fn fmc084(&self) -> &Fmc084 {
        &self.fmc084
    }
    #[doc = "0x88 - DMA DRAM/SRAM Side Address"]
    #[inline(always)]
    pub const fn fmc088(&self) -> &Fmc088 {
        &self.fmc088
    }
    #[doc = "0x8c - DMA Length Register"]
    #[inline(always)]
    pub const fn fmc08c(&self) -> &Fmc08c {
        &self.fmc08c
    }
    #[doc = "0x90 - CheckSum Calculation Result"]
    #[inline(always)]
    pub const fn fmc090(&self) -> &Fmc090 {
        &self.fmc090
    }
    #[doc = "0x94 - CE0 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn fmc094(&self) -> &Fmc094 {
        &self.fmc094
    }
    #[doc = "0x98 - CE1 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn fmc098(&self) -> &Fmc098 {
        &self.fmc098
    }
    #[doc = "0xa0 - Command Filter Control Register"]
    #[inline(always)]
    pub const fn fmc0a0(&self) -> &Fmc0a0 {
        &self.fmc0a0
    }
    #[doc = "0xa4 - Write Address Filter Control Register"]
    #[inline(always)]
    pub const fn fmc0a4(&self) -> &Fmc0a4 {
        &self.fmc0a4
    }
    #[doc = "0xa8 - Register Lock Control Register (SRST\\#)"]
    #[inline(always)]
    pub const fn fmc0a8(&self) -> &Fmc0a8 {
        &self.fmc0a8
    }
    #[doc = "0xac - Register Lock Control Register (Watchdog)"]
    #[inline(always)]
    pub const fn fmc0ac(&self) -> &Fmc0ac {
        &self.fmc0ac
    }
    #[doc = "0xb0 - Write Address Filter Register \\#1"]
    #[inline(always)]
    pub const fn fmc0b0(&self) -> &Fmc0b0 {
        &self.fmc0b0
    }
    #[doc = "0xb4 - Write Address Filter Register \\#2"]
    #[inline(always)]
    pub const fn fmc0b4(&self) -> &Fmc0b4 {
        &self.fmc0b4
    }
    #[doc = "0xb8 - Write Address Filter Register \\#3"]
    #[inline(always)]
    pub const fn fmc0b8(&self) -> &Fmc0b8 {
        &self.fmc0b8
    }
    #[doc = "0xbc - Write Address Filter Register \\#4"]
    #[inline(always)]
    pub const fn fmc0bc(&self) -> &Fmc0bc {
        &self.fmc0bc
    }
    #[doc = "0xc0 - Write Address Filter Register \\#5"]
    #[inline(always)]
    pub const fn fmc0c0(&self) -> &Fmc0c0 {
        &self.fmc0c0
    }
    #[doc = "0xc4 - Write Address Filter Register \\#6"]
    #[inline(always)]
    pub const fn fmc0c4(&self) -> &Fmc0c4 {
        &self.fmc0c4
    }
    #[doc = "0xc8 - Write Address Filter Register \\#7"]
    #[inline(always)]
    pub const fn fmc0c8(&self) -> &Fmc0c8 {
        &self.fmc0c8
    }
    #[doc = "0xcc - Write Address Filter Register \\#8"]
    #[inline(always)]
    pub const fn fmc0cc(&self) -> &Fmc0cc {
        &self.fmc0cc
    }
    #[doc = "0x100 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc100(&self) -> &Fmc100 {
        &self.fmc100
    }
    #[doc = "0x104 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc104(&self) -> &Fmc104 {
        &self.fmc104
    }
    #[doc = "0x108 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc108(&self) -> &Fmc108 {
        &self.fmc108
    }
    #[doc = "0x10c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc10c(&self) -> &Fmc10c {
        &self.fmc10c
    }
    #[doc = "0x110 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc110(&self) -> &Fmc110 {
        &self.fmc110
    }
    #[doc = "0x114 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc114(&self) -> &Fmc114 {
        &self.fmc114
    }
    #[doc = "0x118 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc118(&self) -> &Fmc118 {
        &self.fmc118
    }
    #[doc = "0x11c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmc11c(&self) -> &Fmc11c {
        &self.fmc11c
    }
    #[doc = "0x120..0x150 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn fmcfqcd(&self, n: usize) -> &Fmcfqcd {
        &self.fmcfqcd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x150 - Fully Qualified Command"]
    #[inline(always)]
    pub fn fmcfqcd_iter(&self) -> impl Iterator<Item = &Fmcfqcd> {
        self.fmcfqcd.iter()
    }
    #[doc = "0x150 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc150(&self) -> &Fmc150 {
        &self.fmc150
    }
    #[doc = "0x154 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc154(&self) -> &Fmc154 {
        &self.fmc154
    }
    #[doc = "0x158 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc158(&self) -> &Fmc158 {
        &self.fmc158
    }
    #[doc = "0x15c - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc15c(&self) -> &Fmc15c {
        &self.fmc15c
    }
    #[doc = "0x160 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc160(&self) -> &Fmc160 {
        &self.fmc160
    }
    #[doc = "0x164 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc164(&self) -> &Fmc164 {
        &self.fmc164
    }
    #[doc = "0x168 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc168(&self) -> &Fmc168 {
        &self.fmc168
    }
    #[doc = "0x16c - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc16c(&self) -> &Fmc16c {
        &self.fmc16c
    }
    #[doc = "0x170 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc170(&self) -> &Fmc170 {
        &self.fmc170
    }
    #[doc = "0x174 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc174(&self) -> &Fmc174 {
        &self.fmc174
    }
    #[doc = "0x178 - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc178(&self) -> &Fmc178 {
        &self.fmc178
    }
    #[doc = "0x17c - Address Qualified Command"]
    #[inline(always)]
    pub const fn fmc17c(&self) -> &Fmc17c {
        &self.fmc17c
    }
    #[doc = "0x200..0x300 - DMA Buffer Data Register"]
    #[inline(always)]
    pub const fn fmcbuf(&self, n: usize) -> &Fmcbuf {
        &self.fmcbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x300 - DMA Buffer Data Register"]
    #[inline(always)]
    pub fn fmcbuf_iter(&self) -> impl Iterator<Item = &Fmcbuf> {
        self.fmcbuf.iter()
    }
}
#[doc = "FMC000 (rw) register accessor: CE Type Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc000`] module"]
#[doc(alias = "FMC000")]
pub type Fmc000 = crate::Reg<fmc000::Fmc000Spec>;
#[doc = "CE Type Setting Register"]
pub mod fmc000;
#[doc = "FMC004 (rw) register accessor: CE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc004`] module"]
#[doc(alias = "FMC004")]
pub type Fmc004 = crate::Reg<fmc004::Fmc004Spec>;
#[doc = "CE Control Register"]
pub mod fmc004;
#[doc = "FMC008 (rw) register accessor: Interrupt Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc008`] module"]
#[doc(alias = "FMC008")]
pub type Fmc008 = crate::Reg<fmc008::Fmc008Spec>;
#[doc = "Interrupt Control and Status Register"]
pub mod fmc008;
#[doc = "FMC00C (rw) register accessor: Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc00c`] module"]
#[doc(alias = "FMC00C")]
pub type Fmc00c = crate::Reg<fmc00c::Fmc00cSpec>;
#[doc = "Command Control Register"]
pub mod fmc00c;
#[doc = "FMC010 (rw) register accessor: SPI CE0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc010`] module"]
#[doc(alias = "FMC010")]
pub type Fmc010 = crate::Reg<fmc010::Fmc010Spec>;
#[doc = "SPI CE0 Control Register"]
pub mod fmc010;
#[doc = "FMC014 (rw) register accessor: SPI CE1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc014`] module"]
#[doc(alias = "FMC014")]
pub type Fmc014 = crate::Reg<fmc014::Fmc014Spec>;
#[doc = "SPI CE1 Control Register"]
pub mod fmc014;
#[doc = "FMC030 (rw) register accessor: CE0 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc030`] module"]
#[doc(alias = "FMC030")]
pub type Fmc030 = crate::Reg<fmc030::Fmc030Spec>;
#[doc = "CE0 Address Decoding Range Register"]
pub mod fmc030;
#[doc = "FMC034 (rw) register accessor: CE1 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc034`] module"]
#[doc(alias = "FMC034")]
pub type Fmc034 = crate::Reg<fmc034::Fmc034Spec>;
#[doc = "CE1 Address Decoding Range Register"]
pub mod fmc034;
#[doc = "FMC050 (rw) register accessor: Auto Soft-Reset Command Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc050`] module"]
#[doc(alias = "FMC050")]
pub type Fmc050 = crate::Reg<fmc050::Fmc050Spec>;
#[doc = "Auto Soft-Reset Command Control"]
pub mod fmc050;
#[doc = "FMC054 (rw) register accessor: SPI Dummy Cycle Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc054`] module"]
#[doc(alias = "FMC054")]
pub type Fmc054 = crate::Reg<fmc054::Fmc054Spec>;
#[doc = "SPI Dummy Cycle Data Register"]
pub mod fmc054;
#[doc = "FMC060 (rw) register accessor: FMC\\_WDT1 Control/Status Register for Address Mode Detection\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc060`] module"]
#[doc(alias = "FMC060")]
pub type Fmc060 = crate::Reg<fmc060::Fmc060Spec>;
#[doc = "FMC\\_WDT1 Control/Status Register for Address Mode Detection"]
pub mod fmc060;
#[doc = "FMC064 (rw) register accessor: FMC\\_WDT2 Control/Status Register for Alternate Boot\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc064::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc064::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc064`] module"]
#[doc(alias = "FMC064")]
pub type Fmc064 = crate::Reg<fmc064::Fmc064Spec>;
#[doc = "FMC\\_WDT2 Control/Status Register for Alternate Boot"]
pub mod fmc064;
#[doc = "FMC068 (rw) register accessor: FMC\\_WDT2 Timer Reload Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc068::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc068::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc068`] module"]
#[doc(alias = "FMC068")]
pub type Fmc068 = crate::Reg<fmc068::Fmc068Spec>;
#[doc = "FMC\\_WDT2 Timer Reload Value Register"]
pub mod fmc068;
#[doc = "FMC06C (rw) register accessor: FMC\\_WDT2 Timer Restart Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc06c`] module"]
#[doc(alias = "FMC06C")]
pub type Fmc06c = crate::Reg<fmc06c::Fmc06cSpec>;
#[doc = "FMC\\_WDT2 Timer Restart Register"]
pub mod fmc06c;
#[doc = "FMC07C (rw) register accessor: DMA Buffer Mode Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc07c`] module"]
#[doc(alias = "FMC07C")]
pub type Fmc07c = crate::Reg<fmc07c::Fmc07cSpec>;
#[doc = "DMA Buffer Mode Length Register"]
pub mod fmc07c;
#[doc = "FMC080 (rw) register accessor: DMA Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc080`] module"]
#[doc(alias = "FMC080")]
pub type Fmc080 = crate::Reg<fmc080::Fmc080Spec>;
#[doc = "DMA Control/Status Register"]
pub mod fmc080;
#[doc = "FMC084 (rw) register accessor: DMA Flash Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc084`] module"]
#[doc(alias = "FMC084")]
pub type Fmc084 = crate::Reg<fmc084::Fmc084Spec>;
#[doc = "DMA Flash Side Address"]
pub mod fmc084;
#[doc = "FMC088 (rw) register accessor: DMA DRAM/SRAM Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc088`] module"]
#[doc(alias = "FMC088")]
pub type Fmc088 = crate::Reg<fmc088::Fmc088Spec>;
#[doc = "DMA DRAM/SRAM Side Address"]
pub mod fmc088;
#[doc = "FMC08C (rw) register accessor: DMA Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc08c`] module"]
#[doc(alias = "FMC08C")]
pub type Fmc08c = crate::Reg<fmc08c::Fmc08cSpec>;
#[doc = "DMA Length Register"]
pub mod fmc08c;
#[doc = "FMC090 (rw) register accessor: CheckSum Calculation Result\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc090`] module"]
#[doc(alias = "FMC090")]
pub type Fmc090 = crate::Reg<fmc090::Fmc090Spec>;
#[doc = "CheckSum Calculation Result"]
pub mod fmc090;
#[doc = "FMC094 (rw) register accessor: CE0 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc094`] module"]
#[doc(alias = "FMC094")]
pub type Fmc094 = crate::Reg<fmc094::Fmc094Spec>;
#[doc = "CE0 SPI Flash Read Timing Compensation"]
pub mod fmc094;
#[doc = "FMC098 (rw) register accessor: CE1 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc098`] module"]
#[doc(alias = "FMC098")]
pub type Fmc098 = crate::Reg<fmc098::Fmc098Spec>;
#[doc = "CE1 SPI Flash Read Timing Compensation"]
pub mod fmc098;
#[doc = "FMC0A0 (rw) register accessor: Command Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0a0`] module"]
#[doc(alias = "FMC0A0")]
pub type Fmc0a0 = crate::Reg<fmc0a0::Fmc0a0Spec>;
#[doc = "Command Filter Control Register"]
pub mod fmc0a0;
#[doc = "FMC0A4 (rw) register accessor: Write Address Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0a4`] module"]
#[doc(alias = "FMC0A4")]
pub type Fmc0a4 = crate::Reg<fmc0a4::Fmc0a4Spec>;
#[doc = "Write Address Filter Control Register"]
pub mod fmc0a4;
#[doc = "FMC0A8 (rw) register accessor: Register Lock Control Register (SRST\\#)\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0a8`] module"]
#[doc(alias = "FMC0A8")]
pub type Fmc0a8 = crate::Reg<fmc0a8::Fmc0a8Spec>;
#[doc = "Register Lock Control Register (SRST\\#)"]
pub mod fmc0a8;
#[doc = "FMC0AC (rw) register accessor: Register Lock Control Register (Watchdog)\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0ac`] module"]
#[doc(alias = "FMC0AC")]
pub type Fmc0ac = crate::Reg<fmc0ac::Fmc0acSpec>;
#[doc = "Register Lock Control Register (Watchdog)"]
pub mod fmc0ac;
#[doc = "FMC0B0 (rw) register accessor: Write Address Filter Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0b0`] module"]
#[doc(alias = "FMC0B0")]
pub type Fmc0b0 = crate::Reg<fmc0b0::Fmc0b0Spec>;
#[doc = "Write Address Filter Register \\#1"]
pub mod fmc0b0;
#[doc = "FMC0B4 (rw) register accessor: Write Address Filter Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0b4`] module"]
#[doc(alias = "FMC0B4")]
pub type Fmc0b4 = crate::Reg<fmc0b4::Fmc0b4Spec>;
#[doc = "Write Address Filter Register \\#2"]
pub mod fmc0b4;
#[doc = "FMC0B8 (rw) register accessor: Write Address Filter Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0b8`] module"]
#[doc(alias = "FMC0B8")]
pub type Fmc0b8 = crate::Reg<fmc0b8::Fmc0b8Spec>;
#[doc = "Write Address Filter Register \\#3"]
pub mod fmc0b8;
#[doc = "FMC0BC (rw) register accessor: Write Address Filter Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0bc`] module"]
#[doc(alias = "FMC0BC")]
pub type Fmc0bc = crate::Reg<fmc0bc::Fmc0bcSpec>;
#[doc = "Write Address Filter Register \\#4"]
pub mod fmc0bc;
#[doc = "FMC0C0 (rw) register accessor: Write Address Filter Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0c0`] module"]
#[doc(alias = "FMC0C0")]
pub type Fmc0c0 = crate::Reg<fmc0c0::Fmc0c0Spec>;
#[doc = "Write Address Filter Register \\#5"]
pub mod fmc0c0;
#[doc = "FMC0C4 (rw) register accessor: Write Address Filter Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0c4`] module"]
#[doc(alias = "FMC0C4")]
pub type Fmc0c4 = crate::Reg<fmc0c4::Fmc0c4Spec>;
#[doc = "Write Address Filter Register \\#6"]
pub mod fmc0c4;
#[doc = "FMC0C8 (rw) register accessor: Write Address Filter Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0c8`] module"]
#[doc(alias = "FMC0C8")]
pub type Fmc0c8 = crate::Reg<fmc0c8::Fmc0c8Spec>;
#[doc = "Write Address Filter Register \\#7"]
pub mod fmc0c8;
#[doc = "FMC0CC (rw) register accessor: Write Address Filter Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc0cc`] module"]
#[doc(alias = "FMC0CC")]
pub type Fmc0cc = crate::Reg<fmc0cc::Fmc0ccSpec>;
#[doc = "Write Address Filter Register \\#8"]
pub mod fmc0cc;
#[doc = "FMC100 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc100`] module"]
#[doc(alias = "FMC100")]
pub type Fmc100 = crate::Reg<fmc100::Fmc100Spec>;
#[doc = "Fully Qualified Command"]
pub mod fmc100;
#[doc = "FMC104 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc104`] module"]
#[doc(alias = "FMC104")]
pub type Fmc104 = crate::Reg<fmc104::Fmc104Spec>;
#[doc = "Fully Qualified Command"]
pub mod fmc104;
#[doc = "FMC108 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc108`] module"]
#[doc(alias = "FMC108")]
pub type Fmc108 = crate::Reg<fmc108::Fmc108Spec>;
#[doc = "Fully Qualified Command"]
pub mod fmc108;
#[doc = "FMC10C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc10c`] module"]
#[doc(alias = "FMC10C")]
pub type Fmc10c = crate::Reg<fmc10c::Fmc10cSpec>;
#[doc = "Fully Qualified Command"]
pub mod fmc10c;
#[doc = "FMC110 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc110`] module"]
#[doc(alias = "FMC110")]
pub type Fmc110 = crate::Reg<fmc110::Fmc110Spec>;
#[doc = "Fully Qualified Command"]
pub mod fmc110;
#[doc = "FMC114 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc114`] module"]
#[doc(alias = "FMC114")]
pub type Fmc114 = crate::Reg<fmc114::Fmc114Spec>;
#[doc = "Fully Qualified Command"]
pub mod fmc114;
#[doc = "FMC118 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc118`] module"]
#[doc(alias = "FMC118")]
pub type Fmc118 = crate::Reg<fmc118::Fmc118Spec>;
#[doc = "Fully Qualified Command"]
pub mod fmc118;
#[doc = "FMC11C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc11c`] module"]
#[doc(alias = "FMC11C")]
pub type Fmc11c = crate::Reg<fmc11c::Fmc11cSpec>;
#[doc = "Fully Qualified Command"]
pub mod fmc11c;
#[doc = "FMCFQCD (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmcfqcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcfqcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmcfqcd`] module"]
#[doc(alias = "FMCFQCD")]
pub type Fmcfqcd = crate::Reg<fmcfqcd::FmcfqcdSpec>;
#[doc = "Fully Qualified Command"]
pub mod fmcfqcd;
#[doc = "FMC150 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc150`] module"]
#[doc(alias = "FMC150")]
pub type Fmc150 = crate::Reg<fmc150::Fmc150Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc150;
#[doc = "FMC154 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc154`] module"]
#[doc(alias = "FMC154")]
pub type Fmc154 = crate::Reg<fmc154::Fmc154Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc154;
#[doc = "FMC158 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc158`] module"]
#[doc(alias = "FMC158")]
pub type Fmc158 = crate::Reg<fmc158::Fmc158Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc158;
#[doc = "FMC15C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc15c`] module"]
#[doc(alias = "FMC15C")]
pub type Fmc15c = crate::Reg<fmc15c::Fmc15cSpec>;
#[doc = "Address Qualified Command"]
pub mod fmc15c;
#[doc = "FMC160 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc160`] module"]
#[doc(alias = "FMC160")]
pub type Fmc160 = crate::Reg<fmc160::Fmc160Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc160;
#[doc = "FMC164 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc164`] module"]
#[doc(alias = "FMC164")]
pub type Fmc164 = crate::Reg<fmc164::Fmc164Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc164;
#[doc = "FMC168 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc168`] module"]
#[doc(alias = "FMC168")]
pub type Fmc168 = crate::Reg<fmc168::Fmc168Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc168;
#[doc = "FMC16C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc16c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc16c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc16c`] module"]
#[doc(alias = "FMC16C")]
pub type Fmc16c = crate::Reg<fmc16c::Fmc16cSpec>;
#[doc = "Address Qualified Command"]
pub mod fmc16c;
#[doc = "FMC170 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc170`] module"]
#[doc(alias = "FMC170")]
pub type Fmc170 = crate::Reg<fmc170::Fmc170Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc170;
#[doc = "FMC174 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc174`] module"]
#[doc(alias = "FMC174")]
pub type Fmc174 = crate::Reg<fmc174::Fmc174Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc174;
#[doc = "FMC178 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc178`] module"]
#[doc(alias = "FMC178")]
pub type Fmc178 = crate::Reg<fmc178::Fmc178Spec>;
#[doc = "Address Qualified Command"]
pub mod fmc178;
#[doc = "FMC17C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmc17c`] module"]
#[doc(alias = "FMC17C")]
pub type Fmc17c = crate::Reg<fmc17c::Fmc17cSpec>;
#[doc = "Address Qualified Command"]
pub mod fmc17c;
#[doc = "FMCBUF (rw) register accessor: DMA Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmcbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmcbuf`] module"]
#[doc(alias = "FMCBUF")]
pub type Fmcbuf = crate::Reg<fmcbuf::FmcbufSpec>;
#[doc = "DMA Buffer Data Register"]
pub mod fmcbuf;
