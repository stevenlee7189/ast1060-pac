#[doc = "Register `GPIO158` reader"]
pub type R = crate::R<Gpio158Spec>;
#[doc = "Register `GPIO158` writer"]
pub type W = crate::W<Gpio158Spec>;
#[doc = "Field `PortGPIU70INTStsReg` reader - Port GPIU\\[7:0\\] interrupt status register"]
pub type PortGpiu70intstsRegR = crate::FieldReader;
#[doc = "Field `PortGPIU70INTStsReg` writer - Port GPIU\\[7:0\\] interrupt status register"]
pub type PortGpiu70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiu70intsts_reg(&self) -> PortGpiu70intstsRegR {
        PortGpiu70intstsRegR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_gpiu70intsts_reg(&mut self) -> PortGpiu70intstsRegW<Gpio158Spec> {
        PortGpiu70intstsRegW::new(self, 0)
    }
}
#[doc = "GPIO\\_U Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio158::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio158::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio158Spec;
impl crate::RegisterSpec for Gpio158Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio158::R`](R) reader structure"]
impl crate::Readable for Gpio158Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio158::W`](W) writer structure"]
impl crate::Writable for Gpio158Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO158 to value 0"]
impl crate::Resettable for Gpio158Spec {}
