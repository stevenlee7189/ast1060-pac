#[doc = "Register `UDMA098` reader"]
pub type R = crate::R<Udma098Spec>;
#[doc = "Register `UDMA098` writer"]
pub type W = crate::W<Udma098Spec>;
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
    pub fn uart3txbuf_base_addr(&mut self) -> Uart3txbufBaseAddrW<Udma098Spec> {
        Uart3txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART3 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma098::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma098::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma098Spec;
impl crate::RegisterSpec for Udma098Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma098::R`](R) reader structure"]
impl crate::Readable for Udma098Spec {}
#[doc = "`write(|w| ..)` method takes [`udma098::W`](W) writer structure"]
impl crate::Writable for Udma098Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA098 to value 0"]
impl crate::Resettable for Udma098Spec {}
