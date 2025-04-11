#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hace00: Hace00,
    hace04: Hace04,
    hace08: Hace08,
    hace0c: Hace0c,
    hace10: Hace10,
    hace14: Hace14,
    hace18: Hace18,
    hace1c: Hace1c,
    hace20: Hace20,
    hace24: Hace24,
    hace28: Hace28,
    hace2c: Hace2c,
    hace30: Hace30,
    hace34: Hace34,
    _reserved14: [u8; 0x18],
    hace50: Hace50,
    hace54: Hace54,
    hace58: Hace58,
    hace5c: Hace5c,
    hace60: Hace60,
    hace64: Hace64,
}
impl RegisterBlock {
    #[doc = "0x00 - Crypto Data Source Base Address Register"]
    #[inline(always)]
    pub const fn hace00(&self) -> &Hace00 {
        &self.hace00
    }
    #[doc = "0x04 - Crypto Data Destination Base Address Register"]
    #[inline(always)]
    pub const fn hace04(&self) -> &Hace04 {
        &self.hace04
    }
    #[doc = "0x08 - Crypto Context Buffer Base Address Registerr"]
    #[inline(always)]
    pub const fn hace08(&self) -> &Hace08 {
        &self.hace08
    }
    #[doc = "0x0c - Crypto Data Length Register"]
    #[inline(always)]
    pub const fn hace0c(&self) -> &Hace0c {
        &self.hace0c
    }
    #[doc = "0x10 - Crypto Engine Command Register"]
    #[inline(always)]
    pub const fn hace10(&self) -> &Hace10 {
        &self.hace10
    }
    #[doc = "0x14 - Crypto AES-GCM Additional Data Length Register"]
    #[inline(always)]
    pub const fn hace14(&self) -> &Hace14 {
        &self.hace14
    }
    #[doc = "0x18 - Crypto AES-GCM Tag Write Buffer Base Address Register"]
    #[inline(always)]
    pub const fn hace18(&self) -> &Hace18 {
        &self.hace18
    }
    #[doc = "0x1c - HAC Engine Status Register"]
    #[inline(always)]
    pub const fn hace1c(&self) -> &Hace1c {
        &self.hace1c
    }
    #[doc = "0x20 - Hash Data Source Base Address Register"]
    #[inline(always)]
    pub const fn hace20(&self) -> &Hace20 {
        &self.hace20
    }
    #[doc = "0x24 - Hash Digest Write Buffer Base Address Register"]
    #[inline(always)]
    pub const fn hace24(&self) -> &Hace24 {
        &self.hace24
    }
    #[doc = "0x28 - Hash HMAC Key Buffer Base Address Register"]
    #[inline(always)]
    pub const fn hace28(&self) -> &Hace28 {
        &self.hace28
    }
    #[doc = "0x2c - Hash Data Length Register"]
    #[inline(always)]
    pub const fn hace2c(&self) -> &Hace2c {
        &self.hace2c
    }
    #[doc = "0x30 - Hash Engine Command Register"]
    #[inline(always)]
    pub const fn hace30(&self) -> &Hace30 {
        &self.hace30
    }
    #[doc = "0x34 - Hash Data Padding Length Register"]
    #[inline(always)]
    pub const fn hace34(&self) -> &Hace34 {
        &self.hace34
    }
    #[doc = "0x50 - Command Queue Base Address"]
    #[inline(always)]
    pub const fn hace50(&self) -> &Hace50 {
        &self.hace50
    }
    #[doc = "0x54 - Command Queue End Pointer"]
    #[inline(always)]
    pub const fn hace54(&self) -> &Hace54 {
        &self.hace54
    }
    #[doc = "0x58 - Command Queue Write Pointer"]
    #[inline(always)]
    pub const fn hace58(&self) -> &Hace58 {
        &self.hace58
    }
    #[doc = "0x5c - Command Queue Read Pointer"]
    #[inline(always)]
    pub const fn hace5c(&self) -> &Hace5c {
        &self.hace5c
    }
    #[doc = "0x60 - HAC Engine Feature Register"]
    #[inline(always)]
    pub const fn hace60(&self) -> &Hace60 {
        &self.hace60
    }
    #[doc = "0x64 - HAC Software Tag Register"]
    #[inline(always)]
    pub const fn hace64(&self) -> &Hace64 {
        &self.hace64
    }
}
#[doc = "HACE00 (rw) register accessor: Crypto Data Source Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace00`] module"]
#[doc(alias = "HACE00")]
pub type Hace00 = crate::Reg<hace00::Hace00Spec>;
#[doc = "Crypto Data Source Base Address Register"]
pub mod hace00;
#[doc = "HACE04 (rw) register accessor: Crypto Data Destination Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace04`] module"]
#[doc(alias = "HACE04")]
pub type Hace04 = crate::Reg<hace04::Hace04Spec>;
#[doc = "Crypto Data Destination Base Address Register"]
pub mod hace04;
#[doc = "HACE08 (rw) register accessor: Crypto Context Buffer Base Address Registerr\n\nYou can [`read`](crate::Reg::read) this register and get [`hace08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace08`] module"]
#[doc(alias = "HACE08")]
pub type Hace08 = crate::Reg<hace08::Hace08Spec>;
#[doc = "Crypto Context Buffer Base Address Registerr"]
pub mod hace08;
#[doc = "HACE0C (rw) register accessor: Crypto Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace0c`] module"]
#[doc(alias = "HACE0C")]
pub type Hace0c = crate::Reg<hace0c::Hace0cSpec>;
#[doc = "Crypto Data Length Register"]
pub mod hace0c;
#[doc = "HACE10 (rw) register accessor: Crypto Engine Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace10`] module"]
#[doc(alias = "HACE10")]
pub type Hace10 = crate::Reg<hace10::Hace10Spec>;
#[doc = "Crypto Engine Command Register"]
pub mod hace10;
#[doc = "HACE14 (rw) register accessor: Crypto AES-GCM Additional Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace14`] module"]
#[doc(alias = "HACE14")]
pub type Hace14 = crate::Reg<hace14::Hace14Spec>;
#[doc = "Crypto AES-GCM Additional Data Length Register"]
pub mod hace14;
#[doc = "HACE18 (rw) register accessor: Crypto AES-GCM Tag Write Buffer Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace18`] module"]
#[doc(alias = "HACE18")]
pub type Hace18 = crate::Reg<hace18::Hace18Spec>;
#[doc = "Crypto AES-GCM Tag Write Buffer Base Address Register"]
pub mod hace18;
#[doc = "HACE1C (rw) register accessor: HAC Engine Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace1c`] module"]
#[doc(alias = "HACE1C")]
pub type Hace1c = crate::Reg<hace1c::Hace1cSpec>;
#[doc = "HAC Engine Status Register"]
pub mod hace1c;
#[doc = "HACE20 (rw) register accessor: Hash Data Source Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace20`] module"]
#[doc(alias = "HACE20")]
pub type Hace20 = crate::Reg<hace20::Hace20Spec>;
#[doc = "Hash Data Source Base Address Register"]
pub mod hace20;
#[doc = "HACE24 (rw) register accessor: Hash Digest Write Buffer Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace24`] module"]
#[doc(alias = "HACE24")]
pub type Hace24 = crate::Reg<hace24::Hace24Spec>;
#[doc = "Hash Digest Write Buffer Base Address Register"]
pub mod hace24;
#[doc = "HACE28 (rw) register accessor: Hash HMAC Key Buffer Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace28`] module"]
#[doc(alias = "HACE28")]
pub type Hace28 = crate::Reg<hace28::Hace28Spec>;
#[doc = "Hash HMAC Key Buffer Base Address Register"]
pub mod hace28;
#[doc = "HACE2C (rw) register accessor: Hash Data Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace2c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace2c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace2c`] module"]
#[doc(alias = "HACE2C")]
pub type Hace2c = crate::Reg<hace2c::Hace2cSpec>;
#[doc = "Hash Data Length Register"]
pub mod hace2c;
#[doc = "HACE30 (rw) register accessor: Hash Engine Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace30`] module"]
#[doc(alias = "HACE30")]
pub type Hace30 = crate::Reg<hace30::Hace30Spec>;
#[doc = "Hash Engine Command Register"]
pub mod hace30;
#[doc = "HACE34 (rw) register accessor: Hash Data Padding Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace34`] module"]
#[doc(alias = "HACE34")]
pub type Hace34 = crate::Reg<hace34::Hace34Spec>;
#[doc = "Hash Data Padding Length Register"]
pub mod hace34;
#[doc = "HACE50 (rw) register accessor: Command Queue Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`hace50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace50`] module"]
#[doc(alias = "HACE50")]
pub type Hace50 = crate::Reg<hace50::Hace50Spec>;
#[doc = "Command Queue Base Address"]
pub mod hace50;
#[doc = "HACE54 (rw) register accessor: Command Queue End Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`hace54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace54`] module"]
#[doc(alias = "HACE54")]
pub type Hace54 = crate::Reg<hace54::Hace54Spec>;
#[doc = "Command Queue End Pointer"]
pub mod hace54;
#[doc = "HACE58 (rw) register accessor: Command Queue Write Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`hace58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace58`] module"]
#[doc(alias = "HACE58")]
pub type Hace58 = crate::Reg<hace58::Hace58Spec>;
#[doc = "Command Queue Write Pointer"]
pub mod hace58;
#[doc = "HACE5C (rw) register accessor: Command Queue Read Pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`hace5c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace5c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace5c`] module"]
#[doc(alias = "HACE5C")]
pub type Hace5c = crate::Reg<hace5c::Hace5cSpec>;
#[doc = "Command Queue Read Pointer"]
pub mod hace5c;
#[doc = "HACE60 (rw) register accessor: HAC Engine Feature Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace60`] module"]
#[doc(alias = "HACE60")]
pub type Hace60 = crate::Reg<hace60::Hace60Spec>;
#[doc = "HAC Engine Feature Register"]
pub mod hace60;
#[doc = "HACE64 (rw) register accessor: HAC Software Tag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hace64`] module"]
#[doc(alias = "HACE64")]
pub type Hace64 = crate::Reg<hace64::Hace64Spec>;
#[doc = "HAC Software Tag Register"]
pub mod hace64;
