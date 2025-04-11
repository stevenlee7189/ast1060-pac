#[doc = "Register `GPIO0FC` reader"]
pub type R = crate::R<Gpio0fcSpec>;
#[doc = "Register `GPIO0FC` writer"]
pub type W = crate::W<Gpio0fcSpec>;
#[doc = "Field `PortGPIOM70WDTRstToleranceEnbl` reader - Port GPIOM\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiom70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOM70WDTRstToleranceEnbl` writer - Port GPIOM\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiom70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70WDTRstToleranceEnbl` reader - Port GPION\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpion70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPION70WDTRstToleranceEnbl` writer - Port GPION\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpion70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70WDTRstToleranceEnbl` reader - Port GPIOO\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioo70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOO70WDTRstToleranceEnbl` writer - Port GPIOO\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioo70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70WDTRstToleranceEnbl` reader - Port GPIOP\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiop70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOP70WDTRstToleranceEnbl` writer - Port GPIOP\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiop70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiom70wdtrst_tolerance_enbl(&self) -> PortGpiom70wdtrstToleranceEnblR {
        PortGpiom70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpion70wdtrst_tolerance_enbl(&self) -> PortGpion70wdtrstToleranceEnblR {
        PortGpion70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioo70wdtrst_tolerance_enbl(&self) -> PortGpioo70wdtrstToleranceEnblR {
        PortGpioo70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiop70wdtrst_tolerance_enbl(&self) -> PortGpiop70wdtrstToleranceEnblR {
        PortGpiop70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiom70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiom70wdtrstToleranceEnblW<Gpio0fcSpec> {
        PortGpiom70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpion70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpion70wdtrstToleranceEnblW<Gpio0fcSpec> {
        PortGpion70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioo70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioo70wdtrstToleranceEnblW<Gpio0fcSpec> {
        PortGpioo70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiop70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiop70wdtrstToleranceEnblW<Gpio0fcSpec> {
        PortGpiop70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0fcSpec;
impl crate::RegisterSpec for Gpio0fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0fc::R`](R) reader structure"]
impl crate::Readable for Gpio0fcSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio0fc::W`](W) writer structure"]
impl crate::Writable for Gpio0fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0FC to value 0"]
impl crate::Resettable for Gpio0fcSpec {}
