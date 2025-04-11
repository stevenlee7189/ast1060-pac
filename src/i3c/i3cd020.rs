#[doc = "Register `I3CD020` reader"]
pub type R = crate::R<I3cd020Spec>;
#[doc = "Register `I3CD020` writer"]
pub type W = crate::W<I3cd020Spec>;
#[doc = "Field `TxBufferThresholdValue` reader - Transmit Buffer Threshold Value."]
pub type TxBufferThresholdValueR = crate::FieldReader;
#[doc = "Field `TxBufferThresholdValue` writer - Transmit Buffer Threshold Value."]
pub type TxBufferThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD3` reader - These bits in Data Buffer Threshold Control register are reserved."]
pub type Rsvd3R = crate::BitReader;
#[doc = "Field `RSVD74` reader - These bits in Data Buffer Threshold Control Register are reserved."]
pub type Rsvd74R = crate::FieldReader;
#[doc = "Field `RxBufferThresholdValue` reader - Receive Buffer Threshold Value."]
pub type RxBufferThresholdValueR = crate::FieldReader;
#[doc = "Field `RxBufferThresholdValue` writer - Receive Buffer Threshold Value."]
pub type RxBufferThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD11` reader - These bits in Data Buffer Threshold Control register are reserved."]
pub type Rsvd11R = crate::BitReader;
#[doc = "Field `RSVD1512` reader - These bits in Data Buffer Threshold Control Register are reserved."]
pub type Rsvd1512R = crate::FieldReader;
#[doc = "Field `XferStartThresholdValue` reader - Transfer Start Threshold Value."]
pub type XferStartThresholdValueR = crate::FieldReader;
#[doc = "Field `XferStartThresholdValue` writer - Transfer Start Threshold Value."]
pub type XferStartThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD19` reader - These bits in Data Buffer Threshold Control Register are reserved."]
pub type Rsvd19R = crate::BitReader;
#[doc = "Field `RSVD2320` reader - These bits in Data Buffer Threshold Control Register are reserved."]
pub type Rsvd2320R = crate::FieldReader;
#[doc = "Field `RxStartThresholdValue` reader - Receive Start Threshold Value."]
pub type RxStartThresholdValueR = crate::FieldReader;
#[doc = "Field `RxStartThresholdValue` writer - Receive Start Threshold Value."]
pub type RxStartThresholdValueW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD27` reader - These bits in Data Buffer Threshold Control register are reserved."]
pub type Rsvd27R = crate::BitReader;
#[doc = "Field `RSVD3128` reader - These bits in Data Buffer Threshold Control register are reserved."]
pub type Rsvd3128R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Transmit Buffer Threshold Value."]
    #[inline(always)]
    pub fn tx_buffer_threshold_value(&self) -> TxBufferThresholdValueR {
        TxBufferThresholdValueR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - These bits in Data Buffer Threshold Control register are reserved."]
    #[inline(always)]
    pub fn rsvd3(&self) -> Rsvd3R {
        Rsvd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - These bits in Data Buffer Threshold Control Register are reserved."]
    #[inline(always)]
    pub fn rsvd74(&self) -> Rsvd74R {
        Rsvd74R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - Receive Buffer Threshold Value."]
    #[inline(always)]
    pub fn rx_buffer_threshold_value(&self) -> RxBufferThresholdValueR {
        RxBufferThresholdValueR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - These bits in Data Buffer Threshold Control register are reserved."]
    #[inline(always)]
    pub fn rsvd11(&self) -> Rsvd11R {
        Rsvd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - These bits in Data Buffer Threshold Control Register are reserved."]
    #[inline(always)]
    pub fn rsvd1512(&self) -> Rsvd1512R {
        Rsvd1512R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:18 - Transfer Start Threshold Value."]
    #[inline(always)]
    pub fn xfer_start_threshold_value(&self) -> XferStartThresholdValueR {
        XferStartThresholdValueR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - These bits in Data Buffer Threshold Control Register are reserved."]
    #[inline(always)]
    pub fn rsvd19(&self) -> Rsvd19R {
        Rsvd19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - These bits in Data Buffer Threshold Control Register are reserved."]
    #[inline(always)]
    pub fn rsvd2320(&self) -> Rsvd2320R {
        Rsvd2320R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Receive Start Threshold Value."]
    #[inline(always)]
    pub fn rx_start_threshold_value(&self) -> RxStartThresholdValueR {
        RxStartThresholdValueR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - These bits in Data Buffer Threshold Control register are reserved."]
    #[inline(always)]
    pub fn rsvd27(&self) -> Rsvd27R {
        Rsvd27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - These bits in Data Buffer Threshold Control register are reserved."]
    #[inline(always)]
    pub fn rsvd3128(&self) -> Rsvd3128R {
        Rsvd3128R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Transmit Buffer Threshold Value."]
    #[inline(always)]
    pub fn tx_buffer_threshold_value(&mut self) -> TxBufferThresholdValueW<I3cd020Spec> {
        TxBufferThresholdValueW::new(self, 0)
    }
    #[doc = "Bits 8:10 - Receive Buffer Threshold Value."]
    #[inline(always)]
    pub fn rx_buffer_threshold_value(&mut self) -> RxBufferThresholdValueW<I3cd020Spec> {
        RxBufferThresholdValueW::new(self, 8)
    }
    #[doc = "Bits 16:18 - Transfer Start Threshold Value."]
    #[inline(always)]
    pub fn xfer_start_threshold_value(&mut self) -> XferStartThresholdValueW<I3cd020Spec> {
        XferStartThresholdValueW::new(self, 16)
    }
    #[doc = "Bits 24:26 - Receive Start Threshold Value."]
    #[inline(always)]
    pub fn rx_start_threshold_value(&mut self) -> RxStartThresholdValueW<I3cd020Spec> {
        RxStartThresholdValueW::new(self, 24)
    }
}
#[doc = "Data Buffer Threshold Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd020Spec;
impl crate::RegisterSpec for I3cd020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd020::R`](R) reader structure"]
impl crate::Readable for I3cd020Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd020::W`](W) writer structure"]
impl crate::Writable for I3cd020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD020 to value 0"]
impl crate::Resettable for I3cd020Spec {}
