#[doc = "Register `I2CC54` reader"]
pub type R = crate::R<I2cc54Spec>;
#[doc = "Register `I2CC54` writer"]
pub type W = crate::W<I2cc54Spec>;
#[doc = "Field `CurDMAOperatingLengthCounter` reader - Current DMA Operating Length Counter"]
pub type CurDmaoperatingLengthCounterR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Current DMA Operating Length Counter"]
    #[inline(always)]
    pub fn cur_dmaoperating_length_counter(&self) -> CurDmaoperatingLengthCounterR {
        CurDmaoperatingLengthCounterR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "Current DMA Operating Length Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc54Spec;
impl crate::RegisterSpec for I2cc54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc54::R`](R) reader structure"]
impl crate::Readable for I2cc54Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cc54::W`](W) writer structure"]
impl crate::Writable for I2cc54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC54 to value 0"]
impl crate::Resettable for I2cc54Spec {}
