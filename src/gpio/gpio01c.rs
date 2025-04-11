#[doc = "Register `GPIO01C` reader"]
pub type R = crate::R<Gpio01cSpec>;
#[doc = "Register `GPIO01C` writer"]
pub type W = crate::W<Gpio01cSpec>;
#[doc = "Field `PortGPIOA70WDTRstToleranceEnbl` reader - Port GPIOA\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioa70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOA70WDTRstToleranceEnbl` writer - Port GPIOA\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioa70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70WDTRstToleranceEnbl` reader - Port GPIOB\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiob70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOB70WDTRstToleranceEnbl` writer - Port GPIOB\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiob70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70WDTRstToleranceEnbl` reader - Port GPIOC\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioc70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOC70WDTRstToleranceEnbl` writer - Port GPIOC\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioc70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70WDTRstToleranceEnbl` reader - Port GPIOD\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiod70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOD70WDTRstToleranceEnbl` writer - Port GPIOD\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiod70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioa70wdtrst_tolerance_enbl(&self) -> PortGpioa70wdtrstToleranceEnblR {
        PortGpioa70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiob70wdtrst_tolerance_enbl(&self) -> PortGpiob70wdtrstToleranceEnblR {
        PortGpiob70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioc70wdtrst_tolerance_enbl(&self) -> PortGpioc70wdtrstToleranceEnblR {
        PortGpioc70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiod70wdtrst_tolerance_enbl(&self) -> PortGpiod70wdtrstToleranceEnblR {
        PortGpiod70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioa70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioa70wdtrstToleranceEnblW<Gpio01cSpec> {
        PortGpioa70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiob70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiob70wdtrstToleranceEnblW<Gpio01cSpec> {
        PortGpiob70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioc70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioc70wdtrstToleranceEnblW<Gpio01cSpec> {
        PortGpioc70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiod70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiod70wdtrstToleranceEnblW<Gpio01cSpec> {
        PortGpiod70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio01c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio01c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio01cSpec;
impl crate::RegisterSpec for Gpio01cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio01c::R`](R) reader structure"]
impl crate::Readable for Gpio01cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio01c::W`](W) writer structure"]
impl crate::Writable for Gpio01cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO01C to value 0"]
impl crate::Resettable for Gpio01cSpec {}
