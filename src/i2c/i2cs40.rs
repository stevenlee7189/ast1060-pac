#[doc = "Register `I2CS40` reader"]
pub type R = crate::R<I2cs40Spec>;
#[doc = "Register `I2CS40` writer"]
pub type W = crate::W<I2cs40Spec>;
#[doc = "Field `SlaveDevAddr1` reader - Slave Device Address1"]
pub type SlaveDevAddr1R = crate::FieldReader;
#[doc = "Field `SlaveDevAddr1` writer - Slave Device Address1"]
pub type SlaveDevAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblSlaveDevAddr1OnlyForNewRegMode` reader - Enable Slave Device Address1 (only for New Register mode)"]
pub type EnblSlaveDevAddr1onlyForNewRegModeR = crate::BitReader;
#[doc = "Field `EnblSlaveDevAddr1OnlyForNewRegMode` writer - Enable Slave Device Address1 (only for New Register mode)"]
pub type EnblSlaveDevAddr1onlyForNewRegModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SlaveDevAddr2` reader - Slave Device Address2"]
pub type SlaveDevAddr2R = crate::FieldReader;
#[doc = "Field `SlaveDevAddr2` writer - Slave Device Address2"]
pub type SlaveDevAddr2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblSlaveDevAddr2` reader - Enable Slave Device Address2"]
pub type EnblSlaveDevAddr2R = crate::BitReader;
#[doc = "Field `EnblSlaveDevAddr2` writer - Enable Slave Device Address2"]
pub type EnblSlaveDevAddr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SlaveDevAddr3` reader - Slave Device Address3"]
pub type SlaveDevAddr3R = crate::FieldReader;
#[doc = "Field `SlaveDevAddr3` writer - Slave Device Address3"]
pub type SlaveDevAddr3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EnblSlaveDevAddr3` reader - Enable Slave Device Address3"]
pub type EnblSlaveDevAddr3R = crate::BitReader;
#[doc = "Field `EnblSlaveDevAddr3` writer - Enable Slave Device Address3"]
pub type EnblSlaveDevAddr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Slave Device Address1"]
    #[inline(always)]
    pub fn slave_dev_addr1(&self) -> SlaveDevAddr1R {
        SlaveDevAddr1R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Enable Slave Device Address1 (only for New Register mode)"]
    #[inline(always)]
    pub fn enbl_slave_dev_addr1only_for_new_reg_mode(&self) -> EnblSlaveDevAddr1onlyForNewRegModeR {
        EnblSlaveDevAddr1onlyForNewRegModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - Slave Device Address2"]
    #[inline(always)]
    pub fn slave_dev_addr2(&self) -> SlaveDevAddr2R {
        SlaveDevAddr2R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable Slave Device Address2"]
    #[inline(always)]
    pub fn enbl_slave_dev_addr2(&self) -> EnblSlaveDevAddr2R {
        EnblSlaveDevAddr2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Slave Device Address3"]
    #[inline(always)]
    pub fn slave_dev_addr3(&self) -> SlaveDevAddr3R {
        SlaveDevAddr3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 23 - Enable Slave Device Address3"]
    #[inline(always)]
    pub fn enbl_slave_dev_addr3(&self) -> EnblSlaveDevAddr3R {
        EnblSlaveDevAddr3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave Device Address1"]
    #[inline(always)]
    pub fn slave_dev_addr1(&mut self) -> SlaveDevAddr1W<I2cs40Spec> {
        SlaveDevAddr1W::new(self, 0)
    }
    #[doc = "Bit 7 - Enable Slave Device Address1 (only for New Register mode)"]
    #[inline(always)]
    pub fn enbl_slave_dev_addr1only_for_new_reg_mode(
        &mut self,
    ) -> EnblSlaveDevAddr1onlyForNewRegModeW<I2cs40Spec> {
        EnblSlaveDevAddr1onlyForNewRegModeW::new(self, 7)
    }
    #[doc = "Bits 8:14 - Slave Device Address2"]
    #[inline(always)]
    pub fn slave_dev_addr2(&mut self) -> SlaveDevAddr2W<I2cs40Spec> {
        SlaveDevAddr2W::new(self, 8)
    }
    #[doc = "Bit 15 - Enable Slave Device Address2"]
    #[inline(always)]
    pub fn enbl_slave_dev_addr2(&mut self) -> EnblSlaveDevAddr2W<I2cs40Spec> {
        EnblSlaveDevAddr2W::new(self, 15)
    }
    #[doc = "Bits 16:22 - Slave Device Address3"]
    #[inline(always)]
    pub fn slave_dev_addr3(&mut self) -> SlaveDevAddr3W<I2cs40Spec> {
        SlaveDevAddr3W::new(self, 16)
    }
    #[doc = "Bit 23 - Enable Slave Device Address3"]
    #[inline(always)]
    pub fn enbl_slave_dev_addr3(&mut self) -> EnblSlaveDevAddr3W<I2cs40Spec> {
        EnblSlaveDevAddr3W::new(self, 23)
    }
}
#[doc = "Slave Device Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs40Spec;
impl crate::RegisterSpec for I2cs40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs40::R`](R) reader structure"]
impl crate::Readable for I2cs40Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cs40::W`](W) writer structure"]
impl crate::Writable for I2cs40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS40 to value 0"]
impl crate::Resettable for I2cs40Spec {}
