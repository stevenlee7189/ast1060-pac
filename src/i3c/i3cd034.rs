#[doc = "Register `I3CD034` reader"]
pub type R = crate::R<I3cd034Spec>;
#[doc = "Register `I3CD034` writer"]
pub type W = crate::W<I3cd034Spec>;
#[doc = "Field `CoreSwRst` reader - Core Software Reset."]
pub type CoreSwRstR = crate::BitReader;
#[doc = "Field `CoreSwRst` writer - Core Software Reset."]
pub type CoreSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CmdQueueSwRst` reader - Command Queue Software Reset."]
pub type CmdQueueSwRstR = crate::BitReader;
#[doc = "Field `CmdQueueSwRst` writer - Command Queue Software Reset."]
pub type CmdQueueSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ResponseQueueSwRst` reader - Response Queue Software Reset."]
pub type ResponseQueueSwRstR = crate::BitReader;
#[doc = "Field `ResponseQueueSwRst` writer - Response Queue Software Reset."]
pub type ResponseQueueSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxBufferSwRst` reader - Transmit Buffer Software Reset."]
pub type TxBufferSwRstR = crate::BitReader;
#[doc = "Field `TxBufferSwRst` writer - Transmit Buffer Software Reset."]
pub type TxBufferSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxBufferSwRst` reader - Receive Buffer Software Reset."]
pub type RxBufferSwRstR = crate::BitReader;
#[doc = "Field `RxBufferSwRst` writer - Receive Buffer Software Reset."]
pub type RxBufferSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IBIQueueSwRst` reader - IBI Queue Software Reset"]
pub type IbiqueueSwRstR = crate::BitReader;
#[doc = "Field `IBIQueueSwRst` writer - IBI Queue Software Reset"]
pub type IbiqueueSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD286` reader - These bits in Reset Control Register are reserved."]
pub type Rsvd286R = crate::FieldReader<u32>;
#[doc = "Field `BusRstType` reader - Bus Reset type"]
pub type BusRstTypeR = crate::FieldReader;
#[doc = "Field `BusRst` reader - Bus Reset."]
pub type BusRstR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Core Software Reset."]
    #[inline(always)]
    pub fn core_sw_rst(&self) -> CoreSwRstR {
        CoreSwRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command Queue Software Reset."]
    #[inline(always)]
    pub fn cmd_queue_sw_rst(&self) -> CmdQueueSwRstR {
        CmdQueueSwRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Response Queue Software Reset."]
    #[inline(always)]
    pub fn response_queue_sw_rst(&self) -> ResponseQueueSwRstR {
        ResponseQueueSwRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Buffer Software Reset."]
    #[inline(always)]
    pub fn tx_buffer_sw_rst(&self) -> TxBufferSwRstR {
        TxBufferSwRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Buffer Software Reset."]
    #[inline(always)]
    pub fn rx_buffer_sw_rst(&self) -> RxBufferSwRstR {
        RxBufferSwRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - IBI Queue Software Reset"]
    #[inline(always)]
    pub fn ibiqueue_sw_rst(&self) -> IbiqueueSwRstR {
        IbiqueueSwRstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:28 - These bits in Reset Control Register are reserved."]
    #[inline(always)]
    pub fn rsvd286(&self) -> Rsvd286R {
        Rsvd286R::new((self.bits >> 6) & 0x007f_ffff)
    }
    #[doc = "Bits 29:30 - Bus Reset type"]
    #[inline(always)]
    pub fn bus_rst_type(&self) -> BusRstTypeR {
        BusRstTypeR::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - Bus Reset."]
    #[inline(always)]
    pub fn bus_rst(&self) -> BusRstR {
        BusRstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Core Software Reset."]
    #[inline(always)]
    pub fn core_sw_rst(&mut self) -> CoreSwRstW<I3cd034Spec> {
        CoreSwRstW::new(self, 0)
    }
    #[doc = "Bit 1 - Command Queue Software Reset."]
    #[inline(always)]
    pub fn cmd_queue_sw_rst(&mut self) -> CmdQueueSwRstW<I3cd034Spec> {
        CmdQueueSwRstW::new(self, 1)
    }
    #[doc = "Bit 2 - Response Queue Software Reset."]
    #[inline(always)]
    pub fn response_queue_sw_rst(&mut self) -> ResponseQueueSwRstW<I3cd034Spec> {
        ResponseQueueSwRstW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Buffer Software Reset."]
    #[inline(always)]
    pub fn tx_buffer_sw_rst(&mut self) -> TxBufferSwRstW<I3cd034Spec> {
        TxBufferSwRstW::new(self, 3)
    }
    #[doc = "Bit 4 - Receive Buffer Software Reset."]
    #[inline(always)]
    pub fn rx_buffer_sw_rst(&mut self) -> RxBufferSwRstW<I3cd034Spec> {
        RxBufferSwRstW::new(self, 4)
    }
    #[doc = "Bit 5 - IBI Queue Software Reset"]
    #[inline(always)]
    pub fn ibiqueue_sw_rst(&mut self) -> IbiqueueSwRstW<I3cd034Spec> {
        IbiqueueSwRstW::new(self, 5)
    }
}
#[doc = "Reset Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd034::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd034::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd034Spec;
impl crate::RegisterSpec for I3cd034Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd034::R`](R) reader structure"]
impl crate::Readable for I3cd034Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd034::W`](W) writer structure"]
impl crate::Writable for I3cd034Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD034 to value 0"]
impl crate::Resettable for I3cd034Spec {}
