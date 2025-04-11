#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2cc00: I2cc00,
    i2cc04: I2cc04,
    i2cc08: I2cc08,
    i2cc0c: I2cc0c,
    i2cm10: I2cm10,
}
impl RegisterBlock {
    #[doc = "0x00 - Master/Slave Function Control Register"]
    #[inline(always)]
    pub const fn i2cc00(&self) -> &I2cc00 {
        &self.i2cc00
    }
    #[doc = "0x04 - Master/Slave Clock and AC Timing Control Register \\#1"]
    #[inline(always)]
    pub const fn i2cc04(&self) -> &I2cc04 {
        &self.i2cc04
    }
    #[doc = "0x08 - Master/Slave Transmit/Receive Byte Buffer Register"]
    #[inline(always)]
    pub const fn i2cc08(&self) -> &I2cc08 {
        &self.i2cc08
    }
    #[doc = "0x0c - Master/Slave Pool Buffer Control Register"]
    #[inline(always)]
    pub const fn i2cc0c(&self) -> &I2cc0c {
        &self.i2cc0c
    }
    #[doc = "0x10 - Master Interrupt Control Register"]
    #[inline(always)]
    pub const fn i2cm10(&self) -> &I2cm10 {
        &self.i2cm10
    }
}
#[doc = "I2CC00 (rw) register accessor: Master/Slave Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc00::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc00::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc00`] module"]
#[doc(alias = "I2CC00")]
pub type I2cc00 = crate::Reg<i2cc00::I2cc00Spec>;
#[doc = "Master/Slave Function Control Register"]
pub mod i2cc00;
#[doc = "I2CC04 (rw) register accessor: Master/Slave Clock and AC Timing Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc04`] module"]
#[doc(alias = "I2CC04")]
pub type I2cc04 = crate::Reg<i2cc04::I2cc04Spec>;
#[doc = "Master/Slave Clock and AC Timing Control Register \\#1"]
pub mod i2cc04;
#[doc = "I2CC08 (rw) register accessor: Master/Slave Transmit/Receive Byte Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc08`] module"]
#[doc(alias = "I2CC08")]
pub type I2cc08 = crate::Reg<i2cc08::I2cc08Spec>;
#[doc = "Master/Slave Transmit/Receive Byte Buffer Register"]
pub mod i2cc08;
#[doc = "I2CC0C (rw) register accessor: Master/Slave Pool Buffer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cc0c`] module"]
#[doc(alias = "I2CC0C")]
pub type I2cc0c = crate::Reg<i2cc0c::I2cc0cSpec>;
#[doc = "Master/Slave Pool Buffer Control Register"]
pub mod i2cc0c;
#[doc = "I2CM10 (rw) register accessor: Master Interrupt Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cm10`] module"]
#[doc(alias = "I2CM10")]
pub type I2cm10 = crate::Reg<i2cm10::I2cm10Spec>;
#[doc = "Master Interrupt Control Register"]
pub mod i2cm10;
