#[doc = "Register `UDMA138` reader"]
pub type R = crate::R<Udma138Spec>;
#[doc = "Register `UDMA138` writer"]
pub type W = crate::W<Udma138Spec>;
#[doc = "Field `UART8TXBufBaseAddr` reader - UART8 TX buffer base address"]
pub type Uart8txbufBaseAddrR = crate::FieldReader<u16>;
#[doc = "Field `UART8TXBufBaseAddr` writer - UART8 TX buffer base address"]
pub type Uart8txbufBaseAddrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - UART8 TX buffer base address"]
    #[inline(always)]
    pub fn uart8txbuf_base_addr(&self) -> Uart8txbufBaseAddrR {
        Uart8txbufBaseAddrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - UART8 TX buffer base address"]
    #[inline(always)]
    pub fn uart8txbuf_base_addr(&mut self) -> Uart8txbufBaseAddrW<Udma138Spec> {
        Uart8txbufBaseAddrW::new(self, 0)
    }
}
#[doc = "UART8 RX buffer base address\n\nYou can [`read`](crate::Reg::read) this register and get [`udma138::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma138::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma138Spec;
impl crate::RegisterSpec for Udma138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma138::R`](R) reader structure"]
impl crate::Readable for Udma138Spec {}
#[doc = "`write(|w| ..)` method takes [`udma138::W`](W) writer structure"]
impl crate::Writable for Udma138Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA138 to value 0"]
impl crate::Resettable for Udma138Spec {}
