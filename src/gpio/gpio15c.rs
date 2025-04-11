#[doc = "Register `GPIO15C` reader"]
pub type R = crate::R<Gpio15cSpec>;
#[doc = "Register `GPIO15C` writer"]
pub type W = crate::W<Gpio15cSpec>;
#[doc = "Field `PortGPIU70WDTRstToleranceEnbl` reader - Port GPIU\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiu70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortGPIU70WDTRstToleranceEnbl` writer - Port GPIU\\[7:0\\] WDT reset tolerance enable"]
pub type PortGpiu70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiu70wdtrst_tolerance_enbl(&self) -> PortGpiu70wdtrstToleranceEnblR {
        PortGpiu70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_gpiu70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortGpiu70wdtrstToleranceEnblW<Gpio15cSpec> {
        PortGpiu70wdtrstToleranceEnblW::new(self, 0)
    }
}
#[doc = "GPIO\\_U Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio15cSpec;
impl crate::RegisterSpec for Gpio15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio15c::R`](R) reader structure"]
impl crate::Readable for Gpio15cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio15c::W`](W) writer structure"]
impl crate::Writable for Gpio15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO15C to value 0"]
impl crate::Resettable for Gpio15cSpec {}
