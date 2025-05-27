#[doc = "Register `I2CC0C` reader"]
pub type R = crate::R<I2cc0cSpec>;
#[doc = "Register `I2CC0C` writer"]
pub type W = crate::W<I2cc0cSpec>;
#[doc = "Field `BufferOrganization` reader - newverBuffer Organization"]
pub type BufferOrganizationR = crate::BitReader;
#[doc = "Field `BufferOrganization` writer - newverBuffer Organization"]
pub type BufferOrganizationW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `TxDataByteCount` reader - Transmit Data Byte Count"]
pub type TxDataByteCountR = crate::FieldReader;
#[doc = "Field `TxDataByteCount` writer - Transmit Data Byte Count"]
pub type TxDataByteCountW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `RxPoolBufferSize` reader - Receive Pool Buffer Size"]
pub type RxPoolBufferSizeR = crate::FieldReader;
#[doc = "Field `RxPoolBufferSize` writer - Receive Pool Buffer Size"]
pub type RxPoolBufferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `ActualRxdPoolBufferSize` reader - Actual Received Pool Buffer Size"]
pub type ActualRxdPoolBufferSizeR = crate::FieldReader;
#[doc = "Field `ActualRxdPoolBufferSize` writer - Actual Received Pool Buffer Size"]
pub type ActualRxdPoolBufferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - newverBuffer Organization"]
    #[inline(always)]
    pub fn buffer_organization(&self) -> BufferOrganizationR {
        BufferOrganizationR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:12 - Transmit Data Byte Count"]
    #[inline(always)]
    pub fn tx_data_byte_count(&self) -> TxDataByteCountR {
        TxDataByteCountR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Receive Pool Buffer Size"]
    #[inline(always)]
    pub fn rx_pool_buffer_size(&self) -> RxPoolBufferSizeR {
        RxPoolBufferSizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:29 - Actual Received Pool Buffer Size"]
    #[inline(always)]
    pub fn actual_rxd_pool_buffer_size(&self) -> ActualRxdPoolBufferSizeR {
        ActualRxdPoolBufferSizeR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - newverBuffer Organization"]
    #[inline(always)]
    pub fn buffer_organization(&mut self) -> BufferOrganizationW<I2cc0cSpec> {
        BufferOrganizationW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Transmit Data Byte Count"]
    #[inline(always)]
    pub fn tx_data_byte_count(&mut self) -> TxDataByteCountW<I2cc0cSpec> {
        TxDataByteCountW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Receive Pool Buffer Size"]
    #[inline(always)]
    pub fn rx_pool_buffer_size(&mut self) -> RxPoolBufferSizeW<I2cc0cSpec> {
        RxPoolBufferSizeW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Actual Received Pool Buffer Size"]
    #[inline(always)]
    pub fn actual_rxd_pool_buffer_size(&mut self) -> ActualRxdPoolBufferSizeW<I2cc0cSpec> {
        ActualRxdPoolBufferSizeW::new(self, 24)
    }
}
#[doc = "Master/Slave Pool Buffer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc0cSpec;
impl crate::RegisterSpec for I2cc0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc0c::R`](R) reader structure"]
impl crate::Readable for I2cc0cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cc0c::W`](W) writer structure"]
impl crate::Writable for I2cc0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC0C to value 0"]
impl crate::Resettable for I2cc0cSpec {}
