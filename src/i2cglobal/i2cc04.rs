#[doc = "Register `I2CC04` reader"]
pub type R = crate::R<I2cc04Spec>;
#[doc = "Register `I2CC04` writer"]
pub type W = crate::W<I2cc04Spec>;
#[doc = "Field `SameAsI2CD04` reader - Same as hlinkI2CD04"]
pub type SameAsI2cd04R = crate::FieldReader<u32>;
#[doc = "Field `SameAsI2CD04` writer - Same as hlinkI2CD04"]
pub type SameAsI2cd04W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `CyclesOfMasterSCLClkhighMinimumPulseWidthTCKHighMin` reader - newverCycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
pub type CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinR = crate::FieldReader;
#[doc = "Field `CyclesOfMasterSCLClkhighMinimumPulseWidthTCKHighMin` writer - newverCycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
pub type CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinW<'a, REG> =
    crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SameAsI2CD08` reader - Same as hlinkI2CD08"]
pub type SameAsI2cd08R = crate::FieldReader;
#[doc = "Field `SameAsI2CD08` writer - Same as hlinkI2CD08"]
pub type SameAsI2cd08W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:19 - Same as hlinkI2CD04"]
    #[inline(always)]
    pub fn same_as_i2cd04(&self) -> SameAsI2cd04R {
        SameAsI2cd04R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:23 - newverCycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclkhigh_minimum_pulse_width_tckhigh_min(
        &self,
    ) -> CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinR {
        CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - Same as hlinkI2CD08"]
    #[inline(always)]
    pub fn same_as_i2cd08(&self) -> SameAsI2cd08R {
        SameAsI2cd08R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:19 - Same as hlinkI2CD04"]
    #[inline(always)]
    pub fn same_as_i2cd04(&mut self) -> SameAsI2cd04W<I2cc04Spec> {
        SameAsI2cd04W::new(self, 0)
    }
    #[doc = "Bits 20:23 - newverCycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclkhigh_minimum_pulse_width_tckhigh_min(
        &mut self,
    ) -> CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinW<I2cc04Spec> {
        CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinW::new(self, 20)
    }
    #[doc = "Bits 24:28 - Same as hlinkI2CD08"]
    #[inline(always)]
    pub fn same_as_i2cd08(&mut self) -> SameAsI2cd08W<I2cc04Spec> {
        SameAsI2cd08W::new(self, 24)
    }
}
#[doc = "Master/Slave Clock and AC Timing Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc04Spec;
impl crate::RegisterSpec for I2cc04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc04::R`](R) reader structure"]
impl crate::Readable for I2cc04Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cc04::W`](W) writer structure"]
impl crate::Writable for I2cc04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC04 to value 0"]
impl crate::Resettable for I2cc04Spec {}
