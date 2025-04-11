#[doc = "Register `UDMA0B8` reader"]
pub type R = crate::R<Udma0b8Spec>;
#[doc = "Register `UDMA0B8` writer"]
pub type W = crate::W<Udma0b8Spec>;
#[doc = "Field `UART4TXBufBaseAddr` reader - UART4 TX buffer base address"]
pub type Uart4txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART4TXBufBaseAddr` writer - UART4 TX buffer base address"]
pub type Uart4txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART4 TX buffer base address"]
    #[inline(always)]
    pub fn uart4txbuf_base_addr(&self) -> Uart4txbufBaseAddrR {
        Uart4txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART4 TX buffer base address"]
    #[inline(always)]
    pub fn uart4txbuf_base_addr(&mut self) -> Uart4txbufBaseAddrW<Udma0b8Spec> {
        Uart4txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART4 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma0b8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma0b8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma0b8Spec;
impl crate::RegisterSpec for Udma0b8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma0b8::R`](R) reader structure"]
impl crate::Readable for Udma0b8Spec {}
#[doc = "`write(|w| ..)` method takes [`udma0b8::W`](W) writer structure"]
impl crate::Writable for Udma0b8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA0B8 to value 0"]
impl crate::Resettable for Udma0b8Spec {}
