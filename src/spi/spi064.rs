#[doc = "Register `SPI064` reader"]
pub type R = crate::R<Spi064Spec>;
#[doc = "Register `SPI064` writer"]
pub type W = crate::W<Spi064Spec>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
#[doc = "SPI size of boot flash\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SpisizeOfBootFlash {
    #[doc = "0: no define"]
    NoDefine = 0,
    #[doc = "1: 2 MB"]
    _2Mb = 1,
    #[doc = "2: 4 MB"]
    _4Mb = 2,
    #[doc = "3: 8 MB"]
    _8Mb = 3,
    #[doc = "4: 16 MB"]
    _16Mb = 4,
    #[doc = "5: 32 MB"]
    _32Mb = 5,
    #[doc = "6: 64 MB"]
    _64Mb = 6,
    #[doc = "7: 128 MB"]
    _128Mb = 7,
}
impl From<SpisizeOfBootFlash> for u8 {
    #[inline(always)]
    fn from(variant: SpisizeOfBootFlash) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SpisizeOfBootFlash {
    type Ux = u8;
}
impl crate::IsEnum for SpisizeOfBootFlash {}
#[doc = "Field `SPISizeOfBootFlash` reader - SPI size of boot flash"]
pub type SpisizeOfBootFlashR = crate::FieldReader<SpisizeOfBootFlash>;
impl SpisizeOfBootFlashR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SpisizeOfBootFlash {
        match self.bits {
            0 => SpisizeOfBootFlash::NoDefine,
            1 => SpisizeOfBootFlash::_2Mb,
            2 => SpisizeOfBootFlash::_4Mb,
            3 => SpisizeOfBootFlash::_8Mb,
            4 => SpisizeOfBootFlash::_16Mb,
            5 => SpisizeOfBootFlash::_32Mb,
            6 => SpisizeOfBootFlash::_64Mb,
            7 => SpisizeOfBootFlash::_128Mb,
            _ => unreachable!(),
        }
    }
    #[doc = "no define"]
    #[inline(always)]
    pub fn is_no_define(&self) -> bool {
        *self == SpisizeOfBootFlash::NoDefine
    }
    #[doc = "2 MB"]
    #[inline(always)]
    pub fn is_2_mb(&self) -> bool {
        *self == SpisizeOfBootFlash::_2Mb
    }
    #[doc = "4 MB"]
    #[inline(always)]
    pub fn is_4_mb(&self) -> bool {
        *self == SpisizeOfBootFlash::_4Mb
    }
    #[doc = "8 MB"]
    #[inline(always)]
    pub fn is_8_mb(&self) -> bool {
        *self == SpisizeOfBootFlash::_8Mb
    }
    #[doc = "16 MB"]
    #[inline(always)]
    pub fn is_16_mb(&self) -> bool {
        *self == SpisizeOfBootFlash::_16Mb
    }
    #[doc = "32 MB"]
    #[inline(always)]
    pub fn is_32_mb(&self) -> bool {
        *self == SpisizeOfBootFlash::_32Mb
    }
    #[doc = "64 MB"]
    #[inline(always)]
    pub fn is_64_mb(&self) -> bool {
        *self == SpisizeOfBootFlash::_64Mb
    }
    #[doc = "128 MB"]
    #[inline(always)]
    pub fn is_128_mb(&self) -> bool {
        *self == SpisizeOfBootFlash::_128Mb
    }
}
#[doc = "Field `SPISizeOfBootFlash` writer - SPI size of boot flash"]
pub type SpisizeOfBootFlashW<'a, REG> =
    crate::FieldWriter<'a, REG, 3, SpisizeOfBootFlash, crate::Safe>;
impl<'a, REG> SpisizeOfBootFlashW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "no define"]
    #[inline(always)]
    pub fn no_define(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::NoDefine)
    }
    #[doc = "2 MB"]
    #[inline(always)]
    pub fn _2_mb(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::_2Mb)
    }
    #[doc = "4 MB"]
    #[inline(always)]
    pub fn _4_mb(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::_4Mb)
    }
    #[doc = "8 MB"]
    #[inline(always)]
    pub fn _8_mb(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::_8Mb)
    }
    #[doc = "16 MB"]
    #[inline(always)]
    pub fn _16_mb(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::_16Mb)
    }
    #[doc = "32 MB"]
    #[inline(always)]
    pub fn _32_mb(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::_32Mb)
    }
    #[doc = "64 MB"]
    #[inline(always)]
    pub fn _64_mb(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::_64Mb)
    }
    #[doc = "128 MB"]
    #[inline(always)]
    pub fn _128_mb(self) -> &'a mut crate::W<REG> {
        self.variant(SpisizeOfBootFlash::_128Mb)
    }
}
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
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::BitReader;
#[doc = "Alternate Boot Mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AlternateBootModeSel {
    #[doc = "0: 2 chips mode"]
    _2ChipsMode = 0,
    #[doc = "1: 1 chip mode"]
    _1ChipMode = 1,
}
impl From<AlternateBootModeSel> for bool {
    #[inline(always)]
    fn from(variant: AlternateBootModeSel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AlternateBootModeSel` reader - Alternate Boot Mode selection"]
pub type AlternateBootModeSelR = crate::BitReader<AlternateBootModeSel>;
impl AlternateBootModeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AlternateBootModeSel {
        match self.bits {
            false => AlternateBootModeSel::_2ChipsMode,
            true => AlternateBootModeSel::_1ChipMode,
        }
    }
    #[doc = "2 chips mode"]
    #[inline(always)]
    pub fn is_2_chips_mode(&self) -> bool {
        *self == AlternateBootModeSel::_2ChipsMode
    }
    #[doc = "1 chip mode"]
    #[inline(always)]
    pub fn is_1_chip_mode(&self) -> bool {
        *self == AlternateBootModeSel::_1ChipMode
    }
}
#[doc = "Field `AlternateBootModeSel` writer - Alternate Boot Mode selection"]
pub type AlternateBootModeSelW<'a, REG> = crate::BitWriter<'a, REG, AlternateBootModeSel>;
impl<'a, REG> AlternateBootModeSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "2 chips mode"]
    #[inline(always)]
    pub fn _2_chips_mode(self) -> &'a mut crate::W<REG> {
        self.variant(AlternateBootModeSel::_2ChipsMode)
    }
    #[doc = "1 chip mode"]
    #[inline(always)]
    pub fn _1_chip_mode(self) -> &'a mut crate::W<REG> {
        self.variant(AlternateBootModeSel::_1ChipMode)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - SPI size of boot flash"]
    #[inline(always)]
    pub fn spisize_of_boot_flash(&self) -> SpisizeOfBootFlashR {
        SpisizeOfBootFlashR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Boot flash source select indicator"]
    #[inline(always)]
    pub fn boot_flash_src_select_indicator(&self) -> BootFlashSrcSelectIndicatorR {
        BootFlashSrcSelectIndicatorR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Alternate Boot Mode selection"]
    #[inline(always)]
    pub fn alternate_boot_mode_sel(&self) -> AlternateBootModeSelR {
        AlternateBootModeSelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 1:3 - SPI size of boot flash"]
    #[inline(always)]
    pub fn spisize_of_boot_flash(&mut self) -> SpisizeOfBootFlashW<Spi064Spec> {
        SpisizeOfBootFlashW::new(self, 1)
    }
    #[doc = "Bit 4 - Boot flash source select indicator"]
    #[inline(always)]
    pub fn boot_flash_src_select_indicator(&mut self) -> BootFlashSrcSelectIndicatorW<Spi064Spec> {
        BootFlashSrcSelectIndicatorW::new(self, 4)
    }
    #[doc = "Bit 6 - Alternate Boot Mode selection"]
    #[inline(always)]
    pub fn alternate_boot_mode_sel(&mut self) -> AlternateBootModeSelW<Spi064Spec> {
        AlternateBootModeSelW::new(self, 6)
    }
}
#[doc = "Alternate Boot Control/Status\n\nYou can [`read`](crate::Reg::read) this register and get [`spi064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi064Spec;
impl crate::RegisterSpec for Spi064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi064::R`](R) reader structure"]
impl crate::Readable for Spi064Spec {}
#[doc = "`write(|w| ..)` method takes [`spi064::W`](W) writer structure"]
impl crate::Writable for Spi064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI064 to value 0"]
impl crate::Resettable for Spi064Spec {}
