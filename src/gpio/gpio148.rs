#[doc = "Register `GPIO148` reader"]
pub type R = crate::R<Gpio148Spec>;
#[doc = "Register `GPIO148` writer"]
pub type W = crate::W<Gpio148Spec>;
#[doc = "Field `PortGPIU70INTEnbl` reader - Port GPIU\\[7:0\\] interrupt enable"]
pub type PortGpiu70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIU70INTEnbl` writer - Port GPIU\\[7:0\\] interrupt enable"]
pub type PortGpiu70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiu70intenbl(&self) -> PortGpiu70intenblR {
        PortGpiu70intenblR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpiu70intenbl(&mut self) -> PortGpiu70intenblW<Gpio148Spec> {
        PortGpiu70intenblW::new(self, 0)
    }
}
#[doc = "GPIO\\_U Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio148Spec;
impl crate::RegisterSpec for Gpio148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio148::R`](R) reader structure"]
impl crate::Readable for Gpio148Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio148::W`](W) writer structure"]
impl crate::Writable for Gpio148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO148 to value 0"]
impl crate::Resettable for Gpio148Spec {}
