#[doc = "Register `GPIO154` reader"]
pub type R = crate::R<Gpio154Spec>;
#[doc = "Register `GPIO154` writer"]
pub type W = crate::W<Gpio154Spec>;
#[doc = "Field `PortGPIU70INTSensitivityType2Sel` reader - Port GPIU\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiu70intsensitivityType2selR = crate::FieldReader;
#[doc = "Field `PortGPIU70INTSensitivityType2Sel` writer - Port GPIU\\[7:0\\] interrupt sensitivity type 2 selection"]
pub type PortGpiu70intsensitivityType2selW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiu70intsensitivity_type2sel(&self) -> PortGpiu70intsensitivityType2selR {
        PortGpiu70intsensitivityType2selR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt sensitivity type 2 selection"]
    #[inline(always)]
    pub fn port_gpiu70intsensitivity_type2sel(
        &mut self,
    ) -> PortGpiu70intsensitivityType2selW<Gpio154Spec> {
        PortGpiu70intsensitivityType2selW::new(self, 0)
    }
}
#[doc = "GPIO\\_U Interrupt Sensitivity Type 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio154::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio154::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio154Spec;
impl crate::RegisterSpec for Gpio154Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio154::R`](R) reader structure"]
impl crate::Readable for Gpio154Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio154::W`](W) writer structure"]
impl crate::Writable for Gpio154Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO154 to value 0"]
impl crate::Resettable for Gpio154Spec {}
