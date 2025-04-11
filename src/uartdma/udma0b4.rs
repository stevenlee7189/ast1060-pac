#[doc = "Register `UDMA0B4` reader"]
pub type R = crate::R<Udma0b4Spec>;
#[doc = "Register `UDMA0B4` writer"]
pub type W = crate::W<Udma0b4Spec>;
#[doc = "Field `UART4RXWrPointer` reader - UART4 RX write pointer"]
pub type Uart4rxwrPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART4 RX write pointer"]
    #[inline(always)]
    pub fn uart4rxwr_pointer(&self) -> Uart4rxwrPointerR {
        Uart4rxwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART4 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0b4Spec;
impl crate::RegisterSpec for Udma0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0b4::R`](R) reader structure"]
impl crate::Readable for Udma0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0b4::W`](W) writer structure"]
impl crate::Writable for Udma0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0B4 to value 0"]
impl crate::Resettable for Udma0b4Spec {}
