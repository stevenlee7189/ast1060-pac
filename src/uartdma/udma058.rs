#[doc = "Register `UDMA058` reader"]
pub type R = crate::R<Udma058Spec>;
#[doc = "Register `UDMA058` writer"]
pub type W = crate::W<Udma058Spec>;
#[doc = "Field `UART1TXBufBaseAddr` reader - UART1 TX buffer base address"]
pub type Uart1txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART1TXBufBaseAddr` writer - UART1 TX buffer base address"]
pub type Uart1txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART1 TX buffer base address"]
    #[inline(always)]
    pub fn uart1txbuf_base_addr(&self) -> Uart1txbufBaseAddrR {
        Uart1txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART1 TX buffer base address"]
    #[inline(always)]
    pub fn uart1txbuf_base_addr(&mut self) -> Uart1txbufBaseAddrW<Udma058Spec> {
        Uart1txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART1 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma058::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma058::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma058Spec;
impl crate::RegisterSpec for Udma058Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma058::R`](R) reader structure"]
impl crate::Readable for Udma058Spec {}
#[doc = "`write(|w| ..)` method takes [`udma058::W`](W) writer structure"]
impl crate::Writable for Udma058Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA058 to value 0"]
impl crate::Resettable for Udma058Spec {}
