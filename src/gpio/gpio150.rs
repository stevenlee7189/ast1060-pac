#[doc = "Register `GPIO150` reader"]
pub type R = crate::R<Gpio150Spec>;
#[doc = "Register `GPIO150` writer"]
pub type W = crate::W<Gpio150Spec>;
#[doc = "Field `PortGPIU70INTSensitivityType1Sel` reader - Port GPIU\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiu70intsensitivityType1selR = crate::FieldReader;
#[doc = "Field `PortGPIU70INTSensitivityType1Sel` writer - Port GPIU\\[7:0\\] interrupt sensitivity type 1 selection"]
pub type PortGpiu70intsensitivityType1selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiu70intsensitivity_type1sel(&self) -> PortGpiu70intsensitivityType1selR {
        PortGpiu70intsensitivityType1selR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt sensitivity type 1 selection"]
    #[inline(always)]
    pub fn port_gpiu70intsensitivity_type1sel(
        &mut self,
    ) -> PortGpiu70intsensitivityType1selW<Gpio150Spec> {
        PortGpiu70intsensitivityType1selW::new(self, 0)
    }
}
#[doc = "GPIO\\_U Interrupt Sensitivity Type 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio150::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio150::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio150Spec;
impl crate::RegisterSpec for Gpio150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio150::R`](R) reader structure"]
impl crate::Readable for Gpio150Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio150::W`](W) writer structure"]
impl crate::Writable for Gpio150Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO150 to value 0"]
impl crate::Resettable for Gpio150Spec {}
