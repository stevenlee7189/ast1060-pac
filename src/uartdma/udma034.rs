#[doc = "Register `UDMA034` reader"]
pub type R = crate::R<Udma034Spec>;
#[doc = "Register `UDMA034` writer"]
pub type W = crate::W<Udma034Spec>;
#[doc = "Field `UARTTXDMAINTSts` reader - UART TX DMA interrupt status"]
pub type UarttxdmaintstsR = crate::FieldReader<u16>;
#[doc = "Field `UARTTXDMAINTSts` writer - UART TX DMA interrupt status"]
pub type UarttxdmaintstsW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `UARTTXDMAINTSts1` reader - UART TX DMA interrupt status"]
pub type Uarttxdmaintsts1R = crate::FieldReader;
#[doc = "Field `UARTTXDMAINTSts1` writer - UART TX DMA interrupt status"]
pub type Uarttxdmaintsts1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:11 - UART TX DMA interrupt status"]
    #[inline(always)]
    pub fn uarttxdmaintsts(&self) -> UarttxdmaintstsR {
        UarttxdmaintstsR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 0:3 - UART TX DMA interrupt status"]
    #[inline(always)]
    pub fn uarttxdmaintsts1(&self) -> Uarttxdmaintsts1R {
        Uarttxdmaintsts1R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - UART TX DMA interrupt status"]
    #[inline(always)]
    pub fn uarttxdmaintsts(&mut self) -> UarttxdmaintstsW<Udma034Spec> {
        UarttxdmaintstsW::new(self, 0)
    }
    #[doc = "Bits 0:3 - UART TX DMA interrupt status"]
    #[inline(always)]
    pub fn uarttxdmaintsts1(&mut self) -> Uarttxdmaintsts1W<Udma034Spec> {
        Uarttxdmaintsts1W::new(self, 0)
    }
}
#[doc = "UART TX DMA interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`udma034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma034Spec;
impl crate::RegisterSpec for Udma034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma034::R`](R) reader structure"]
impl crate::Readable for Udma034Spec {}
#[doc = "`write(|w| ..)` method takes [`udma034::W`](W) writer structure"]
impl crate::Writable for Udma034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA034 to value 0"]
impl crate::Resettable for Udma034Spec {}
