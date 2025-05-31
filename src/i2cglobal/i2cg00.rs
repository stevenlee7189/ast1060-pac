#[doc = "Register `I2CG00` reader"]
pub type R = crate::R<I2cg00Spec>;
#[doc = "Register `I2CG00` writer"]
pub type W = crate::W<I2cg00Spec>;
#[doc = "Field `I2CSMBusDev1INT` reader - I2C/SMBus Device #1 Interrupt"]
pub type I2csmbusDev1intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev2INT` reader - I2C/SMBus Device #2 Interrupt"]
pub type I2csmbusDev2intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev3INT` reader - I2C/SMBus Device #3 Interrupt"]
pub type I2csmbusDev3intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev4INT` reader - I2C/SMBus Device #4 Interrupt"]
pub type I2csmbusDev4intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev5INT` reader - I2C/SMBus Device #5 Interrupt"]
pub type I2csmbusDev5intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev6INT` reader - I2C/SMBus Device #6 Interrupt"]
pub type I2csmbusDev6intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev7INT` reader - I2C/SMBus Device #7 Interrupt"]
pub type I2csmbusDev7intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev8INT` reader - I2C/SMBus Device #8 Interrupt"]
pub type I2csmbusDev8intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev9INT` reader - I2C/SMBus Device #9 Interrupt"]
pub type I2csmbusDev9intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev10INT` reader - I2C/SMBus Device #10 Interrupt"]
pub type I2csmbusDev10intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev11INT` reader - I2C/SMBus Device #11 Interrupt"]
pub type I2csmbusDev11intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev12INT` reader - I2C/SMBus Device #12 Interrupt"]
pub type I2csmbusDev12intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev13INT` reader - I2C/SMBus Device #13 Interrupt"]
pub type I2csmbusDev13intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev14INT` reader - I2C/SMBus Device #14 Interrupt"]
pub type I2csmbusDev14intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev15INT` reader - newverI2C/SMBus Device #15 Interrupt"]
pub type I2csmbusDev15intR = crate::BitReader;
#[doc = "Field `I2CSMBusDev16INT` reader - newverI2C/SMBus Device #16 Interrupt"]
pub type I2csmbusDev16intR = crate::BitReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - I2C/SMBus Device #1 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev1int(&self) -> I2csmbusDev1intR {
        I2csmbusDev1intR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C/SMBus Device #2 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev2int(&self) -> I2csmbusDev2intR {
        I2csmbusDev2intR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C/SMBus Device #3 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev3int(&self) -> I2csmbusDev3intR {
        I2csmbusDev3intR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C/SMBus Device #4 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev4int(&self) -> I2csmbusDev4intR {
        I2csmbusDev4intR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I2C/SMBus Device #5 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev5int(&self) -> I2csmbusDev5intR {
        I2csmbusDev5intR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I2C/SMBus Device #6 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev6int(&self) -> I2csmbusDev6intR {
        I2csmbusDev6intR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C/SMBus Device #7 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev7int(&self) -> I2csmbusDev7intR {
        I2csmbusDev7intR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C/SMBus Device #8 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev8int(&self) -> I2csmbusDev8intR {
        I2csmbusDev8intR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - I2C/SMBus Device #9 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev9int(&self) -> I2csmbusDev9intR {
        I2csmbusDev9intR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - I2C/SMBus Device #10 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev10int(&self) -> I2csmbusDev10intR {
        I2csmbusDev10intR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - I2C/SMBus Device #11 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev11int(&self) -> I2csmbusDev11intR {
        I2csmbusDev11intR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2C/SMBus Device #12 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev12int(&self) -> I2csmbusDev12intR {
        I2csmbusDev12intR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - I2C/SMBus Device #13 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev13int(&self) -> I2csmbusDev13intR {
        I2csmbusDev13intR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - I2C/SMBus Device #14 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev14int(&self) -> I2csmbusDev14intR {
        I2csmbusDev14intR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - newverI2C/SMBus Device #15 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev15int(&self) -> I2csmbusDev15intR {
        I2csmbusDev15intR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - newverI2C/SMBus Device #16 Interrupt"]
    #[inline(always)]
    pub fn i2csmbus_dev16int(&self) -> I2csmbusDev16intR {
        I2csmbusDev16intR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "Device Master Mode Interrupt Status Register (I2CG0C\\[3\\] = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cg00Spec;
impl crate::RegisterSpec for I2cg00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cg00::R`](R) reader structure"]
impl crate::Readable for I2cg00Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cg00::W`](W) writer structure"]
impl crate::Writable for I2cg00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CG00 to value 0"]
impl crate::Resettable for I2cg00Spec {}
