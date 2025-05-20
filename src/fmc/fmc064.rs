#[doc = "Register `FMC064` reader"]
pub type R = crate::R<Fmc064Spec>;
#[doc = "Register `FMC064` writer"]
pub type W = crate::W<Fmc064Spec>;
#[doc = "Field `EnblWatchdog` reader - Enable watchdog"]
pub type EnblWatchdogR = crate::BitReader;
#[doc = "Field `EnblWatchdog` writer - Enable watchdog"]
pub type EnblWatchdogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Boot flash source select indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BootFlashSrcSelectIndicator {
    #[doc = "0: boot from primary source"]
    BootFromPrimarySource = 0,
    #[doc = "1: boot from alternate source"]
    BootFromAlternateSource = 1,
}
impl From<BootFlashSrcSelectIndicator> for bool {
    #[inline(always)]
    fn from(variant: BootFlashSrcSelectIndicator) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BootFlashSrcSelectIndicator` reader - Boot flash source select indicator"]
pub type BootFlashSrcSelectIndicatorR = crate::BitReader<BootFlashSrcSelectIndicator>;
impl BootFlashSrcSelectIndicatorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BootFlashSrcSelectIndicator {
        match self.bits {
            false => BootFlashSrcSelectIndicator::BootFromPrimarySource,
            true => BootFlashSrcSelectIndicator::BootFromAlternateSource,
        }
    }
    #[doc = "boot from primary source"]
    #[inline(always)]
    pub fn is_boot_from_primary_source(&self) -> bool {
        *self == BootFlashSrcSelectIndicator::BootFromPrimarySource
    }
    #[doc = "boot from alternate source"]
    #[inline(always)]
    pub fn is_boot_from_alternate_source(&self) -> bool {
        *self == BootFlashSrcSelectIndicator::BootFromAlternateSource
    }
}
#[doc = "Field `BootFlashSrcSelectIndicator` writer - Boot flash source select indicator"]
pub type BootFlashSrcSelectIndicatorW<'a, REG> =
    crate::BitWriter<'a, REG, BootFlashSrcSelectIndicator>;
impl<'a, REG> BootFlashSrcSelectIndicatorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "boot from primary source"]
    #[inline(always)]
    pub fn boot_from_primary_source(self) -> &'a mut crate::W<REG> {
        self.variant(BootFlashSrcSelectIndicator::BootFromPrimarySource)
    }
    #[doc = "boot from alternate source"]
    #[inline(always)]
    pub fn boot_from_alternate_source(self) -> &'a mut crate::W<REG> {
        self.variant(BootFlashSrcSelectIndicator::BootFromAlternateSource)
    }
}
#[doc = "Field `SingleChipBootModeSrcSelectIndicator` reader - Single chip boot mode source select indicator"]
pub type SingleChipBootModeSrcSelectIndicatorR = crate::BitReader;
#[doc = "Alternate Boot Mode that defined by OTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlternateBootModeThatDefinedByOtp {
    #[doc = "0: 2 chips mode"]
    _2ChipsMode = 0,
    #[doc = "1: 1 chip mode"]
    _1ChipMode = 1,
}
impl From<AlternateBootModeThatDefinedByOtp> for bool {
    #[inline(always)]
    fn from(variant: AlternateBootModeThatDefinedByOtp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AlternateBootModeThatDefinedByOTP` reader - Alternate Boot Mode that defined by OTP"]
pub type AlternateBootModeThatDefinedByOtpR = crate::BitReader<AlternateBootModeThatDefinedByOtp>;
impl AlternateBootModeThatDefinedByOtpR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlternateBootModeThatDefinedByOtp {
        match self.bits {
            false => AlternateBootModeThatDefinedByOtp::_2ChipsMode,
            true => AlternateBootModeThatDefinedByOtp::_1ChipMode,
        }
    }
    #[doc = "2 chips mode"]
    #[inline(always)]
    pub fn is_2_chips_mode(&self) -> bool {
        *self == AlternateBootModeThatDefinedByOtp::_2ChipsMode
    }
    #[doc = "1 chip mode"]
    #[inline(always)]
    pub fn is_1_chip_mode(&self) -> bool {
        *self == AlternateBootModeThatDefinedByOtp::_1ChipMode
    }
}
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Field `WDActiveEventCounter` reader - Watchdog active event counter"]
pub type WdactiveEventCounterR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable watchdog"]
    #[inline(always)]
    pub fn enbl_watchdog(&self) -> EnblWatchdogR {
        EnblWatchdogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Boot flash source select indicator"]
    #[inline(always)]
    pub fn boot_flash_src_select_indicator(&self) -> BootFlashSrcSelectIndicatorR {
        BootFlashSrcSelectIndicatorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Single chip boot mode source select indicator"]
    #[inline(always)]
    pub fn single_chip_boot_mode_src_select_indicator(
        &self,
    ) -> SingleChipBootModeSrcSelectIndicatorR {
        SingleChipBootModeSrcSelectIndicatorR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alternate Boot Mode that defined by OTP"]
    #[inline(always)]
    pub fn alternate_boot_mode_that_defined_by_otp(&self) -> AlternateBootModeThatDefinedByOtpR {
        AlternateBootModeThatDefinedByOtpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog active event counter"]
    #[inline(always)]
    pub fn wdactive_event_counter(&self) -> WdactiveEventCounterR {
        WdactiveEventCounterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable watchdog"]
    #[inline(always)]
    pub fn enbl_watchdog(&mut self) -> EnblWatchdogW<Fmc064Spec> {
        EnblWatchdogW::new(self, 0)
    }
    #[doc = "Bit 4 - Boot flash source select indicator"]
    #[inline(always)]
    pub fn boot_flash_src_select_indicator(&mut self) -> BootFlashSrcSelectIndicatorW<Fmc064Spec> {
        BootFlashSrcSelectIndicatorW::new(self, 4)
    }
}
#[doc = "FMC\\_WDT2 Control/Status Register for Alternate Boot\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc064Spec;
impl crate::RegisterSpec for Fmc064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc064::R`](R) reader structure"]
impl crate::Readable for Fmc064Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc064::W`](W) writer structure"]
impl crate::Writable for Fmc064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC064 to value 0"]
impl crate::Resettable for Fmc064Spec {}
