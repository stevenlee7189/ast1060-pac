#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    buff0: Buff0,
    buff1: Buff1,
    buff2: Buff2,
    buff3: Buff3,
    buff4: Buff4,
    buff5: Buff5,
    buff6: Buff6,
    buff7: Buff7,
}
impl RegisterBlock {
    #[doc = "0x00 - I2C Buffer Mode Buff0"]
    #[inline(always)]
    pub const fn buff0(&self) -> &Buff0 {
        &self.buff0
    }
    #[doc = "0x04 - I2C Buffer Mode Buff1"]
    #[inline(always)]
    pub const fn buff1(&self) -> &Buff1 {
        &self.buff1
    }
    #[doc = "0x08 - I2C Buffer Mode Buff2"]
    #[inline(always)]
    pub const fn buff2(&self) -> &Buff2 {
        &self.buff2
    }
    #[doc = "0x0c - I2C Buffer Mode Buff3"]
    #[inline(always)]
    pub const fn buff3(&self) -> &Buff3 {
        &self.buff3
    }
    #[doc = "0x10 - I2C Buffer Mode Buff4"]
    #[inline(always)]
    pub const fn buff4(&self) -> &Buff4 {
        &self.buff4
    }
    #[doc = "0x14 - I2C Buffer Mode Buff5"]
    #[inline(always)]
    pub const fn buff5(&self) -> &Buff5 {
        &self.buff5
    }
    #[doc = "0x18 - I2C Buffer Mode Buff6"]
    #[inline(always)]
    pub const fn buff6(&self) -> &Buff6 {
        &self.buff6
    }
    #[doc = "0x1c - I2C Buffer Mode Buff7"]
    #[inline(always)]
    pub const fn buff7(&self) -> &Buff7 {
        &self.buff7
    }
}
#[doc = "BUFF0 (rw) register accessor: I2C Buffer Mode Buff0\n\nYou can [`read`](crate::Reg::read) this register and get [`buff0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff0`] module"]
#[doc(alias = "BUFF0")]
pub type Buff0 = crate::Reg<buff0::Buff0Spec>;
#[doc = "I2C Buffer Mode Buff0"]
pub mod buff0;
#[doc = "BUFF1 (rw) register accessor: I2C Buffer Mode Buff1\n\nYou can [`read`](crate::Reg::read) this register and get [`buff1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff1`] module"]
#[doc(alias = "BUFF1")]
pub type Buff1 = crate::Reg<buff1::Buff1Spec>;
#[doc = "I2C Buffer Mode Buff1"]
pub mod buff1;
#[doc = "BUFF2 (rw) register accessor: I2C Buffer Mode Buff2\n\nYou can [`read`](crate::Reg::read) this register and get [`buff2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff2`] module"]
#[doc(alias = "BUFF2")]
pub type Buff2 = crate::Reg<buff2::Buff2Spec>;
#[doc = "I2C Buffer Mode Buff2"]
pub mod buff2;
#[doc = "BUFF3 (rw) register accessor: I2C Buffer Mode Buff3\n\nYou can [`read`](crate::Reg::read) this register and get [`buff3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff3`] module"]
#[doc(alias = "BUFF3")]
pub type Buff3 = crate::Reg<buff3::Buff3Spec>;
#[doc = "I2C Buffer Mode Buff3"]
pub mod buff3;
#[doc = "BUFF4 (rw) register accessor: I2C Buffer Mode Buff4\n\nYou can [`read`](crate::Reg::read) this register and get [`buff4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff4`] module"]
#[doc(alias = "BUFF4")]
pub type Buff4 = crate::Reg<buff4::Buff4Spec>;
#[doc = "I2C Buffer Mode Buff4"]
pub mod buff4;
#[doc = "BUFF5 (rw) register accessor: I2C Buffer Mode Buff5\n\nYou can [`read`](crate::Reg::read) this register and get [`buff5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff5`] module"]
#[doc(alias = "BUFF5")]
pub type Buff5 = crate::Reg<buff5::Buff5Spec>;
#[doc = "I2C Buffer Mode Buff5"]
pub mod buff5;
#[doc = "BUFF6 (rw) register accessor: I2C Buffer Mode Buff6\n\nYou can [`read`](crate::Reg::read) this register and get [`buff6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff6`] module"]
#[doc(alias = "BUFF6")]
pub type Buff6 = crate::Reg<buff6::Buff6Spec>;
#[doc = "I2C Buffer Mode Buff6"]
pub mod buff6;
#[doc = "BUFF7 (rw) register accessor: I2C Buffer Mode Buff7\n\nYou can [`read`](crate::Reg::read) this register and get [`buff7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buff7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@buff7`] module"]
#[doc(alias = "BUFF7")]
pub type Buff7 = crate::Reg<buff7::Buff7Spec>;
#[doc = "I2C Buffer Mode Buff7"]
pub mod buff7;
