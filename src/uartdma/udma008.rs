#[doc = "Register `UDMA008` reader"]
pub type R = crate::R<Udma008Spec>;
#[doc = "Register `UDMA008` writer"]
pub type W = crate::W<Udma008Spec>;
#[doc = "Field `TXBufSize` reader - TX buffer size"]
pub type TxbufSizeR = crate::FieldReader;
#[doc = "Field `TXBufSize` writer - TX buffer size"]
pub type TxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXBufSize` reader - RX buffer size"]
pub type RxbufSizeR = crate::FieldReader;
#[doc = "Field `RXBufSize` writer - RX buffer size"]
pub type RxbufSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DisableTimeOut` reader - disable time out"]
pub type DisableTimeOutR = crate::BitReader;
#[doc = "Field `DisableTimeOut` writer - disable time out"]
pub type DisableTimeOutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - TX buffer size"]
    #[inline(always)]
    pub fn txbuf_size(&self) -> TxbufSizeR {
        TxbufSizeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - RX buffer size"]
    #[inline(always)]
    pub fn rxbuf_size(&self) -> RxbufSizeR {
        RxbufSizeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - disable time out"]
    #[inline(always)]
    pub fn disable_time_out(&self) -> DisableTimeOutR {
        DisableTimeOutR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TX buffer size"]
    #[inline(always)]
    pub fn txbuf_size(&mut self) -> TxbufSizeW<Udma008Spec> {
        TxbufSizeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - RX buffer size"]
    #[inline(always)]
    pub fn rxbuf_size(&mut self) -> RxbufSizeW<Udma008Spec> {
        RxbufSizeW::new(self, 2)
    }
    #[doc = "Bit 4 - disable time out"]
    #[inline(always)]
    pub fn disable_time_out(&mut self) -> DisableTimeOutW<Udma008Spec> {
        DisableTimeOutW::new(self, 4)
    }
}
#[doc = "Misc, buffer size\n\nYou can [`read`](crate::Reg::read) this register and get [`udma008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udma008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Udma008Spec;
impl crate::RegisterSpec for Udma008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udma008::R`](R) reader structure"]
impl crate::Readable for Udma008Spec {}
#[doc = "`write(|w| ..)` method takes [`udma008::W`](W) writer structure"]
impl crate::Writable for Udma008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDMA008 to value 0"]
impl crate::Resettable for Udma008Spec {}
