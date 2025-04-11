#[doc = "Register `GPIO12C` reader"]
pub type R = crate::R<Gpio12cSpec>;
#[doc = "Register `GPIO12C` writer"]
pub type W = crate::W<Gpio12cSpec>;
#[doc = "Field `PortGPIOQ70WDTRstToleranceEnbl` reader - Port GPIOQ\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioq70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70WDTRstToleranceEnbl` writer - Port GPIOQ\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpioq70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70WDTRstToleranceEnbl` reader - Port GPIOR\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpior70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOR70WDTRstToleranceEnbl` writer - Port GPIOR\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpior70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70WDTRstToleranceEnbl` reader - Port GPIOS\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpios70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIOS70WDTRstToleranceEnbl` writer - Port GPIOS\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpios70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70WDTRstToleranceEnbl` reader - Port GPIT\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpit70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIT70WDTRstToleranceEnbl` writer - Port GPIT\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpit70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioq70wdtrst_tolerance_enbl(&self) -> PortGpioq70wdtrstToleranceEnblR {
        PortGpioq70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpior70wdtrst_tolerance_enbl(&self) -> PortGpior70wdtrstToleranceEnblR {
        PortGpior70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpios70wdtrst_tolerance_enbl(&self) -> PortGpios70wdtrstToleranceEnblR {
        PortGpios70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpit70wdtrst_tolerance_enbl(&self) -> PortGpit70wdtrstToleranceEnblR {
        PortGpit70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpioq70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpioq70wdtrstToleranceEnblW<Gpio12cSpec> {
        PortGpioq70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpior70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpior70wdtrstToleranceEnblW<Gpio12cSpec> {
        PortGpior70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpios70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpios70wdtrstToleranceEnblW<Gpio12cSpec> {
        PortGpios70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpit70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpit70wdtrstToleranceEnblW<Gpio12cSpec> {
        PortGpit70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio12c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio12c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio12cSpec;
impl crate::RegisterSpec for Gpio12cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio12c::R`](R) reader structure"]
impl crate::Readable for Gpio12cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio12c::W`](W) writer structure"]
impl crate::Writable for Gpio12cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO12C to value 0"]
impl crate::Resettable for Gpio12cSpec {}
