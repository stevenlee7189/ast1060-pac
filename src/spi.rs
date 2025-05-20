#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    spi000: Spi000,
    spi004: Spi004,
    spi008: Spi008,
    spi00c: Spi00c,
    spi010: Spi010,
    spi014: Spi014,
    _reserved6: [u8; 0x18],
    spi030: Spi030,
    spi034: Spi034,
    _reserved8: [u8; 0x1c],
    spi054: Spi054,
    _reserved9: [u8; 0x28],
    spi080: Spi080,
    spi084: Spi084,
    spi088: Spi088,
    spi08c: Spi08c,
    spi090: Spi090,
    spi094: Spi094,
    spi098: Spi098,
    _reserved16: [u8; 0x04],
    spi0a0: Spi0a0,
    spi0a4: Spi0a4,
    spi0a8: Spi0a8,
    spi0ac: Spi0ac,
    spi0b0: Spi0b0,
    spi0b4: Spi0b4,
    spi0b8: Spi0b8,
    spi0bc: Spi0bc,
    spi0c0: Spi0c0,
    spi0c4: Spi0c4,
    _reserved26: [u8; 0x38],
    spi100: Spi100,
    spi104: Spi104,
    spi108: Spi108,
    spi10c: Spi10c,
    spi110: Spi110,
    spi114: Spi114,
    spi118: Spi118,
    spi11c: Spi11c,
    spirfqcd: [Spirfqcd; 12],
    spi150: Spi150,
    spi154: Spi154,
    spi158: Spi158,
    spi15c: Spi15c,
    spi160: Spi160,
    spi164: Spi164,
    spi168: Spi168,
    spi16c: Spi16c,
    spi170: Spi170,
    spi174: Spi174,
    spi178: Spi178,
    spi17c: Spi17c,
    _reserved47: [u8; 0x80],
    spirbuf: [Spirbuf; 64],
}
impl RegisterBlock {
    #[doc = "0x00 - SPI Flash Configuration Register"]
    #[inline(always)]
    pub const fn spi000(&self) -> &Spi000 {
        &self.spi000
    }
    #[doc = "0x04 - CE Control Register"]
    #[inline(always)]
    pub const fn spi004(&self) -> &Spi004 {
        &self.spi004
    }
    #[doc = "0x08 - Interrupt Control and Status Register"]
    #[inline(always)]
    pub const fn spi008(&self) -> &Spi008 {
        &self.spi008
    }
    #[doc = "0x0c - Command Control Register"]
    #[inline(always)]
    pub const fn spi00c(&self) -> &Spi00c {
        &self.spi00c
    }
    #[doc = "0x10 - CE0 Control Register"]
    #[inline(always)]
    pub const fn spi010(&self) -> &Spi010 {
        &self.spi010
    }
    #[doc = "0x14 - CE1 Control Register"]
    #[inline(always)]
    pub const fn spi014(&self) -> &Spi014 {
        &self.spi014
    }
    #[doc = "0x30 - CE0 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi030(&self) -> &Spi030 {
        &self.spi030
    }
    #[doc = "0x34 - CE1 Address Decoding Range Register"]
    #[inline(always)]
    pub const fn spi034(&self) -> &Spi034 {
        &self.spi034
    }
    #[doc = "0x54 - SPI Dummy Cycle Data Register"]
    #[inline(always)]
    pub const fn spi054(&self) -> &Spi054 {
        &self.spi054
    }
    #[doc = "0x80 - DMA Control/Status Register"]
    #[inline(always)]
    pub const fn spi080(&self) -> &Spi080 {
        &self.spi080
    }
    #[doc = "0x84 - DMA Flash Side Address"]
    #[inline(always)]
    pub const fn spi084(&self) -> &Spi084 {
        &self.spi084
    }
    #[doc = "0x88 - DMA DRAM/SRAM Side Address"]
    #[inline(always)]
    pub const fn spi088(&self) -> &Spi088 {
        &self.spi088
    }
    #[doc = "0x8c - DMA Length Register"]
    #[inline(always)]
    pub const fn spi08c(&self) -> &Spi08c {
        &self.spi08c
    }
    #[doc = "0x90 - CheckSum Calculation Result"]
    #[inline(always)]
    pub const fn spi090(&self) -> &Spi090 {
        &self.spi090
    }
    #[doc = "0x94 - CE0 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi094(&self) -> &Spi094 {
        &self.spi094
    }
    #[doc = "0x98 - CE1 SPI Flash Read Timing Compensation"]
    #[inline(always)]
    pub const fn spi098(&self) -> &Spi098 {
        &self.spi098
    }
    #[doc = "0xa0 - Command Filter Control Register"]
    #[inline(always)]
    pub const fn spi0a0(&self) -> &Spi0a0 {
        &self.spi0a0
    }
    #[doc = "0xa4 - Write Address Filter Control Register"]
    #[inline(always)]
    pub const fn spi0a4(&self) -> &Spi0a4 {
        &self.spi0a4
    }
    #[doc = "0xa8 - Register Lock Control Register (SRST\\#)"]
    #[inline(always)]
    pub const fn spi0a8(&self) -> &Spi0a8 {
        &self.spi0a8
    }
    #[doc = "0xac - Register Lock Control Register (Watchdog)"]
    #[inline(always)]
    pub const fn spi0ac(&self) -> &Spi0ac {
        &self.spi0ac
    }
    #[doc = "0xb0 - Write Address Filter Register \\#1"]
    #[inline(always)]
    pub const fn spi0b0(&self) -> &Spi0b0 {
        &self.spi0b0
    }
    #[doc = "0xb4 - Write Address Filter Register \\#2"]
    #[inline(always)]
    pub const fn spi0b4(&self) -> &Spi0b4 {
        &self.spi0b4
    }
    #[doc = "0xb8 - Write Address Filter Register \\#3"]
    #[inline(always)]
    pub const fn spi0b8(&self) -> &Spi0b8 {
        &self.spi0b8
    }
    #[doc = "0xbc - Write Address Filter Register \\#4"]
    #[inline(always)]
    pub const fn spi0bc(&self) -> &Spi0bc {
        &self.spi0bc
    }
    #[doc = "0xc0 - Write Address Filter Register \\#5"]
    #[inline(always)]
    pub const fn spi0c0(&self) -> &Spi0c0 {
        &self.spi0c0
    }
    #[doc = "0xc4 - Write Address Filter Register \\#6"]
    #[inline(always)]
    pub const fn spi0c4(&self) -> &Spi0c4 {
        &self.spi0c4
    }
    #[doc = "0x100 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi100(&self) -> &Spi100 {
        &self.spi100
    }
    #[doc = "0x104 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi104(&self) -> &Spi104 {
        &self.spi104
    }
    #[doc = "0x108 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi108(&self) -> &Spi108 {
        &self.spi108
    }
    #[doc = "0x10c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi10c(&self) -> &Spi10c {
        &self.spi10c
    }
    #[doc = "0x110 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi110(&self) -> &Spi110 {
        &self.spi110
    }
    #[doc = "0x114 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi114(&self) -> &Spi114 {
        &self.spi114
    }
    #[doc = "0x118 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi118(&self) -> &Spi118 {
        &self.spi118
    }
    #[doc = "0x11c - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spi11c(&self) -> &Spi11c {
        &self.spi11c
    }
    #[doc = "0x120..0x150 - Fully Qualified Command"]
    #[inline(always)]
    pub const fn spirfqcd(&self, n: usize) -> &Spirfqcd {
        &self.spirfqcd[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x120..0x150 - Fully Qualified Command"]
    #[inline(always)]
    pub fn spirfqcd_iter(&self) -> impl Iterator<Item = &Spirfqcd> {
        self.spirfqcd.iter()
    }
    #[doc = "0x150 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi150(&self) -> &Spi150 {
        &self.spi150
    }
    #[doc = "0x154 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi154(&self) -> &Spi154 {
        &self.spi154
    }
    #[doc = "0x158 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi158(&self) -> &Spi158 {
        &self.spi158
    }
    #[doc = "0x15c - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi15c(&self) -> &Spi15c {
        &self.spi15c
    }
    #[doc = "0x160 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi160(&self) -> &Spi160 {
        &self.spi160
    }
    #[doc = "0x164 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi164(&self) -> &Spi164 {
        &self.spi164
    }
    #[doc = "0x168 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi168(&self) -> &Spi168 {
        &self.spi168
    }
    #[doc = "0x16c - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi16c(&self) -> &Spi16c {
        &self.spi16c
    }
    #[doc = "0x170 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi170(&self) -> &Spi170 {
        &self.spi170
    }
    #[doc = "0x174 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi174(&self) -> &Spi174 {
        &self.spi174
    }
    #[doc = "0x178 - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi178(&self) -> &Spi178 {
        &self.spi178
    }
    #[doc = "0x17c - Address Qualified Command"]
    #[inline(always)]
    pub const fn spi17c(&self) -> &Spi17c {
        &self.spi17c
    }
    #[doc = "0x200..0x300 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub const fn spirbuf(&self, n: usize) -> &Spirbuf {
        &self.spirbuf[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x300 - DMA Buffer R/W Port"]
    #[inline(always)]
    pub fn spirbuf_iter(&self) -> impl Iterator<Item = &Spirbuf> {
        self.spirbuf.iter()
    }
}
#[doc = "SPI000 (rw) register accessor: SPI Flash Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi000`] module"]
#[doc(alias = "SPI000")]
pub type Spi000 = crate::Reg<spi000::Spi000Spec>;
#[doc = "SPI Flash Configuration Register"]
pub mod spi000;
#[doc = "SPI004 (rw) register accessor: CE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi004`] module"]
#[doc(alias = "SPI004")]
pub type Spi004 = crate::Reg<spi004::Spi004Spec>;
#[doc = "CE Control Register"]
pub mod spi004;
#[doc = "SPI008 (rw) register accessor: Interrupt Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi008`] module"]
#[doc(alias = "SPI008")]
pub type Spi008 = crate::Reg<spi008::Spi008Spec>;
#[doc = "Interrupt Control and Status Register"]
pub mod spi008;
#[doc = "SPI00C (rw) register accessor: Command Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi00c`] module"]
#[doc(alias = "SPI00C")]
pub type Spi00c = crate::Reg<spi00c::Spi00cSpec>;
#[doc = "Command Control Register"]
pub mod spi00c;
#[doc = "SPI010 (rw) register accessor: CE0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi010`] module"]
#[doc(alias = "SPI010")]
pub type Spi010 = crate::Reg<spi010::Spi010Spec>;
#[doc = "CE0 Control Register"]
pub mod spi010;
#[doc = "SPI014 (rw) register accessor: CE1 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi014`] module"]
#[doc(alias = "SPI014")]
pub type Spi014 = crate::Reg<spi014::Spi014Spec>;
#[doc = "CE1 Control Register"]
pub mod spi014;
#[doc = "SPI030 (rw) register accessor: CE0 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi030`] module"]
#[doc(alias = "SPI030")]
pub type Spi030 = crate::Reg<spi030::Spi030Spec>;
#[doc = "CE0 Address Decoding Range Register"]
pub mod spi030;
#[doc = "SPI034 (rw) register accessor: CE1 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi034`] module"]
#[doc(alias = "SPI034")]
pub type Spi034 = crate::Reg<spi034::Spi034Spec>;
#[doc = "CE1 Address Decoding Range Register"]
pub mod spi034;
#[doc = "SPI054 (rw) register accessor: SPI Dummy Cycle Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi054`] module"]
#[doc(alias = "SPI054")]
pub type Spi054 = crate::Reg<spi054::Spi054Spec>;
#[doc = "SPI Dummy Cycle Data Register"]
pub mod spi054;
#[doc = "SPI080 (rw) register accessor: DMA Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi080`] module"]
#[doc(alias = "SPI080")]
pub type Spi080 = crate::Reg<spi080::Spi080Spec>;
#[doc = "DMA Control/Status Register"]
pub mod spi080;
#[doc = "SPI084 (rw) register accessor: DMA Flash Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi084`] module"]
#[doc(alias = "SPI084")]
pub type Spi084 = crate::Reg<spi084::Spi084Spec>;
#[doc = "DMA Flash Side Address"]
pub mod spi084;
#[doc = "SPI088 (rw) register accessor: DMA DRAM/SRAM Side Address\n\nYou can [`read`](crate::Reg::read) this register and get [`spi088::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi088::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi088`] module"]
#[doc(alias = "SPI088")]
pub type Spi088 = crate::Reg<spi088::Spi088Spec>;
#[doc = "DMA DRAM/SRAM Side Address"]
pub mod spi088;
#[doc = "SPI08C (rw) register accessor: DMA Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi08c`] module"]
#[doc(alias = "SPI08C")]
pub type Spi08c = crate::Reg<spi08c::Spi08cSpec>;
#[doc = "DMA Length Register"]
pub mod spi08c;
#[doc = "SPI090 (rw) register accessor: CheckSum Calculation Result\n\nYou can [`read`](crate::Reg::read) this register and get [`spi090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi090`] module"]
#[doc(alias = "SPI090")]
pub type Spi090 = crate::Reg<spi090::Spi090Spec>;
#[doc = "CheckSum Calculation Result"]
pub mod spi090;
#[doc = "SPI094 (rw) register accessor: CE0 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi094::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi094::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi094`] module"]
#[doc(alias = "SPI094")]
pub type Spi094 = crate::Reg<spi094::Spi094Spec>;
#[doc = "CE0 SPI Flash Read Timing Compensation"]
pub mod spi094;
#[doc = "SPI098 (rw) register accessor: CE1 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`spi098::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi098::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi098`] module"]
#[doc(alias = "SPI098")]
pub type Spi098 = crate::Reg<spi098::Spi098Spec>;
#[doc = "CE1 SPI Flash Read Timing Compensation"]
pub mod spi098;
#[doc = "SPI0A0 (rw) register accessor: Command Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0a0`] module"]
#[doc(alias = "SPI0A0")]
pub type Spi0a0 = crate::Reg<spi0a0::Spi0a0Spec>;
#[doc = "Command Filter Control Register"]
pub mod spi0a0;
#[doc = "SPI0A4 (rw) register accessor: Write Address Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0a4`] module"]
#[doc(alias = "SPI0A4")]
pub type Spi0a4 = crate::Reg<spi0a4::Spi0a4Spec>;
#[doc = "Write Address Filter Control Register"]
pub mod spi0a4;
#[doc = "SPI0A8 (rw) register accessor: Register Lock Control Register (SRST\\#)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0a8`] module"]
#[doc(alias = "SPI0A8")]
pub type Spi0a8 = crate::Reg<spi0a8::Spi0a8Spec>;
#[doc = "Register Lock Control Register (SRST\\#)"]
pub mod spi0a8;
#[doc = "SPI0AC (rw) register accessor: Register Lock Control Register (Watchdog)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0ac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0ac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0ac`] module"]
#[doc(alias = "SPI0AC")]
pub type Spi0ac = crate::Reg<spi0ac::Spi0acSpec>;
#[doc = "Register Lock Control Register (Watchdog)"]
pub mod spi0ac;
#[doc = "SPI0B0 (rw) register accessor: Write Address Filter Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0b0`] module"]
#[doc(alias = "SPI0B0")]
pub type Spi0b0 = crate::Reg<spi0b0::Spi0b0Spec>;
#[doc = "Write Address Filter Register \\#1"]
pub mod spi0b0;
#[doc = "SPI0B4 (rw) register accessor: Write Address Filter Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0b4`] module"]
#[doc(alias = "SPI0B4")]
pub type Spi0b4 = crate::Reg<spi0b4::Spi0b4Spec>;
#[doc = "Write Address Filter Register \\#2"]
pub mod spi0b4;
#[doc = "SPI0B8 (rw) register accessor: Write Address Filter Register \\#3\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0b8`] module"]
#[doc(alias = "SPI0B8")]
pub type Spi0b8 = crate::Reg<spi0b8::Spi0b8Spec>;
#[doc = "Write Address Filter Register \\#3"]
pub mod spi0b8;
#[doc = "SPI0BC (rw) register accessor: Write Address Filter Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0bc`] module"]
#[doc(alias = "SPI0BC")]
pub type Spi0bc = crate::Reg<spi0bc::Spi0bcSpec>;
#[doc = "Write Address Filter Register \\#4"]
pub mod spi0bc;
#[doc = "SPI0C0 (rw) register accessor: Write Address Filter Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0c0`] module"]
#[doc(alias = "SPI0C0")]
pub type Spi0c0 = crate::Reg<spi0c0::Spi0c0Spec>;
#[doc = "Write Address Filter Register \\#5"]
pub mod spi0c0;
#[doc = "SPI0C4 (rw) register accessor: Write Address Filter Register \\#6\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0c4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0c4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi0c4`] module"]
#[doc(alias = "SPI0C4")]
pub type Spi0c4 = crate::Reg<spi0c4::Spi0c4Spec>;
#[doc = "Write Address Filter Register \\#6"]
pub mod spi0c4;
#[doc = "SPI100 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi100`] module"]
#[doc(alias = "SPI100")]
pub type Spi100 = crate::Reg<spi100::Spi100Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi100;
#[doc = "SPI104 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi104`] module"]
#[doc(alias = "SPI104")]
pub type Spi104 = crate::Reg<spi104::Spi104Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi104;
#[doc = "SPI108 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi108`] module"]
#[doc(alias = "SPI108")]
pub type Spi108 = crate::Reg<spi108::Spi108Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi108;
#[doc = "SPI10C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi10c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi10c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi10c`] module"]
#[doc(alias = "SPI10C")]
pub type Spi10c = crate::Reg<spi10c::Spi10cSpec>;
#[doc = "Fully Qualified Command"]
pub mod spi10c;
#[doc = "SPI110 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi110`] module"]
#[doc(alias = "SPI110")]
pub type Spi110 = crate::Reg<spi110::Spi110Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi110;
#[doc = "SPI114 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi114`] module"]
#[doc(alias = "SPI114")]
pub type Spi114 = crate::Reg<spi114::Spi114Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi114;
#[doc = "SPI118 (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi118`] module"]
#[doc(alias = "SPI118")]
pub type Spi118 = crate::Reg<spi118::Spi118Spec>;
#[doc = "Fully Qualified Command"]
pub mod spi118;
#[doc = "SPI11C (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi11c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi11c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi11c`] module"]
#[doc(alias = "SPI11C")]
pub type Spi11c = crate::Reg<spi11c::Spi11cSpec>;
#[doc = "Fully Qualified Command"]
pub mod spi11c;
#[doc = "SPIRFQCD (rw) register accessor: Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spirfqcd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spirfqcd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spirfqcd`] module"]
#[doc(alias = "SPIRFQCD")]
pub type Spirfqcd = crate::Reg<spirfqcd::SpirfqcdSpec>;
#[doc = "Fully Qualified Command"]
pub mod spirfqcd;
#[doc = "SPI150 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi150`] module"]
#[doc(alias = "SPI150")]
pub type Spi150 = crate::Reg<spi150::Spi150Spec>;
#[doc = "Address Qualified Command"]
pub mod spi150;
#[doc = "SPI154 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi154`] module"]
#[doc(alias = "SPI154")]
pub type Spi154 = crate::Reg<spi154::Spi154Spec>;
#[doc = "Address Qualified Command"]
pub mod spi154;
#[doc = "SPI158 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi158`] module"]
#[doc(alias = "SPI158")]
pub type Spi158 = crate::Reg<spi158::Spi158Spec>;
#[doc = "Address Qualified Command"]
pub mod spi158;
#[doc = "SPI15C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi15c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi15c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi15c`] module"]
#[doc(alias = "SPI15C")]
pub type Spi15c = crate::Reg<spi15c::Spi15cSpec>;
#[doc = "Address Qualified Command"]
pub mod spi15c;
#[doc = "SPI160 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi160`] module"]
#[doc(alias = "SPI160")]
pub type Spi160 = crate::Reg<spi160::Spi160Spec>;
#[doc = "Address Qualified Command"]
pub mod spi160;
#[doc = "SPI164 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi164`] module"]
#[doc(alias = "SPI164")]
pub type Spi164 = crate::Reg<spi164::Spi164Spec>;
#[doc = "Address Qualified Command"]
pub mod spi164;
#[doc = "SPI168 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi168`] module"]
#[doc(alias = "SPI168")]
pub type Spi168 = crate::Reg<spi168::Spi168Spec>;
#[doc = "Address Qualified Command"]
pub mod spi168;
#[doc = "SPI16C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi16c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi16c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi16c`] module"]
#[doc(alias = "SPI16C")]
pub type Spi16c = crate::Reg<spi16c::Spi16cSpec>;
#[doc = "Address Qualified Command"]
pub mod spi16c;
#[doc = "SPI170 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi170`] module"]
#[doc(alias = "SPI170")]
pub type Spi170 = crate::Reg<spi170::Spi170Spec>;
#[doc = "Address Qualified Command"]
pub mod spi170;
#[doc = "SPI174 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi174`] module"]
#[doc(alias = "SPI174")]
pub type Spi174 = crate::Reg<spi174::Spi174Spec>;
#[doc = "Address Qualified Command"]
pub mod spi174;
#[doc = "SPI178 (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi178`] module"]
#[doc(alias = "SPI178")]
pub type Spi178 = crate::Reg<spi178::Spi178Spec>;
#[doc = "Address Qualified Command"]
pub mod spi178;
#[doc = "SPI17C (rw) register accessor: Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi17c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi17c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spi17c`] module"]
#[doc(alias = "SPI17C")]
pub type Spi17c = crate::Reg<spi17c::Spi17cSpec>;
#[doc = "Address Qualified Command"]
pub mod spi17c;
#[doc = "SPIRBUF (rw) register accessor: DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spirbuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spirbuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spirbuf`] module"]
#[doc(alias = "SPIRBUF")]
pub type Spirbuf = crate::Reg<spirbuf::SpirbufSpec>;
#[doc = "DMA Buffer R/W Port"]
pub mod spirbuf;
