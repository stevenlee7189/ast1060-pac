#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    i2cfilterthr04: I2cfilterthr04,
    i2cfilterthr08: I2cfilterthr08,
    i2cfilterthr0c: I2cfilterthr0c,
    i2cfilterthr10: I2cfilterthr10,
    i2cfilterthr14: I2cfilterthr14,
    i2cfilterthr18: I2cfilterthr18,
    _reserved6: [u8; 0x04],
    i2cfilterthr20: I2cfilterthr20,
    i2cfilterthr24: I2cfilterthr24,
    _reserved8: [u8; 0x18],
    i2cfilterthr40: I2cfilterthr40,
    i2cfilterthr44: I2cfilterthr44,
    i2cfilterthr48: I2cfilterthr48,
    i2cfilterthr4c: I2cfilterthr4c,
    _reserved12: [u8; 0x10],
    i2cfilterthr60: I2cfilterthr60,
}
impl RegisterBlock {
    #[doc = "0x04 - I2CFLT\\_THRN\\_EN"]
    #[inline(always)]
    pub const fn i2cfilterthr04(&self) -> &I2cfilterthr04 {
        &self.i2cfilterthr04
    }
    #[doc = "0x08 - I2CFLT\\_THRN\\_ADR"]
    #[inline(always)]
    pub const fn i2cfilterthr08(&self) -> &I2cfilterthr08 {
        &self.i2cfilterthr08
    }
    #[doc = "0x0c - I2CFLT\\_THRN\\_CFG"]
    #[inline(always)]
    pub const fn i2cfilterthr0c(&self) -> &I2cfilterthr0c {
        &self.i2cfilterthr0c
    }
    #[doc = "0x10 - I2CFLT\\_THRN\\_TMR"]
    #[inline(always)]
    pub const fn i2cfilterthr10(&self) -> &I2cfilterthr10 {
        &self.i2cfilterthr10
    }
    #[doc = "0x14 - I2CFLT\\_THRN\\_INTEN"]
    #[inline(always)]
    pub const fn i2cfilterthr14(&self) -> &I2cfilterthr14 {
        &self.i2cfilterthr14
    }
    #[doc = "0x18 - I2CFLT\\_THRN\\_INTS"]
    #[inline(always)]
    pub const fn i2cfilterthr18(&self) -> &I2cfilterthr18 {
        &self.i2cfilterthr18
    }
    #[doc = "0x20 - I2CFLT\\_THRN\\_STATUS"]
    #[inline(always)]
    pub const fn i2cfilterthr20(&self) -> &I2cfilterthr20 {
        &self.i2cfilterthr20
    }
    #[doc = "0x24 - I2CFLT\\_THRN\\_SEQ"]
    #[inline(always)]
    pub const fn i2cfilterthr24(&self) -> &I2cfilterthr24 {
        &self.i2cfilterthr24
    }
    #[doc = "0x40 - I2CFLT\\_THRN\\_MAP0"]
    #[inline(always)]
    pub const fn i2cfilterthr40(&self) -> &I2cfilterthr40 {
        &self.i2cfilterthr40
    }
    #[doc = "0x44 - I2CFLT\\_THRN\\_MAP1"]
    #[inline(always)]
    pub const fn i2cfilterthr44(&self) -> &I2cfilterthr44 {
        &self.i2cfilterthr44
    }
    #[doc = "0x48 - I2CFLT\\_THRN\\_MAP2"]
    #[inline(always)]
    pub const fn i2cfilterthr48(&self) -> &I2cfilterthr48 {
        &self.i2cfilterthr48
    }
    #[doc = "0x4c - I2CFLT\\_THRN\\_MAP3"]
    #[inline(always)]
    pub const fn i2cfilterthr4c(&self) -> &I2cfilterthr4c {
        &self.i2cfilterthr4c
    }
    #[doc = "0x60 - I2CFLT\\_THRN\\_INFO"]
    #[inline(always)]
    pub const fn i2cfilterthr60(&self) -> &I2cfilterthr60 {
        &self.i2cfilterthr60
    }
}
#[doc = "I2CFILTERTHR04 (rw) register accessor: I2CFLT\\_THRN\\_EN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr04::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr04::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr04`] module"]
#[doc(alias = "I2CFILTERTHR04")]
pub type I2cfilterthr04 = crate::Reg<i2cfilterthr04::I2cfilterthr04Spec>;
#[doc = "I2CFLT\\_THRN\\_EN"]
pub mod i2cfilterthr04;
#[doc = "I2CFILTERTHR08 (rw) register accessor: I2CFLT\\_THRN\\_ADR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr08::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr08::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr08`] module"]
#[doc(alias = "I2CFILTERTHR08")]
pub type I2cfilterthr08 = crate::Reg<i2cfilterthr08::I2cfilterthr08Spec>;
#[doc = "I2CFLT\\_THRN\\_ADR"]
pub mod i2cfilterthr08;
#[doc = "I2CFILTERTHR0C (rw) register accessor: I2CFLT\\_THRN\\_CFG\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr0c`] module"]
#[doc(alias = "I2CFILTERTHR0C")]
pub type I2cfilterthr0c = crate::Reg<i2cfilterthr0c::I2cfilterthr0cSpec>;
#[doc = "I2CFLT\\_THRN\\_CFG"]
pub mod i2cfilterthr0c;
#[doc = "I2CFILTERTHR10 (rw) register accessor: I2CFLT\\_THRN\\_TMR\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr10`] module"]
#[doc(alias = "I2CFILTERTHR10")]
pub type I2cfilterthr10 = crate::Reg<i2cfilterthr10::I2cfilterthr10Spec>;
#[doc = "I2CFLT\\_THRN\\_TMR"]
pub mod i2cfilterthr10;
#[doc = "I2CFILTERTHR14 (rw) register accessor: I2CFLT\\_THRN\\_INTEN\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr14`] module"]
#[doc(alias = "I2CFILTERTHR14")]
pub type I2cfilterthr14 = crate::Reg<i2cfilterthr14::I2cfilterthr14Spec>;
#[doc = "I2CFLT\\_THRN\\_INTEN"]
pub mod i2cfilterthr14;
#[doc = "I2CFILTERTHR18 (rw) register accessor: I2CFLT\\_THRN\\_INTS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr18`] module"]
#[doc(alias = "I2CFILTERTHR18")]
pub type I2cfilterthr18 = crate::Reg<i2cfilterthr18::I2cfilterthr18Spec>;
#[doc = "I2CFLT\\_THRN\\_INTS"]
pub mod i2cfilterthr18;
#[doc = "I2CFILTERTHR20 (rw) register accessor: I2CFLT\\_THRN\\_STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr20`] module"]
#[doc(alias = "I2CFILTERTHR20")]
pub type I2cfilterthr20 = crate::Reg<i2cfilterthr20::I2cfilterthr20Spec>;
#[doc = "I2CFLT\\_THRN\\_STATUS"]
pub mod i2cfilterthr20;
#[doc = "I2CFILTERTHR24 (rw) register accessor: I2CFLT\\_THRN\\_SEQ\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr24`] module"]
#[doc(alias = "I2CFILTERTHR24")]
pub type I2cfilterthr24 = crate::Reg<i2cfilterthr24::I2cfilterthr24Spec>;
#[doc = "I2CFLT\\_THRN\\_SEQ"]
pub mod i2cfilterthr24;
#[doc = "I2CFILTERTHR40 (rw) register accessor: I2CFLT\\_THRN\\_MAP0\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr40`] module"]
#[doc(alias = "I2CFILTERTHR40")]
pub type I2cfilterthr40 = crate::Reg<i2cfilterthr40::I2cfilterthr40Spec>;
#[doc = "I2CFLT\\_THRN\\_MAP0"]
pub mod i2cfilterthr40;
#[doc = "I2CFILTERTHR44 (rw) register accessor: I2CFLT\\_THRN\\_MAP1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr44`] module"]
#[doc(alias = "I2CFILTERTHR44")]
pub type I2cfilterthr44 = crate::Reg<i2cfilterthr44::I2cfilterthr44Spec>;
#[doc = "I2CFLT\\_THRN\\_MAP1"]
pub mod i2cfilterthr44;
#[doc = "I2CFILTERTHR48 (rw) register accessor: I2CFLT\\_THRN\\_MAP2\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr48`] module"]
#[doc(alias = "I2CFILTERTHR48")]
pub type I2cfilterthr48 = crate::Reg<i2cfilterthr48::I2cfilterthr48Spec>;
#[doc = "I2CFLT\\_THRN\\_MAP2"]
pub mod i2cfilterthr48;
#[doc = "I2CFILTERTHR4C (rw) register accessor: I2CFLT\\_THRN\\_MAP3\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr4c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr4c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr4c`] module"]
#[doc(alias = "I2CFILTERTHR4C")]
pub type I2cfilterthr4c = crate::Reg<i2cfilterthr4c::I2cfilterthr4cSpec>;
#[doc = "I2CFLT\\_THRN\\_MAP3"]
pub mod i2cfilterthr4c;
#[doc = "I2CFILTERTHR60 (rw) register accessor: I2CFLT\\_THRN\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cfilterthr60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cfilterthr60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2cfilterthr60`] module"]
#[doc(alias = "I2CFILTERTHR60")]
pub type I2cfilterthr60 = crate::Reg<i2cfilterthr60::I2cfilterthr60Spec>;
#[doc = "I2CFLT\\_THRN\\_INFO"]
pub mod i2cfilterthr60;
