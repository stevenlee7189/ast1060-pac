#[doc = "Register `GPIO168` reader"]
pub type R = crate::R<Gpio168Spec>;
#[doc = "Register `GPIO168` writer"]
pub type W = crate::W<Gpio168Spec>;
#[doc = "Field `PortGPIU70InputMask` reader - Port GPIU\\[7:0\\] input mask"]
pub type PortGpiu70inputMaskR = crate::FieldReader;
#[doc = "Field `PortGPIU70InputMask` writer - Port GPIU\\[7:0\\] input mask"]
pub type PortGpiu70inputMaskW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiu70input_mask(&self) -> PortGpiu70inputMaskR {
        PortGpiu70inputMaskR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIU\\[7:0\\] input mask"]
    #[inline(always)]
    pub fn port_gpiu70input_mask(&mut self) -> PortGpiu70inputMaskW<Gpio168Spec> {
        PortGpiu70inputMaskW::new(self, 0)
    }
}
#[doc = "GPIO\\_U Input Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio168::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio168::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio168Spec;
impl crate::RegisterSpec for Gpio168Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio168::R`](R) reader structure"]
impl crate::Readable for Gpio168Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio168::W`](W) writer structure"]
impl crate::Writable for Gpio168Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO168 to value 0"]
impl crate::Resettable for Gpio168Spec {}
