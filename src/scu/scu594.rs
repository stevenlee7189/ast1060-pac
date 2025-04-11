#[doc = "Register `SCU594` reader"]
pub type R = crate::R<Scu594Spec>;
#[doc = "Register `SCU594` writer"]
pub type W = crate::W<Scu594Spec>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `ChipID` reader - Chip ID"]
pub type ChipIdR = crate::FieldReader;
#[doc = "Field `Reserved8` reader - Reserved (1)"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `DisADC` reader - Disable ADC"]
pub type DisAdcR = crate::BitReader;
#[doc = "Field `DisSecBoot` reader - Disable Secure Boot"]
pub type DisSecBootR = crate::BitReader;
#[doc = "Field `Reserved7` reader - Reserved (1)"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `DisI3C` reader - Disable I3C"]
pub type DisI3cR = crate::BitReader;
#[doc = "Field `Reserved6` reader - Reserved (1)"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `DisAES` reader - Disable AES"]
pub type DisAesR = crate::BitReader;
#[doc = "Field `Reserved5` reader - Reserved (1)"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `Reserved4` reader - Reserved (1)"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved3` reader - Reserved (1)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `DisI2C` reader - Disable I2C"]
pub type DisI2cR = crate::BitReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved (1)"]
pub type Reserved1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Chip ID"]
    #[inline(always)]
    pub fn chip_id(&self) -> ChipIdR {
        ChipIdR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable ADC"]
    #[inline(always)]
    pub fn dis_adc(&self) -> DisAdcR {
        DisAdcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable Secure Boot"]
    #[inline(always)]
    pub fn dis_sec_boot(&self) -> DisSecBootR {
        DisSecBootR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable I3C"]
    #[inline(always)]
    pub fn dis_i3c(&self) -> DisI3cR {
        DisI3cR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable AES"]
    #[inline(always)]
    pub fn dis_aes(&self) -> DisAesR {
        DisAesR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disable I2C"]
    #[inline(always)]
    pub fn dis_i2c(&self) -> DisI2cR {
        DisI2cR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved (1)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {}
#[doc = "EFUSE Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu594::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu594::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu594Spec;
impl crate::RegisterSpec for Scu594Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu594::R`](R) reader structure"]
impl crate::Readable for Scu594Spec {}
#[doc = "`write(|w| ..)` method takes [`scu594::W`](W) writer structure"]
impl crate::Writable for Scu594Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU594 to value 0"]
impl crate::Resettable for Scu594Spec {}
