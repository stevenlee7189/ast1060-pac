#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i3cd000: I3cd000,
    i3cd004: I3cd004,
    i3cd008: I3cd008,
    i3cd00c: I3cd00c,
    i3cd010: I3cd010,
    i3cd014: I3cd014,
    i3cd018: I3cd018,
    i3cd01c: I3cd01c,
    i3cd020: I3cd020,
    i3cd024: I3cd024,
    _reserved10: [u8; 0x04],
    i3cd02c: I3cd02c,
    i3cd030: I3cd030,
    i3cd034: I3cd034,
    i3cd038: I3cd038,
    i3cd03c: I3cd03c,
    i3cd040: I3cd040,
    i3cd044: I3cd044,
    i3cd048: I3cd048,
    i3cd04c: I3cd04c,
    i3cd050: I3cd050,
    i3cd054: I3cd054,
    i3cd058: I3cd058,
    i3cd05c: I3cd05c,
    i3cd060: I3cd060,
    _reserved24: [u8; 0x08],
    i3cd06c: I3cd06c,
    i3cd070: I3cd070,
    i3cd074: I3cd074,
    i3cd078: I3cd078,
    i3cd07c: I3cd07c,
    i3cd080: I3cd080,
    i3cd084: I3cd084,
    _reserved31: [u8; 0x04],
    i3cd08c: I3cd08c,
    i3cd090: I3cd090,
    _reserved33: [u8; 0x1c],
    i3cd0b0: I3cd0b0,
    i3cd0b4: I3cd0b4,
    i3cd0b8: I3cd0b8,
    i3cd0bc: I3cd0bc,
    i3cd0c0: I3cd0c0,
    _reserved38: [u8; 0x04],
    i3cd0c8: I3cd0c8,
    i3cd0cc: I3cd0cc,
    i3cd0d0: I3cd0d0,
    i3cd0d4: I3cd0d4,
    i3cd0d8: I3cd0d8,
    i3cd0dc: I3cd0dc,
    i3cd0e0: I3cd0e0,
    i3cd0e4: I3cd0e4,
    i3cd0e8: I3cd0e8,
    i3cd0ec: I3cd0ec,
    _reserved48: [u8; 0x0110],
    i3cd200: I3cd200,
    i3cd204: I3cd204,
    i3cd208: I3cd208,
    i3cd20c: I3cd20c,
    i3cd210: I3cd210,
    i3cd214: I3cd214,
    i3cd218: I3cd218,
    i3cd21c: I3cd21c,
    i3cd220: I3cd220,
    i3cd224: I3cd224,
    i3cd228: I3cd228,
    i3cd22c: I3cd22c,
    i3cd230: I3cd230,
    i3cd234: I3cd234,
    i3cd238: I3cd238,
    i3cd23c: I3cd23c,
    i3cd240: I3cd240,
    i3cd244: I3cd244,
    i3cd248: I3cd248,
    i3cd24c: I3cd24c,
    i3cd250: I3cd250,
    i3cd254: I3cd254,
    i3cd258: I3cd258,
    i3cd25c: I3cd25c,
    i3cd260: I3cd260,
    i3cd264: I3cd264,
    i3cd268: I3cd268,
    i3cd26c: I3cd26c,
    i3cd270: I3cd270,
    i3cd274: I3cd274,
    i3cd278: I3cd278,
    i3cd27c: I3cd27c,
    i3cd280: I3cd280,
    i3cd284: I3cd284,
    i3cd288: I3cd288,
    i3cd28c: I3cd28c,
    i3cd290: I3cd290,
    i3cd294: I3cd294,
    i3cd298: I3cd298,
    i3cd29c: I3cd29c,
}
impl RegisterBlock {
    #[doc = "0x00 - Devicee Control Register"]
    #[inline(always)]
    pub const fn i3cd000(&self) -> &I3cd000 {
        &self.i3cd000
    }
    #[doc = "0x04 - Device Address Register"]
    #[inline(always)]
    pub const fn i3cd004(&self) -> &I3cd004 {
        &self.i3cd004
    }
    #[doc = "0x08 - Hardware Capability register"]
    #[inline(always)]
    pub const fn i3cd008(&self) -> &I3cd008 {
        &self.i3cd008
    }
    #[doc = "0x0c - COMMAND\\_QUEUE\\_PORT"]
    #[inline(always)]
    pub const fn i3cd00c(&self) -> &I3cd00c {
        &self.i3cd00c
    }
    #[doc = "0x10 - Response Queue"]
    #[inline(always)]
    pub const fn i3cd010(&self) -> &I3cd010 {
        &self.i3cd010
    }
    #[doc = "0x14 - Receive/Transmit Data Port Register"]
    #[inline(always)]
    pub const fn i3cd014(&self) -> &I3cd014 {
        &self.i3cd014
    }
    #[doc = "0x18 - In-Band Interrupt Queue Data Register"]
    #[inline(always)]
    pub const fn i3cd018(&self) -> &I3cd018 {
        &self.i3cd018
    }
    #[doc = "0x1c - Queue Threshold Control Register"]
    #[inline(always)]
    pub const fn i3cd01c(&self) -> &I3cd01c {
        &self.i3cd01c
    }
    #[doc = "0x20 - Data Buffer Threshold Control Register"]
    #[inline(always)]
    pub const fn i3cd020(&self) -> &I3cd020 {
        &self.i3cd020
    }
    #[doc = "0x24 - IBI Queue Control Register"]
    #[inline(always)]
    pub const fn i3cd024(&self) -> &I3cd024 {
        &self.i3cd024
    }
    #[doc = "0x2c - IBI MR Request Rejection Control Register"]
    #[inline(always)]
    pub const fn i3cd02c(&self) -> &I3cd02c {
        &self.i3cd02c
    }
    #[doc = "0x30 - IBI SIR Request Rejection Control Register"]
    #[inline(always)]
    pub const fn i3cd030(&self) -> &I3cd030 {
        &self.i3cd030
    }
    #[doc = "0x34 - Reset Control Register"]
    #[inline(always)]
    pub const fn i3cd034(&self) -> &I3cd034 {
        &self.i3cd034
    }
    #[doc = "0x38 - Slave Event Control Register"]
    #[inline(always)]
    pub const fn i3cd038(&self) -> &I3cd038 {
        &self.i3cd038
    }
    #[doc = "0x3c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn i3cd03c(&self) -> &I3cd03c {
        &self.i3cd03c
    }
    #[doc = "0x40 - Interrupt Status Enable Register"]
    #[inline(always)]
    pub const fn i3cd040(&self) -> &I3cd040 {
        &self.i3cd040
    }
    #[doc = "0x44 - Interrupt Signal Enable Register"]
    #[inline(always)]
    pub const fn i3cd044(&self) -> &I3cd044 {
        &self.i3cd044
    }
    #[doc = "0x48 - Interrupt Force Enable Register"]
    #[inline(always)]
    pub const fn i3cd048(&self) -> &I3cd048 {
        &self.i3cd048
    }
    #[doc = "0x4c - QUEUE STATUS LEVEL Register"]
    #[inline(always)]
    pub const fn i3cd04c(&self) -> &I3cd04c {
        &self.i3cd04c
    }
    #[doc = "0x50 - DATA BUFFER STATUS LEVEL Register"]
    #[inline(always)]
    pub const fn i3cd050(&self) -> &I3cd050 {
        &self.i3cd050
    }
    #[doc = "0x54 - PRESENT STATE Register"]
    #[inline(always)]
    pub const fn i3cd054(&self) -> &I3cd054 {
        &self.i3cd054
    }
    #[doc = "0x58 - Device Operating Status Register"]
    #[inline(always)]
    pub const fn i3cd058(&self) -> &I3cd058 {
        &self.i3cd058
    }
    #[doc = "0x5c - Pointer for Device Address Table Registers"]
    #[inline(always)]
    pub const fn i3cd05c(&self) -> &I3cd05c {
        &self.i3cd05c
    }
    #[doc = "0x60 - Pointer for Device Characteristics Table"]
    #[inline(always)]
    pub const fn i3cd060(&self) -> &I3cd060 {
        &self.i3cd060
    }
    #[doc = "0x6c - Pointer for Vendor specific Registers."]
    #[inline(always)]
    pub const fn i3cd06c(&self) -> &I3cd06c {
        &self.i3cd06c
    }
    #[doc = "0x70 - Provisional ID Register."]
    #[inline(always)]
    pub const fn i3cd070(&self) -> &I3cd070 {
        &self.i3cd070
    }
    #[doc = "0x74 - Provisional ID Register."]
    #[inline(always)]
    pub const fn i3cd074(&self) -> &I3cd074 {
        &self.i3cd074
    }
    #[doc = "0x78 - I3C Slave Characteristic Register"]
    #[inline(always)]
    pub const fn i3cd078(&self) -> &I3cd078 {
        &self.i3cd078
    }
    #[doc = "0x7c - I3C Max Write/Read Length Register"]
    #[inline(always)]
    pub const fn i3cd07c(&self) -> &I3cd07c {
        &self.i3cd07c
    }
    #[doc = "0x80 - MXDS Maximum Read Turnaround Time Register"]
    #[inline(always)]
    pub const fn i3cd080(&self) -> &I3cd080 {
        &self.i3cd080
    }
    #[doc = "0x84 - MXDS Maximum Data Speed Register"]
    #[inline(always)]
    pub const fn i3cd084(&self) -> &I3cd084 {
        &self.i3cd084
    }
    #[doc = "0x8c - Slave Interrupt Request Register"]
    #[inline(always)]
    pub const fn i3cd08c(&self) -> &I3cd08c {
        &self.i3cd08c
    }
    #[doc = "0x90 - TSP/TSL Symbol Timing Register"]
    #[inline(always)]
    pub const fn i3cd090(&self) -> &I3cd090 {
        &self.i3cd090
    }
    #[doc = "0xb0 - Device Control Extended Register"]
    #[inline(always)]
    pub const fn i3cd0b0(&self) -> &I3cd0b0 {
        &self.i3cd0b0
    }
    #[doc = "0xb4 - SCL I3C Open Drain Timing Register"]
    #[inline(always)]
    pub const fn i3cd0b4(&self) -> &I3cd0b4 {
        &self.i3cd0b4
    }
    #[doc = "0xb8 - SCL I3C Push Pull Timing Register"]
    #[inline(always)]
    pub const fn i3cd0b8(&self) -> &I3cd0b8 {
        &self.i3cd0b8
    }
    #[doc = "0xbc - SCL I2C Fast Mode Timing Register"]
    #[inline(always)]
    pub const fn i3cd0bc(&self) -> &I3cd0bc {
        &self.i3cd0bc
    }
    #[doc = "0xc0 - SCL I2C Fast Mode Plus Timing Register"]
    #[inline(always)]
    pub const fn i3cd0c0(&self) -> &I3cd0c0 {
        &self.i3cd0c0
    }
    #[doc = "0xc8 - SCL Extended Low Count Timing Register"]
    #[inline(always)]
    pub const fn i3cd0c8(&self) -> &I3cd0c8 {
        &self.i3cd0c8
    }
    #[doc = "0xcc - SCL Termination Bit Low count Timing Register"]
    #[inline(always)]
    pub const fn i3cd0cc(&self) -> &I3cd0cc {
        &self.i3cd0cc
    }
    #[doc = "0xd0 - SDA Hold and Mode Switch Delay Timing Register"]
    #[inline(always)]
    pub const fn i3cd0d0(&self) -> &I3cd0d0 {
        &self.i3cd0d0
    }
    #[doc = "0xd4 - Bus Free Timing Register"]
    #[inline(always)]
    pub const fn i3cd0d4(&self) -> &I3cd0d4 {
        &self.i3cd0d4
    }
    #[doc = "0xd8 - Bus Idle Timing Register"]
    #[inline(always)]
    pub const fn i3cd0d8(&self) -> &I3cd0d8 {
        &self.i3cd0d8
    }
    #[doc = "0xdc - SCL Low Master Extended Timeout Register"]
    #[inline(always)]
    pub const fn i3cd0dc(&self) -> &I3cd0dc {
        &self.i3cd0dc
    }
    #[doc = "0xe0 - I3C Version ID Register"]
    #[inline(always)]
    pub const fn i3cd0e0(&self) -> &I3cd0e0 {
        &self.i3cd0e0
    }
    #[doc = "0xe4 - I3C Version TYPE Register"]
    #[inline(always)]
    pub const fn i3cd0e4(&self) -> &I3cd0e4 {
        &self.i3cd0e4
    }
    #[doc = "0xe8 - I3C Queue Size Capability Register"]
    #[inline(always)]
    pub const fn i3cd0e8(&self) -> &I3cd0e8 {
        &self.i3cd0e8
    }
    #[doc = "0xec - I3C IBI Payload Length Register"]
    #[inline(always)]
    pub const fn i3cd0ec(&self) -> &I3cd0ec {
        &self.i3cd0ec
    }
    #[doc = "0x200 - Device Characteristic Table Location-1 of Device1 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd200(&self) -> &I3cd200 {
        &self.i3cd200
    }
    #[doc = "0x204 - Device Characteristic Table Location-2 of Device1 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd204(&self) -> &I3cd204 {
        &self.i3cd204
    }
    #[doc = "0x208 - Device Characteristic Table Location-3 of Device1 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd208(&self) -> &I3cd208 {
        &self.i3cd208
    }
    #[doc = "0x20c - Device Characteristic Table Location-4 of Device1 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd20c(&self) -> &I3cd20c {
        &self.i3cd20c
    }
    #[doc = "0x210 - Device Characteristic Table Location-1 of Device2 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd210(&self) -> &I3cd210 {
        &self.i3cd210
    }
    #[doc = "0x214 - Device Characteristic Table Location-2 of Device2 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd214(&self) -> &I3cd214 {
        &self.i3cd214
    }
    #[doc = "0x218 - Device Characteristic Table Location-2 of Device3 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd218(&self) -> &I3cd218 {
        &self.i3cd218
    }
    #[doc = "0x21c - Device Characteristic Table Location-2 of Device4 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd21c(&self) -> &I3cd21c {
        &self.i3cd21c
    }
    #[doc = "0x220 - Device Characteristic Table Location-1 of Device3 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd220(&self) -> &I3cd220 {
        &self.i3cd220
    }
    #[doc = "0x224 - Device Characteristic Table Location-2 of Device3 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd224(&self) -> &I3cd224 {
        &self.i3cd224
    }
    #[doc = "0x228 - Device Characteristic Table Location-3 of Device3 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd228(&self) -> &I3cd228 {
        &self.i3cd228
    }
    #[doc = "0x22c - Device Characteristic Table Location-4 of Device3 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd22c(&self) -> &I3cd22c {
        &self.i3cd22c
    }
    #[doc = "0x230 - Device Characteristic Table Location-1 of Device4 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd230(&self) -> &I3cd230 {
        &self.i3cd230
    }
    #[doc = "0x234 - Device Characteristic Table Location-2 of Device4 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd234(&self) -> &I3cd234 {
        &self.i3cd234
    }
    #[doc = "0x238 - Device Characteristic Table Location-3 of Device4 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd238(&self) -> &I3cd238 {
        &self.i3cd238
    }
    #[doc = "0x23c - Device Characteristic Table Location-4 of Device4 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd23c(&self) -> &I3cd23c {
        &self.i3cd23c
    }
    #[doc = "0x240 - Device Characteristic Table Location-1 of Device5 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd240(&self) -> &I3cd240 {
        &self.i3cd240
    }
    #[doc = "0x244 - Device Characteristic Table Location-2 of Device5 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd244(&self) -> &I3cd244 {
        &self.i3cd244
    }
    #[doc = "0x248 - Device Characteristic Table Location-3 of Device5 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd248(&self) -> &I3cd248 {
        &self.i3cd248
    }
    #[doc = "0x24c - Device Characteristic Table Location-4 of Device5 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd24c(&self) -> &I3cd24c {
        &self.i3cd24c
    }
    #[doc = "0x250 - Device Characteristic Table Location-1 of Device6 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd250(&self) -> &I3cd250 {
        &self.i3cd250
    }
    #[doc = "0x254 - Device Characteristic Table Location-2 of Device6 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd254(&self) -> &I3cd254 {
        &self.i3cd254
    }
    #[doc = "0x258 - Device Characteristic Table Location-3 of Device6 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd258(&self) -> &I3cd258 {
        &self.i3cd258
    }
    #[doc = "0x25c - Device Characteristic Table Location-4 of Device6 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd25c(&self) -> &I3cd25c {
        &self.i3cd25c
    }
    #[doc = "0x260 - Device Characteristic Table Location-1 of Device7 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd260(&self) -> &I3cd260 {
        &self.i3cd260
    }
    #[doc = "0x264 - Device Characteristic Table Location-2 of Device7 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd264(&self) -> &I3cd264 {
        &self.i3cd264
    }
    #[doc = "0x268 - Device Characteristic Table Location-3 of Device7 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd268(&self) -> &I3cd268 {
        &self.i3cd268
    }
    #[doc = "0x26c - Device Characteristic Table Location-4 of Device7 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd26c(&self) -> &I3cd26c {
        &self.i3cd26c
    }
    #[doc = "0x270 - Device Characteristic Table Location-1 of Device8 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd270(&self) -> &I3cd270 {
        &self.i3cd270
    }
    #[doc = "0x274 - Device Characteristic Table Location-2 of Device8 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd274(&self) -> &I3cd274 {
        &self.i3cd274
    }
    #[doc = "0x278 - Device Characteristic Table Location-3 of Device8 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd278(&self) -> &I3cd278 {
        &self.i3cd278
    }
    #[doc = "0x27c - Device Characteristic Table Location-4 of Device8 (Master Mode)"]
    #[inline(always)]
    pub const fn i3cd27c(&self) -> &I3cd27c {
        &self.i3cd27c
    }
    #[doc = "0x280 - Device Address Table of Device1"]
    #[inline(always)]
    pub const fn i3cd280(&self) -> &I3cd280 {
        &self.i3cd280
    }
    #[doc = "0x284 - Device Address Table of Device2"]
    #[inline(always)]
    pub const fn i3cd284(&self) -> &I3cd284 {
        &self.i3cd284
    }
    #[doc = "0x288 - Device Address Table of Device3"]
    #[inline(always)]
    pub const fn i3cd288(&self) -> &I3cd288 {
        &self.i3cd288
    }
    #[doc = "0x28c - Device Address Table of Device4"]
    #[inline(always)]
    pub const fn i3cd28c(&self) -> &I3cd28c {
        &self.i3cd28c
    }
    #[doc = "0x290 - Device Address Table of Device5"]
    #[inline(always)]
    pub const fn i3cd290(&self) -> &I3cd290 {
        &self.i3cd290
    }
    #[doc = "0x294 - Device Address Table of Device6"]
    #[inline(always)]
    pub const fn i3cd294(&self) -> &I3cd294 {
        &self.i3cd294
    }
    #[doc = "0x298 - Device Address Table of Device7"]
    #[inline(always)]
    pub const fn i3cd298(&self) -> &I3cd298 {
        &self.i3cd298
    }
    #[doc = "0x29c - Device Address Table of Device8"]
    #[inline(always)]
    pub const fn i3cd29c(&self) -> &I3cd29c {
        &self.i3cd29c
    }
}
#[doc = "I3CD000 (rw) register accessor: Devicee Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd000::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd000::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd000`] module"]
#[doc(alias = "I3CD000")]
pub type I3cd000 = crate::Reg<i3cd000::I3cd000Spec>;
#[doc = "Devicee Control Register"]
pub mod i3cd000;
#[doc = "I3CD004 (rw) register accessor: Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd004::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd004::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd004`] module"]
#[doc(alias = "I3CD004")]
pub type I3cd004 = crate::Reg<i3cd004::I3cd004Spec>;
#[doc = "Device Address Register"]
pub mod i3cd004;
#[doc = "I3CD008 (rw) register accessor: Hardware Capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd008::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd008::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd008`] module"]
#[doc(alias = "I3CD008")]
pub type I3cd008 = crate::Reg<i3cd008::I3cd008Spec>;
#[doc = "Hardware Capability register"]
pub mod i3cd008;
#[doc = "I3CD00C (rw) register accessor: COMMAND\\_QUEUE\\_PORT\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd00c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd00c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd00c`] module"]
#[doc(alias = "I3CD00C")]
pub type I3cd00c = crate::Reg<i3cd00c::I3cd00cSpec>;
#[doc = "COMMAND\\_QUEUE\\_PORT"]
pub mod i3cd00c;
#[doc = "I3CD010 (rw) register accessor: Response Queue\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd010::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd010::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd010`] module"]
#[doc(alias = "I3CD010")]
pub type I3cd010 = crate::Reg<i3cd010::I3cd010Spec>;
#[doc = "Response Queue"]
pub mod i3cd010;
#[doc = "I3CD014 (rw) register accessor: Receive/Transmit Data Port Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd014::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd014::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd014`] module"]
#[doc(alias = "I3CD014")]
pub type I3cd014 = crate::Reg<i3cd014::I3cd014Spec>;
#[doc = "Receive/Transmit Data Port Register"]
pub mod i3cd014;
#[doc = "I3CD018 (rw) register accessor: In-Band Interrupt Queue Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd018::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd018::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd018`] module"]
#[doc(alias = "I3CD018")]
pub type I3cd018 = crate::Reg<i3cd018::I3cd018Spec>;
#[doc = "In-Band Interrupt Queue Data Register"]
pub mod i3cd018;
#[doc = "I3CD01C (rw) register accessor: Queue Threshold Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd01c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd01c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd01c`] module"]
#[doc(alias = "I3CD01C")]
pub type I3cd01c = crate::Reg<i3cd01c::I3cd01cSpec>;
#[doc = "Queue Threshold Control Register"]
pub mod i3cd01c;
#[doc = "I3CD020 (rw) register accessor: Data Buffer Threshold Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd020::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd020::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd020`] module"]
#[doc(alias = "I3CD020")]
pub type I3cd020 = crate::Reg<i3cd020::I3cd020Spec>;
#[doc = "Data Buffer Threshold Control Register"]
pub mod i3cd020;
#[doc = "I3CD024 (rw) register accessor: IBI Queue Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd024::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd024::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd024`] module"]
#[doc(alias = "I3CD024")]
pub type I3cd024 = crate::Reg<i3cd024::I3cd024Spec>;
#[doc = "IBI Queue Control Register"]
pub mod i3cd024;
#[doc = "I3CD02C (rw) register accessor: IBI MR Request Rejection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd02c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd02c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd02c`] module"]
#[doc(alias = "I3CD02C")]
pub type I3cd02c = crate::Reg<i3cd02c::I3cd02cSpec>;
#[doc = "IBI MR Request Rejection Control Register"]
pub mod i3cd02c;
#[doc = "I3CD030 (rw) register accessor: IBI SIR Request Rejection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd030::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd030::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd030`] module"]
#[doc(alias = "I3CD030")]
pub type I3cd030 = crate::Reg<i3cd030::I3cd030Spec>;
#[doc = "IBI SIR Request Rejection Control Register"]
pub mod i3cd030;
#[doc = "I3CD034 (rw) register accessor: Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd034::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd034::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd034`] module"]
#[doc(alias = "I3CD034")]
pub type I3cd034 = crate::Reg<i3cd034::I3cd034Spec>;
#[doc = "Reset Control Register"]
pub mod i3cd034;
#[doc = "I3CD038 (rw) register accessor: Slave Event Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd038::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd038::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd038`] module"]
#[doc(alias = "I3CD038")]
pub type I3cd038 = crate::Reg<i3cd038::I3cd038Spec>;
#[doc = "Slave Event Control Register"]
pub mod i3cd038;
#[doc = "I3CD03C (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd03c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd03c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd03c`] module"]
#[doc(alias = "I3CD03C")]
pub type I3cd03c = crate::Reg<i3cd03c::I3cd03cSpec>;
#[doc = "Interrupt Status Register"]
pub mod i3cd03c;
#[doc = "I3CD040 (rw) register accessor: Interrupt Status Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd040::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd040::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd040`] module"]
#[doc(alias = "I3CD040")]
pub type I3cd040 = crate::Reg<i3cd040::I3cd040Spec>;
#[doc = "Interrupt Status Enable Register"]
pub mod i3cd040;
#[doc = "I3CD044 (rw) register accessor: Interrupt Signal Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd044::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd044::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd044`] module"]
#[doc(alias = "I3CD044")]
pub type I3cd044 = crate::Reg<i3cd044::I3cd044Spec>;
#[doc = "Interrupt Signal Enable Register"]
pub mod i3cd044;
#[doc = "I3CD048 (rw) register accessor: Interrupt Force Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd048::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd048::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd048`] module"]
#[doc(alias = "I3CD048")]
pub type I3cd048 = crate::Reg<i3cd048::I3cd048Spec>;
#[doc = "Interrupt Force Enable Register"]
pub mod i3cd048;
#[doc = "I3CD04C (rw) register accessor: QUEUE STATUS LEVEL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd04c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd04c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd04c`] module"]
#[doc(alias = "I3CD04C")]
pub type I3cd04c = crate::Reg<i3cd04c::I3cd04cSpec>;
#[doc = "QUEUE STATUS LEVEL Register"]
pub mod i3cd04c;
#[doc = "I3CD050 (rw) register accessor: DATA BUFFER STATUS LEVEL Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd050::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd050::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd050`] module"]
#[doc(alias = "I3CD050")]
pub type I3cd050 = crate::Reg<i3cd050::I3cd050Spec>;
#[doc = "DATA BUFFER STATUS LEVEL Register"]
pub mod i3cd050;
#[doc = "I3CD054 (rw) register accessor: PRESENT STATE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd054::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd054::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd054`] module"]
#[doc(alias = "I3CD054")]
pub type I3cd054 = crate::Reg<i3cd054::I3cd054Spec>;
#[doc = "PRESENT STATE Register"]
pub mod i3cd054;
#[doc = "I3CD058 (rw) register accessor: Device Operating Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd058::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd058::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd058`] module"]
#[doc(alias = "I3CD058")]
pub type I3cd058 = crate::Reg<i3cd058::I3cd058Spec>;
#[doc = "Device Operating Status Register"]
pub mod i3cd058;
#[doc = "I3CD05C (rw) register accessor: Pointer for Device Address Table Registers\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd05c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd05c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd05c`] module"]
#[doc(alias = "I3CD05C")]
pub type I3cd05c = crate::Reg<i3cd05c::I3cd05cSpec>;
#[doc = "Pointer for Device Address Table Registers"]
pub mod i3cd05c;
#[doc = "I3CD060 (rw) register accessor: Pointer for Device Characteristics Table\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd060::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd060::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd060`] module"]
#[doc(alias = "I3CD060")]
pub type I3cd060 = crate::Reg<i3cd060::I3cd060Spec>;
#[doc = "Pointer for Device Characteristics Table"]
pub mod i3cd060;
#[doc = "I3CD06C (rw) register accessor: Pointer for Vendor specific Registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd06c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd06c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd06c`] module"]
#[doc(alias = "I3CD06C")]
pub type I3cd06c = crate::Reg<i3cd06c::I3cd06cSpec>;
#[doc = "Pointer for Vendor specific Registers."]
pub mod i3cd06c;
#[doc = "I3CD070 (rw) register accessor: Provisional ID Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd070::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd070::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd070`] module"]
#[doc(alias = "I3CD070")]
pub type I3cd070 = crate::Reg<i3cd070::I3cd070Spec>;
#[doc = "Provisional ID Register."]
pub mod i3cd070;
#[doc = "I3CD074 (rw) register accessor: Provisional ID Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd074::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd074::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd074`] module"]
#[doc(alias = "I3CD074")]
pub type I3cd074 = crate::Reg<i3cd074::I3cd074Spec>;
#[doc = "Provisional ID Register."]
pub mod i3cd074;
#[doc = "I3CD078 (rw) register accessor: I3C Slave Characteristic Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd078::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd078::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd078`] module"]
#[doc(alias = "I3CD078")]
pub type I3cd078 = crate::Reg<i3cd078::I3cd078Spec>;
#[doc = "I3C Slave Characteristic Register"]
pub mod i3cd078;
#[doc = "I3CD07C (rw) register accessor: I3C Max Write/Read Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd07c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd07c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd07c`] module"]
#[doc(alias = "I3CD07C")]
pub type I3cd07c = crate::Reg<i3cd07c::I3cd07cSpec>;
#[doc = "I3C Max Write/Read Length Register"]
pub mod i3cd07c;
#[doc = "I3CD080 (rw) register accessor: MXDS Maximum Read Turnaround Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd080::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd080::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd080`] module"]
#[doc(alias = "I3CD080")]
pub type I3cd080 = crate::Reg<i3cd080::I3cd080Spec>;
#[doc = "MXDS Maximum Read Turnaround Time Register"]
pub mod i3cd080;
#[doc = "I3CD084 (rw) register accessor: MXDS Maximum Data Speed Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd084::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd084::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd084`] module"]
#[doc(alias = "I3CD084")]
pub type I3cd084 = crate::Reg<i3cd084::I3cd084Spec>;
#[doc = "MXDS Maximum Data Speed Register"]
pub mod i3cd084;
#[doc = "I3CD08C (rw) register accessor: Slave Interrupt Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd08c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd08c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd08c`] module"]
#[doc(alias = "I3CD08C")]
pub type I3cd08c = crate::Reg<i3cd08c::I3cd08cSpec>;
#[doc = "Slave Interrupt Request Register"]
pub mod i3cd08c;
#[doc = "I3CD090 (rw) register accessor: TSP/TSL Symbol Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd090::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd090::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd090`] module"]
#[doc(alias = "I3CD090")]
pub type I3cd090 = crate::Reg<i3cd090::I3cd090Spec>;
#[doc = "TSP/TSL Symbol Timing Register"]
pub mod i3cd090;
#[doc = "I3CD0B0 (rw) register accessor: Device Control Extended Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0b0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0b0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0b0`] module"]
#[doc(alias = "I3CD0B0")]
pub type I3cd0b0 = crate::Reg<i3cd0b0::I3cd0b0Spec>;
#[doc = "Device Control Extended Register"]
pub mod i3cd0b0;
#[doc = "I3CD0B4 (rw) register accessor: SCL I3C Open Drain Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0b4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0b4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0b4`] module"]
#[doc(alias = "I3CD0B4")]
pub type I3cd0b4 = crate::Reg<i3cd0b4::I3cd0b4Spec>;
#[doc = "SCL I3C Open Drain Timing Register"]
pub mod i3cd0b4;
#[doc = "I3CD0B8 (rw) register accessor: SCL I3C Push Pull Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0b8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0b8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0b8`] module"]
#[doc(alias = "I3CD0B8")]
pub type I3cd0b8 = crate::Reg<i3cd0b8::I3cd0b8Spec>;
#[doc = "SCL I3C Push Pull Timing Register"]
pub mod i3cd0b8;
#[doc = "I3CD0BC (rw) register accessor: SCL I2C Fast Mode Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0bc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0bc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0bc`] module"]
#[doc(alias = "I3CD0BC")]
pub type I3cd0bc = crate::Reg<i3cd0bc::I3cd0bcSpec>;
#[doc = "SCL I2C Fast Mode Timing Register"]
pub mod i3cd0bc;
#[doc = "I3CD0C0 (rw) register accessor: SCL I2C Fast Mode Plus Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0c0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0c0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0c0`] module"]
#[doc(alias = "I3CD0C0")]
pub type I3cd0c0 = crate::Reg<i3cd0c0::I3cd0c0Spec>;
#[doc = "SCL I2C Fast Mode Plus Timing Register"]
pub mod i3cd0c0;
#[doc = "I3CD0C8 (rw) register accessor: SCL Extended Low Count Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0c8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0c8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0c8`] module"]
#[doc(alias = "I3CD0C8")]
pub type I3cd0c8 = crate::Reg<i3cd0c8::I3cd0c8Spec>;
#[doc = "SCL Extended Low Count Timing Register"]
pub mod i3cd0c8;
#[doc = "I3CD0CC (rw) register accessor: SCL Termination Bit Low count Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0cc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0cc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0cc`] module"]
#[doc(alias = "I3CD0CC")]
pub type I3cd0cc = crate::Reg<i3cd0cc::I3cd0ccSpec>;
#[doc = "SCL Termination Bit Low count Timing Register"]
pub mod i3cd0cc;
#[doc = "I3CD0D0 (rw) register accessor: SDA Hold and Mode Switch Delay Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0d0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0d0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0d0`] module"]
#[doc(alias = "I3CD0D0")]
pub type I3cd0d0 = crate::Reg<i3cd0d0::I3cd0d0Spec>;
#[doc = "SDA Hold and Mode Switch Delay Timing Register"]
pub mod i3cd0d0;
#[doc = "I3CD0D4 (rw) register accessor: Bus Free Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0d4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0d4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0d4`] module"]
#[doc(alias = "I3CD0D4")]
pub type I3cd0d4 = crate::Reg<i3cd0d4::I3cd0d4Spec>;
#[doc = "Bus Free Timing Register"]
pub mod i3cd0d4;
#[doc = "I3CD0D8 (rw) register accessor: Bus Idle Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0d8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0d8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0d8`] module"]
#[doc(alias = "I3CD0D8")]
pub type I3cd0d8 = crate::Reg<i3cd0d8::I3cd0d8Spec>;
#[doc = "Bus Idle Timing Register"]
pub mod i3cd0d8;
#[doc = "I3CD0DC (rw) register accessor: SCL Low Master Extended Timeout Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0dc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0dc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0dc`] module"]
#[doc(alias = "I3CD0DC")]
pub type I3cd0dc = crate::Reg<i3cd0dc::I3cd0dcSpec>;
#[doc = "SCL Low Master Extended Timeout Register"]
pub mod i3cd0dc;
#[doc = "I3CD0E0 (rw) register accessor: I3C Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0e0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0e0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0e0`] module"]
#[doc(alias = "I3CD0E0")]
pub type I3cd0e0 = crate::Reg<i3cd0e0::I3cd0e0Spec>;
#[doc = "I3C Version ID Register"]
pub mod i3cd0e0;
#[doc = "I3CD0E4 (rw) register accessor: I3C Version TYPE Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0e4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0e4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0e4`] module"]
#[doc(alias = "I3CD0E4")]
pub type I3cd0e4 = crate::Reg<i3cd0e4::I3cd0e4Spec>;
#[doc = "I3C Version TYPE Register"]
pub mod i3cd0e4;
#[doc = "I3CD0E8 (rw) register accessor: I3C Queue Size Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0e8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0e8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0e8`] module"]
#[doc(alias = "I3CD0E8")]
pub type I3cd0e8 = crate::Reg<i3cd0e8::I3cd0e8Spec>;
#[doc = "I3C Queue Size Capability Register"]
pub mod i3cd0e8;
#[doc = "I3CD0EC (rw) register accessor: I3C IBI Payload Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0ec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0ec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd0ec`] module"]
#[doc(alias = "I3CD0EC")]
pub type I3cd0ec = crate::Reg<i3cd0ec::I3cd0ecSpec>;
#[doc = "I3C IBI Payload Length Register"]
pub mod i3cd0ec;
#[doc = "I3CD200 (rw) register accessor: Device Characteristic Table Location-1 of Device1 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd200`] module"]
#[doc(alias = "I3CD200")]
pub type I3cd200 = crate::Reg<i3cd200::I3cd200Spec>;
#[doc = "Device Characteristic Table Location-1 of Device1 (Master Mode)"]
pub mod i3cd200;
#[doc = "I3CD204 (rw) register accessor: Device Characteristic Table Location-2 of Device1 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd204`] module"]
#[doc(alias = "I3CD204")]
pub type I3cd204 = crate::Reg<i3cd204::I3cd204Spec>;
#[doc = "Device Characteristic Table Location-2 of Device1 (Master Mode)"]
pub mod i3cd204;
#[doc = "I3CD208 (rw) register accessor: Device Characteristic Table Location-3 of Device1 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd208`] module"]
#[doc(alias = "I3CD208")]
pub type I3cd208 = crate::Reg<i3cd208::I3cd208Spec>;
#[doc = "Device Characteristic Table Location-3 of Device1 (Master Mode)"]
pub mod i3cd208;
#[doc = "I3CD20C (rw) register accessor: Device Characteristic Table Location-4 of Device1 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd20c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd20c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd20c`] module"]
#[doc(alias = "I3CD20C")]
pub type I3cd20c = crate::Reg<i3cd20c::I3cd20cSpec>;
#[doc = "Device Characteristic Table Location-4 of Device1 (Master Mode)"]
pub mod i3cd20c;
#[doc = "I3CD210 (rw) register accessor: Device Characteristic Table Location-1 of Device2 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd210`] module"]
#[doc(alias = "I3CD210")]
pub type I3cd210 = crate::Reg<i3cd210::I3cd210Spec>;
#[doc = "Device Characteristic Table Location-1 of Device2 (Master Mode)"]
pub mod i3cd210;
#[doc = "I3CD214 (rw) register accessor: Device Characteristic Table Location-2 of Device2 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd214`] module"]
#[doc(alias = "I3CD214")]
pub type I3cd214 = crate::Reg<i3cd214::I3cd214Spec>;
#[doc = "Device Characteristic Table Location-2 of Device2 (Master Mode)"]
pub mod i3cd214;
#[doc = "I3CD218 (rw) register accessor: Device Characteristic Table Location-2 of Device3 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd218`] module"]
#[doc(alias = "I3CD218")]
pub type I3cd218 = crate::Reg<i3cd218::I3cd218Spec>;
#[doc = "Device Characteristic Table Location-2 of Device3 (Master Mode)"]
pub mod i3cd218;
#[doc = "I3CD21C (rw) register accessor: Device Characteristic Table Location-2 of Device4 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd21c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd21c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd21c`] module"]
#[doc(alias = "I3CD21C")]
pub type I3cd21c = crate::Reg<i3cd21c::I3cd21cSpec>;
#[doc = "Device Characteristic Table Location-2 of Device4 (Master Mode)"]
pub mod i3cd21c;
#[doc = "I3CD220 (rw) register accessor: Device Characteristic Table Location-1 of Device3 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd220`] module"]
#[doc(alias = "I3CD220")]
pub type I3cd220 = crate::Reg<i3cd220::I3cd220Spec>;
#[doc = "Device Characteristic Table Location-1 of Device3 (Master Mode)"]
pub mod i3cd220;
#[doc = "I3CD224 (rw) register accessor: Device Characteristic Table Location-2 of Device3 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd224`] module"]
#[doc(alias = "I3CD224")]
pub type I3cd224 = crate::Reg<i3cd224::I3cd224Spec>;
#[doc = "Device Characteristic Table Location-2 of Device3 (Master Mode)"]
pub mod i3cd224;
#[doc = "I3CD228 (rw) register accessor: Device Characteristic Table Location-3 of Device3 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd228`] module"]
#[doc(alias = "I3CD228")]
pub type I3cd228 = crate::Reg<i3cd228::I3cd228Spec>;
#[doc = "Device Characteristic Table Location-3 of Device3 (Master Mode)"]
pub mod i3cd228;
#[doc = "I3CD22C (rw) register accessor: Device Characteristic Table Location-4 of Device3 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd22c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd22c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd22c`] module"]
#[doc(alias = "I3CD22C")]
pub type I3cd22c = crate::Reg<i3cd22c::I3cd22cSpec>;
#[doc = "Device Characteristic Table Location-4 of Device3 (Master Mode)"]
pub mod i3cd22c;
#[doc = "I3CD230 (rw) register accessor: Device Characteristic Table Location-1 of Device4 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd230`] module"]
#[doc(alias = "I3CD230")]
pub type I3cd230 = crate::Reg<i3cd230::I3cd230Spec>;
#[doc = "Device Characteristic Table Location-1 of Device4 (Master Mode)"]
pub mod i3cd230;
#[doc = "I3CD234 (rw) register accessor: Device Characteristic Table Location-2 of Device4 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd234`] module"]
#[doc(alias = "I3CD234")]
pub type I3cd234 = crate::Reg<i3cd234::I3cd234Spec>;
#[doc = "Device Characteristic Table Location-2 of Device4 (Master Mode)"]
pub mod i3cd234;
#[doc = "I3CD238 (rw) register accessor: Device Characteristic Table Location-3 of Device4 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd238`] module"]
#[doc(alias = "I3CD238")]
pub type I3cd238 = crate::Reg<i3cd238::I3cd238Spec>;
#[doc = "Device Characteristic Table Location-3 of Device4 (Master Mode)"]
pub mod i3cd238;
#[doc = "I3CD23C (rw) register accessor: Device Characteristic Table Location-4 of Device4 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd23c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd23c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd23c`] module"]
#[doc(alias = "I3CD23C")]
pub type I3cd23c = crate::Reg<i3cd23c::I3cd23cSpec>;
#[doc = "Device Characteristic Table Location-4 of Device4 (Master Mode)"]
pub mod i3cd23c;
#[doc = "I3CD240 (rw) register accessor: Device Characteristic Table Location-1 of Device5 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd240`] module"]
#[doc(alias = "I3CD240")]
pub type I3cd240 = crate::Reg<i3cd240::I3cd240Spec>;
#[doc = "Device Characteristic Table Location-1 of Device5 (Master Mode)"]
pub mod i3cd240;
#[doc = "I3CD244 (rw) register accessor: Device Characteristic Table Location-2 of Device5 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd244`] module"]
#[doc(alias = "I3CD244")]
pub type I3cd244 = crate::Reg<i3cd244::I3cd244Spec>;
#[doc = "Device Characteristic Table Location-2 of Device5 (Master Mode)"]
pub mod i3cd244;
#[doc = "I3CD248 (rw) register accessor: Device Characteristic Table Location-3 of Device5 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd248`] module"]
#[doc(alias = "I3CD248")]
pub type I3cd248 = crate::Reg<i3cd248::I3cd248Spec>;
#[doc = "Device Characteristic Table Location-3 of Device5 (Master Mode)"]
pub mod i3cd248;
#[doc = "I3CD24C (rw) register accessor: Device Characteristic Table Location-4 of Device5 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd24c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd24c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd24c`] module"]
#[doc(alias = "I3CD24C")]
pub type I3cd24c = crate::Reg<i3cd24c::I3cd24cSpec>;
#[doc = "Device Characteristic Table Location-4 of Device5 (Master Mode)"]
pub mod i3cd24c;
#[doc = "I3CD250 (rw) register accessor: Device Characteristic Table Location-1 of Device6 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd250`] module"]
#[doc(alias = "I3CD250")]
pub type I3cd250 = crate::Reg<i3cd250::I3cd250Spec>;
#[doc = "Device Characteristic Table Location-1 of Device6 (Master Mode)"]
pub mod i3cd250;
#[doc = "I3CD254 (rw) register accessor: Device Characteristic Table Location-2 of Device6 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd254`] module"]
#[doc(alias = "I3CD254")]
pub type I3cd254 = crate::Reg<i3cd254::I3cd254Spec>;
#[doc = "Device Characteristic Table Location-2 of Device6 (Master Mode)"]
pub mod i3cd254;
#[doc = "I3CD258 (rw) register accessor: Device Characteristic Table Location-3 of Device6 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd258`] module"]
#[doc(alias = "I3CD258")]
pub type I3cd258 = crate::Reg<i3cd258::I3cd258Spec>;
#[doc = "Device Characteristic Table Location-3 of Device6 (Master Mode)"]
pub mod i3cd258;
#[doc = "I3CD25C (rw) register accessor: Device Characteristic Table Location-4 of Device6 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd25c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd25c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd25c`] module"]
#[doc(alias = "I3CD25C")]
pub type I3cd25c = crate::Reg<i3cd25c::I3cd25cSpec>;
#[doc = "Device Characteristic Table Location-4 of Device6 (Master Mode)"]
pub mod i3cd25c;
#[doc = "I3CD260 (rw) register accessor: Device Characteristic Table Location-1 of Device7 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd260`] module"]
#[doc(alias = "I3CD260")]
pub type I3cd260 = crate::Reg<i3cd260::I3cd260Spec>;
#[doc = "Device Characteristic Table Location-1 of Device7 (Master Mode)"]
pub mod i3cd260;
#[doc = "I3CD264 (rw) register accessor: Device Characteristic Table Location-2 of Device7 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd264`] module"]
#[doc(alias = "I3CD264")]
pub type I3cd264 = crate::Reg<i3cd264::I3cd264Spec>;
#[doc = "Device Characteristic Table Location-2 of Device7 (Master Mode)"]
pub mod i3cd264;
#[doc = "I3CD268 (rw) register accessor: Device Characteristic Table Location-3 of Device7 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd268`] module"]
#[doc(alias = "I3CD268")]
pub type I3cd268 = crate::Reg<i3cd268::I3cd268Spec>;
#[doc = "Device Characteristic Table Location-3 of Device7 (Master Mode)"]
pub mod i3cd268;
#[doc = "I3CD26C (rw) register accessor: Device Characteristic Table Location-4 of Device7 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd26c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd26c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd26c`] module"]
#[doc(alias = "I3CD26C")]
pub type I3cd26c = crate::Reg<i3cd26c::I3cd26cSpec>;
#[doc = "Device Characteristic Table Location-4 of Device7 (Master Mode)"]
pub mod i3cd26c;
#[doc = "I3CD270 (rw) register accessor: Device Characteristic Table Location-1 of Device8 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd270`] module"]
#[doc(alias = "I3CD270")]
pub type I3cd270 = crate::Reg<i3cd270::I3cd270Spec>;
#[doc = "Device Characteristic Table Location-1 of Device8 (Master Mode)"]
pub mod i3cd270;
#[doc = "I3CD274 (rw) register accessor: Device Characteristic Table Location-2 of Device8 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd274`] module"]
#[doc(alias = "I3CD274")]
pub type I3cd274 = crate::Reg<i3cd274::I3cd274Spec>;
#[doc = "Device Characteristic Table Location-2 of Device8 (Master Mode)"]
pub mod i3cd274;
#[doc = "I3CD278 (rw) register accessor: Device Characteristic Table Location-3 of Device8 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd278`] module"]
#[doc(alias = "I3CD278")]
pub type I3cd278 = crate::Reg<i3cd278::I3cd278Spec>;
#[doc = "Device Characteristic Table Location-3 of Device8 (Master Mode)"]
pub mod i3cd278;
#[doc = "I3CD27C (rw) register accessor: Device Characteristic Table Location-4 of Device8 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd27c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd27c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd27c`] module"]
#[doc(alias = "I3CD27C")]
pub type I3cd27c = crate::Reg<i3cd27c::I3cd27cSpec>;
#[doc = "Device Characteristic Table Location-4 of Device8 (Master Mode)"]
pub mod i3cd27c;
#[doc = "I3CD280 (rw) register accessor: Device Address Table of Device1\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd280`] module"]
#[doc(alias = "I3CD280")]
pub type I3cd280 = crate::Reg<i3cd280::I3cd280Spec>;
#[doc = "Device Address Table of Device1"]
pub mod i3cd280;
#[doc = "I3CD284 (rw) register accessor: Device Address Table of Device2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd284::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd284::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd284`] module"]
#[doc(alias = "I3CD284")]
pub type I3cd284 = crate::Reg<i3cd284::I3cd284Spec>;
#[doc = "Device Address Table of Device2"]
pub mod i3cd284;
#[doc = "I3CD288 (rw) register accessor: Device Address Table of Device3\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd288::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd288::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd288`] module"]
#[doc(alias = "I3CD288")]
pub type I3cd288 = crate::Reg<i3cd288::I3cd288Spec>;
#[doc = "Device Address Table of Device3"]
pub mod i3cd288;
#[doc = "I3CD28C (rw) register accessor: Device Address Table of Device4\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd28c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd28c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd28c`] module"]
#[doc(alias = "I3CD28C")]
pub type I3cd28c = crate::Reg<i3cd28c::I3cd28cSpec>;
#[doc = "Device Address Table of Device4"]
pub mod i3cd28c;
#[doc = "I3CD290 (rw) register accessor: Device Address Table of Device5\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd290::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd290::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd290`] module"]
#[doc(alias = "I3CD290")]
pub type I3cd290 = crate::Reg<i3cd290::I3cd290Spec>;
#[doc = "Device Address Table of Device5"]
pub mod i3cd290;
#[doc = "I3CD294 (rw) register accessor: Device Address Table of Device6\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd294::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd294::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd294`] module"]
#[doc(alias = "I3CD294")]
pub type I3cd294 = crate::Reg<i3cd294::I3cd294Spec>;
#[doc = "Device Address Table of Device6"]
pub mod i3cd294;
#[doc = "I3CD298 (rw) register accessor: Device Address Table of Device7\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd298::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd298::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd298`] module"]
#[doc(alias = "I3CD298")]
pub type I3cd298 = crate::Reg<i3cd298::I3cd298Spec>;
#[doc = "Device Address Table of Device7"]
pub mod i3cd298;
#[doc = "I3CD29C (rw) register accessor: Device Address Table of Device8\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd29c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd29c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i3cd29c`] module"]
#[doc(alias = "I3CD29C")]
pub type I3cd29c = crate::Reg<i3cd29c::I3cd29cSpec>;
#[doc = "Device Address Table of Device8"]
pub mod i3cd29c;
