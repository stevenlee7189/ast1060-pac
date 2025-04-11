#[doc = "Register `UDMA074` reader"]
pub type R = crate::R<Udma074Spec>;
#[doc = "Register `UDMA074` writer"]
pub type W = crate::W<Udma074Spec>;
#[doc = "Field `UART2RXWrPointer` reader - UART2 RX write pointer"]
pub type Uart2rxwrPointerR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - UART2 RX write pointer"]
    #[inline(always)]
    pub fn uart2rxwr_pointer(&self) -> Uart2rxwrPointerR {
        Uart2rxwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {}
#[doc = "UART2 RX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma074Spec;
impl crate::RegisterSpec for Udma074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma074::R`](R) reader structure"]
impl crate::Readable for Udma074Spec {}
#[doc = "`write(|w| ..)` method takes [`udma074::W`](W) writer structure"]
impl crate::Writable for Udma074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA074 to value 0"]
impl crate::Resettable for Udma074Spec {}
