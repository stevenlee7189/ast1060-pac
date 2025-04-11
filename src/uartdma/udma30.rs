#[doc = "Register `UDMA30` reader"]
pub type R = crate::R<Udma30Spec>;
#[doc = "Register `UDMA30` writer"]
pub type W = crate::W<Udma30Spec>;
#[doc = "Field `UARTTXDMAINTEnbl` reader - UART TX DMA interrupt enable"]
pub type UarttxdmaintenblR = crate::FieldReader<u16>;
#[doc = "Field `UARTTXDMAINTEnbl` writer - UART TX DMA interrupt enable"]
pub type UarttxdmaintenblW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTTXDMAINTEnbl1` reader - UART TX DMA interrupt enable"]
pub type Uarttxdmaintenbl1R = crate::FieldReader;
#[doc = "Field `UARTTXDMAINTEnbl1` writer - UART TX DMA interrupt enable"]
pub type Uarttxdmaintenbl1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uarttxdmaintenbl(&self) -> UarttxdmaintenblR {
        UarttxdmaintenblR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uarttxdmaintenbl1(&self) -> Uarttxdmaintenbl1R {
        Uarttxdmaintenbl1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uarttxdmaintenbl(&mut self) -> UarttxdmaintenblW<Udma30Spec> {
        UarttxdmaintenblW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART TX DMA interrupt enable"]
    #[inline(always)]
    pub fn uarttxdmaintenbl1(&mut self) -> Uarttxdmaintenbl1W<Udma30Spec> {
        Uarttxdmaintenbl1W::new(self, 0)
    }
}
#[doc = "UART TX DMA interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`udma30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma30Spec;
impl crate::RegisterSpec for Udma30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma30::R`](R) reader structure"]
impl crate::Readable for Udma30Spec {}
#[doc = "`write(|w| ..)` method takes [`udma30::W`](W) writer structure"]
impl crate::Writable for Udma30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA30 to value 0"]
impl crate::Resettable for Udma30Spec {}
