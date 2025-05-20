#[doc = "Register `FMC004` reader"]
pub type R = crate::R<Fmc004Spec>;
#[doc = "Register `FMC004` writer"]
pub type W = crate::W<Fmc004Spec>;
#[doc = "CE0 SPI address mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ce0spiaddrModeSel {
    #[doc = "0: (3B)3 bytes (smaller than or equal to 16MB)"]
    _3b3BytesSmallerThanOrEqualTo16mb = 0,
    #[doc = "1: (4B)4 bytes (larger than 16MB)"]
    _4b4BytesLargerThan16mb = 1,
}
impl From<Ce0spiaddrModeSel> for bool {
    #[inline(always)]
    fn from(variant: Ce0spiaddrModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE0SPIAddrModeSel` reader - CE0 SPI address mode selection"]
pub type Ce0spiaddrModeSelR = crate::BitReader<Ce0spiaddrModeSel>;
impl Ce0spiaddrModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ce0spiaddrModeSel {
        match self.bits {
            false => Ce0spiaddrModeSel::_3b3BytesSmallerThanOrEqualTo16mb,
            true => Ce0spiaddrModeSel::_4b4BytesLargerThan16mb,
        }
    }
    #[doc = "(3B)3 bytes (smaller than or equal to 16MB)"]
    #[inline(always)]
    pub fn is_3b3_bytes_smaller_than_or_equal_to_16mb(&self) -> bool {
        *self == Ce0spiaddrModeSel::_3b3BytesSmallerThanOrEqualTo16mb
    }
    #[doc = "(4B)4 bytes (larger than 16MB)"]
    #[inline(always)]
    pub fn is_4b4_bytes_larger_than_16mb(&self) -> bool {
        *self == Ce0spiaddrModeSel::_4b4BytesLargerThan16mb
    }
}
#[doc = "Field `CE0SPIAddrModeSel` writer - CE0 SPI address mode selection"]
pub type Ce0spiaddrModeSelW<'a, REG> = crate::BitWriter<'a, REG, Ce0spiaddrModeSel>;
impl<'a, REG> Ce0spiaddrModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "(3B)3 bytes (smaller than or equal to 16MB)"]
    #[inline(always)]
    pub fn _3b3_bytes_smaller_than_or_equal_to_16mb(self) -> &'a mut crate::W<REG> {
        self.variant(Ce0spiaddrModeSel::_3b3BytesSmallerThanOrEqualTo16mb)
    }
    #[doc = "(4B)4 bytes (larger than 16MB)"]
    #[inline(always)]
    pub fn _4b4_bytes_larger_than_16mb(self) -> &'a mut crate::W<REG> {
        self.variant(Ce0spiaddrModeSel::_4b4BytesLargerThan16mb)
    }
}
#[doc = "Field `CE1SPIAddrModeSel` reader - CE1 SPI address mode selection"]
pub type Ce1spiaddrModeSelR = crate::BitReader;
#[doc = "Field `CE1SPIAddrModeSel` writer - CE1 SPI address mode selection"]
pub type Ce1spiaddrModeSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "CE0 4B address Auto-Read command selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ce04baddrAutoReadCmdSel {
    #[doc = "0: use 0x03 command"]
    Use0x03Command = 0,
    #[doc = "1: use 0x13 command"]
    Use0x13Command = 1,
}
impl From<Ce04baddrAutoReadCmdSel> for bool {
    #[inline(always)]
    fn from(variant: Ce04baddrAutoReadCmdSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE04BAddrAutoReadCmdSel` reader - CE0 4B address Auto-Read command selection"]
pub type Ce04baddrAutoReadCmdSelR = crate::BitReader<Ce04baddrAutoReadCmdSel>;
impl Ce04baddrAutoReadCmdSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ce04baddrAutoReadCmdSel {
        match self.bits {
            false => Ce04baddrAutoReadCmdSel::Use0x03Command,
            true => Ce04baddrAutoReadCmdSel::Use0x13Command,
        }
    }
    #[doc = "use 0x03 command"]
    #[inline(always)]
    pub fn is_use_0x03_command(&self) -> bool {
        *self == Ce04baddrAutoReadCmdSel::Use0x03Command
    }
    #[doc = "use 0x13 command"]
    #[inline(always)]
    pub fn is_use_0x13_command(&self) -> bool {
        *self == Ce04baddrAutoReadCmdSel::Use0x13Command
    }
}
#[doc = "Field `CE04BAddrAutoReadCmdSel` writer - CE0 4B address Auto-Read command selection"]
pub type Ce04baddrAutoReadCmdSelW<'a, REG> = crate::BitWriter<'a, REG, Ce04baddrAutoReadCmdSel>;
impl<'a, REG> Ce04baddrAutoReadCmdSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use 0x03 command"]
    #[inline(always)]
    pub fn use_0x03_command(self) -> &'a mut crate::W<REG> {
        self.variant(Ce04baddrAutoReadCmdSel::Use0x03Command)
    }
    #[doc = "use 0x13 command"]
    #[inline(always)]
    pub fn use_0x13_command(self) -> &'a mut crate::W<REG> {
        self.variant(Ce04baddrAutoReadCmdSel::Use0x13Command)
    }
}
#[doc = "Field `CE14BAddrAutoReadCmdSel` reader - CE1 4B address Auto-Read command selection"]
pub type Ce14baddrAutoReadCmdSelR = crate::BitReader;
#[doc = "Field `CE14BAddrAutoReadCmdSel` writer - CE1 4B address Auto-Read command selection"]
pub type Ce14baddrAutoReadCmdSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Select CE0 minimum inactive timing (tCSH)\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SelectCe0minimumInactiveTimingTcsh {
    #[doc = "0: 16 x HCLK"]
    _16XHclk = 0,
    #[doc = "1: 24 x HCLK"]
    _24XHclk = 1,
    #[doc = "2: 32 x HCLK"]
    _32XHclk = 2,
    #[doc = "3: 40 x HCLK"]
    _40XHclk = 3,
}
impl From<SelectCe0minimumInactiveTimingTcsh> for u8 {
    #[inline(always)]
    fn from(variant: SelectCe0minimumInactiveTimingTcsh) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SelectCe0minimumInactiveTimingTcsh {
    type Ux = u8;
}
impl crate::IsEnum for SelectCe0minimumInactiveTimingTcsh {}
#[doc = "Field `SelectCE0MinimumInactiveTimingTCSH` reader - Select CE0 minimum inactive timing (tCSH)"]
pub type SelectCe0minimumInactiveTimingTcshR =
    crate::FieldReader<SelectCe0minimumInactiveTimingTcsh>;
impl SelectCe0minimumInactiveTimingTcshR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SelectCe0minimumInactiveTimingTcsh {
        match self.bits {
            0 => SelectCe0minimumInactiveTimingTcsh::_16XHclk,
            1 => SelectCe0minimumInactiveTimingTcsh::_24XHclk,
            2 => SelectCe0minimumInactiveTimingTcsh::_32XHclk,
            3 => SelectCe0minimumInactiveTimingTcsh::_40XHclk,
            _ => unreachable!(),
        }
    }
    #[doc = "16 x HCLK"]
    #[inline(always)]
    pub fn is_16_x_hclk(&self) -> bool {
        *self == SelectCe0minimumInactiveTimingTcsh::_16XHclk
    }
    #[doc = "24 x HCLK"]
    #[inline(always)]
    pub fn is_24_x_hclk(&self) -> bool {
        *self == SelectCe0minimumInactiveTimingTcsh::_24XHclk
    }
    #[doc = "32 x HCLK"]
    #[inline(always)]
    pub fn is_32_x_hclk(&self) -> bool {
        *self == SelectCe0minimumInactiveTimingTcsh::_32XHclk
    }
    #[doc = "40 x HCLK"]
    #[inline(always)]
    pub fn is_40_x_hclk(&self) -> bool {
        *self == SelectCe0minimumInactiveTimingTcsh::_40XHclk
    }
}
#[doc = "Field `SelectCE0MinimumInactiveTimingTCSH` writer - Select CE0 minimum inactive timing (tCSH)"]
pub type SelectCe0minimumInactiveTimingTcshW<'a, REG> =
    crate::FieldWriter<'a, REG, 2, SelectCe0minimumInactiveTimingTcsh, crate::Safe>;
impl<'a, REG> SelectCe0minimumInactiveTimingTcshW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "16 x HCLK"]
    #[inline(always)]
    pub fn _16_x_hclk(self) -> &'a mut crate::W<REG> {
        self.variant(SelectCe0minimumInactiveTimingTcsh::_16XHclk)
    }
    #[doc = "24 x HCLK"]
    #[inline(always)]
    pub fn _24_x_hclk(self) -> &'a mut crate::W<REG> {
        self.variant(SelectCe0minimumInactiveTimingTcsh::_24XHclk)
    }
    #[doc = "32 x HCLK"]
    #[inline(always)]
    pub fn _32_x_hclk(self) -> &'a mut crate::W<REG> {
        self.variant(SelectCe0minimumInactiveTimingTcsh::_32XHclk)
    }
    #[doc = "40 x HCLK"]
    #[inline(always)]
    pub fn _40_x_hclk(self) -> &'a mut crate::W<REG> {
        self.variant(SelectCe0minimumInactiveTimingTcsh::_40XHclk)
    }
}
#[doc = "Field `SelectCE1MinimumInactiveTimingTCSH` reader - Select CE1 minimum inactive timing (tCSH)"]
pub type SelectCe1minimumInactiveTimingTcshR = crate::FieldReader;
#[doc = "Field `SelectCE1MinimumInactiveTimingTCSH` writer - Select CE1 minimum inactive timing (tCSH)"]
pub type SelectCe1minimumInactiveTimingTcshW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - CE0 SPI address mode selection"]
    #[inline(always)]
    pub fn ce0spiaddr_mode_sel(&self) -> Ce0spiaddrModeSelR {
        Ce0spiaddrModeSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CE1 SPI address mode selection"]
    #[inline(always)]
    pub fn ce1spiaddr_mode_sel(&self) -> Ce1spiaddrModeSelR {
        Ce1spiaddrModeSelR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - CE0 4B address Auto-Read command selection"]
    #[inline(always)]
    pub fn ce04baddr_auto_read_cmd_sel(&self) -> Ce04baddrAutoReadCmdSelR {
        Ce04baddrAutoReadCmdSelR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CE1 4B address Auto-Read command selection"]
    #[inline(always)]
    pub fn ce14baddr_auto_read_cmd_sel(&self) -> Ce14baddrAutoReadCmdSelR {
        Ce14baddrAutoReadCmdSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Select CE0 minimum inactive timing (tCSH)"]
    #[inline(always)]
    pub fn select_ce0minimum_inactive_timing_tcsh(&self) -> SelectCe0minimumInactiveTimingTcshR {
        SelectCe0minimumInactiveTimingTcshR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Select CE1 minimum inactive timing (tCSH)"]
    #[inline(always)]
    pub fn select_ce1minimum_inactive_timing_tcsh(&self) -> SelectCe1minimumInactiveTimingTcshR {
        SelectCe1minimumInactiveTimingTcshR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - CE0 SPI address mode selection"]
    #[inline(always)]
    pub fn ce0spiaddr_mode_sel(&mut self) -> Ce0spiaddrModeSelW<Fmc004Spec> {
        Ce0spiaddrModeSelW::new(self, 0)
    }
    #[doc = "Bit 1 - CE1 SPI address mode selection"]
    #[inline(always)]
    pub fn ce1spiaddr_mode_sel(&mut self) -> Ce1spiaddrModeSelW<Fmc004Spec> {
        Ce1spiaddrModeSelW::new(self, 1)
    }
    #[doc = "Bit 4 - CE0 4B address Auto-Read command selection"]
    #[inline(always)]
    pub fn ce04baddr_auto_read_cmd_sel(&mut self) -> Ce04baddrAutoReadCmdSelW<Fmc004Spec> {
        Ce04baddrAutoReadCmdSelW::new(self, 4)
    }
    #[doc = "Bit 5 - CE1 4B address Auto-Read command selection"]
    #[inline(always)]
    pub fn ce14baddr_auto_read_cmd_sel(&mut self) -> Ce14baddrAutoReadCmdSelW<Fmc004Spec> {
        Ce14baddrAutoReadCmdSelW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Select CE0 minimum inactive timing (tCSH)"]
    #[inline(always)]
    pub fn select_ce0minimum_inactive_timing_tcsh(
        &mut self,
    ) -> SelectCe0minimumInactiveTimingTcshW<Fmc004Spec> {
        SelectCe0minimumInactiveTimingTcshW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Select CE1 minimum inactive timing (tCSH)"]
    #[inline(always)]
    pub fn select_ce1minimum_inactive_timing_tcsh(
        &mut self,
    ) -> SelectCe1minimumInactiveTimingTcshW<Fmc004Spec> {
        SelectCe1minimumInactiveTimingTcshW::new(self, 10)
    }
}
#[doc = "CE Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc004Spec;
impl crate::RegisterSpec for Fmc004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc004::R`](R) reader structure"]
impl crate::Readable for Fmc004Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc004::W`](W) writer structure"]
impl crate::Writable for Fmc004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC004 to value 0x2a00"]
impl crate::Resettable for Fmc004Spec {
    const RESET_VALUE: u32 = 0x2a00;
}
