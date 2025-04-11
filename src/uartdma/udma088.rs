#[doc = "Register `UDMA088` reader"]
pub type R = crate::R<Udma088Spec>;
#[doc = "Register `UDMA088` writer"]
pub type W = crate::W<Udma088Spec>;
#[doc = "Field `UART3TXBufBaseAddr` reader - UART3 TX buffer base address"]
pub type Uart3txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART3TXBufBaseAddr` writer - UART3 TX buffer base address"]
pub type Uart3txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART3 TX buffer base address"]
    #[inline(always)]
    pub fn uart3txbuf_base_addr(&self) -> Uart3txbufBaseAddrR {
        Uart3txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART3 TX buffer base address"]
    #[inline(always)]
    pub fn uart3txbuf_base_addr(&mut self) -> Uart3txbufBaseAddrW<Udma088Spec> {
        Uart3txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART3 TX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma088::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma088::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma088Spec;
impl crate::RegisterSpec for Udma088Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma088::R`](R) reader structure"]
impl crate::Readable for Udma088Spec {}
#[doc = "`write(|w| ..)` method takes [`udma088::W`](W) writer structure"]
impl crate::Writable for Udma088Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA088 to value 0"]
impl crate::Resettable for Udma088Spec {}
