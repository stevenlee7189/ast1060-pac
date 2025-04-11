#[doc = "Register `UDMA1A0` reader"]
pub type R = crate::R<Udma1a0Spec>;
#[doc = "Register `UDMA1A0` writer"]
pub type W = crate::W<Udma1a0Spec>;
#[doc = "Field `UART12TXReadPointer` reader - UART12 TX read pointer"]
pub type Uart12txreadPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART12 TX read pointer"]
    #[inline(always)]
    pub fn uart12txread_pointer(&self) -> Uart12txreadPointerR {
        Uart12txreadPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART12 TX read pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma1a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma1a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma1a0Spec;
impl crate::RegisterSpec for Udma1a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma1a0::R`](R) reader structure"]
impl crate::Readable for Udma1a0Spec {}
#[doc = "`write(|w| ..)` method takes [`udma1a0::W`](W) writer structure"]
impl crate::Writable for Udma1a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA1A0 to value 0"]
impl crate::Resettable for Udma1a0Spec {}
