#[doc = "Register `I3CD0E8` reader"]
pub type R = crate::R<I3cd0e8Spec>;
#[doc = "Register `I3CD0E8` writer"]
pub type W = crate::W<I3cd0e8Spec>;
#[doc = "Field `TXBUFSIZE` reader - TX_BUF_SIZE"]
pub type TxbufsizeR = crate::FieldReader;
#[doc = "Field `RXBUFSIZE` reader - RX_BUF_SIZE"]
pub type RxbufsizeR = crate::FieldReader;
#[doc = "Field `CMDBUFSIZE` reader - CMD_BUF_SIZE"]
pub type CmdbufsizeR = crate::FieldReader;
#[doc = "Field `RESPBUFSIZE` reader - RESP_BUF_SIZE"]
pub type RespbufsizeR = crate::FieldReader;
#[doc = "Field `IBIBUFSIZE` reader - IBI_BUF_SIZE"]
pub type IbibufsizeR = crate::FieldReader;
#[doc = "Field `QueueSizeCapability` reader - Queue Size Capability"]
pub type QueueSizeCapabilityR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - TX_BUF_SIZE"]
    #[inline(always)]
    pub fn txbufsize(&self) -> TxbufsizeR {
        TxbufsizeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - RX_BUF_SIZE"]
    #[inline(always)]
    pub fn rxbufsize(&self) -> RxbufsizeR {
        RxbufsizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - CMD_BUF_SIZE"]
    #[inline(always)]
    pub fn cmdbufsize(&self) -> CmdbufsizeR {
        CmdbufsizeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RESP_BUF_SIZE"]
    #[inline(always)]
    pub fn respbufsize(&self) -> RespbufsizeR {
        RespbufsizeR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - IBI_BUF_SIZE"]
    #[inline(always)]
    pub fn ibibufsize(&self) -> IbibufsizeR {
        IbibufsizeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Queue Size Capability"]
    #[inline(always)]
    pub fn queue_size_capability(&self) -> QueueSizeCapabilityR {
        QueueSizeCapabilityR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "I3C Queue Size Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0e8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0e8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0e8Spec;
impl crate::RegisterSpec for I3cd0e8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0e8::R`](R) reader structure"]
impl crate::Readable for I3cd0e8Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0e8::W`](W) writer structure"]
impl crate::Writable for I3cd0e8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0E8 to value 0"]
impl crate::Resettable for I3cd0e8Spec {}
