#[doc = "Register `UDMA084` reader"]
pub type R = crate::R<Udma084Spec>;
#[doc = "Register `UDMA084` writer"]
pub type W = crate::W<Udma084Spec>;
#[doc = "Field `UART3TXWrPointer` reader - UART3 TX write pointer"]
pub type Uart3txwrPointerR = crate::FieldReader<u16>;
#[doc = "Field `UART3TXWrPointer` writer - UART3 TX write pointer"]
pub type Uart3txwrPointerW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART3 TX write pointer"]
    #[inline(always)]
    pub fn uart3txwr_pointer(&self) -> Uart3txwrPointerR {
        Uart3txwrPointerR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART3 TX write pointer"]
    #[inline(always)]
    pub fn uart3txwr_pointer(&mut self) -> Uart3txwrPointerW<Udma084Spec> {
        Uart3txwrPointerW::new(self, 0)
    }
}
#[doc = "UART3 TX write pointer\n\nYou can [`read`](crate::Reg::read) this register and get [`udma084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma084Spec;
impl crate::RegisterSpec for Udma084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma084::R`](R) reader structure"]
impl crate::Readable for Udma084Spec {}
#[doc = "`write(|w| ..)` method takes [`udma084::W`](W) writer structure"]
impl crate::Writable for Udma084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA084 to value 0"]
impl crate::Resettable for Udma084Spec {}
