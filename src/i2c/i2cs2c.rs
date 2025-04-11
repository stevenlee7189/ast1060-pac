#[doc = "Register `I2CS2C` reader"]
pub type R = crate::R<I2cs2cSpec>;
#[doc = "Register `I2CS2C` writer"]
pub type W = crate::W<I2cs2cSpec>;
#[doc = "Field `DMATxBufLenByte` reader - DMA Tx buffer length (Byte)"]
pub type DmatxBufLenByteR = crate::FieldReader<u16>;
#[doc = "Field `DMATxBufLenByte` writer - DMA Tx buffer length (Byte)"]
pub type DmatxBufLenByteW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMATxBufLenWrEnblForCurCmd` reader - DMA Tx buffer length write enable for current command"]
pub type DmatxBufLenWrEnblForCurCmdR = crate::BitReader;
#[doc = "Field `DMATxBufLenWrEnblForCurCmd` writer - DMA Tx buffer length write enable for current command"]
pub type DmatxBufLenWrEnblForCurCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMARxBufLenByte` reader - DMA Rx buffer length (Byte)"]
pub type DmarxBufLenByteR = crate::FieldReader<u16>;
#[doc = "Field `DMARxBufLenByte` writer - DMA Rx buffer length (Byte)"]
pub type DmarxBufLenByteW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `DMARxBufLenWrEnblForCurCmd` reader - DMA Rx buffer length write enable for current command"]
pub type DmarxBufLenWrEnblForCurCmdR = crate::BitReader;
#[doc = "Field `DMARxBufLenWrEnblForCurCmd` writer - DMA Rx buffer length write enable for current command"]
pub type DmarxBufLenWrEnblForCurCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - DMA Tx buffer length (Byte)"]
    #[inline(always)]
    pub fn dmatx_buf_len_byte(&self) -> DmatxBufLenByteR {
        DmatxBufLenByteR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 15 - DMA Tx buffer length write enable for current command"]
    #[inline(always)]
    pub fn dmatx_buf_len_wr_enbl_for_cur_cmd(&self) -> DmatxBufLenWrEnblForCurCmdR {
        DmatxBufLenWrEnblForCurCmdR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:27 - DMA Rx buffer length (Byte)"]
    #[inline(always)]
    pub fn dmarx_buf_len_byte(&self) -> DmarxBufLenByteR {
        DmarxBufLenByteR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - DMA Rx buffer length write enable for current command"]
    #[inline(always)]
    pub fn dmarx_buf_len_wr_enbl_for_cur_cmd(&self) -> DmarxBufLenWrEnblForCurCmdR {
        DmarxBufLenWrEnblForCurCmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - DMA Tx buffer length (Byte)"]
    #[inline(always)]
    pub fn dmatx_buf_len_byte(&mut self) -> DmatxBufLenByteW<I2cs2cSpec> {
        DmatxBufLenByteW::new(self, 0)
    }
    #[doc = "Bit 15 - DMA Tx buffer length write enable for current command"]
    #[inline(always)]
    pub fn dmatx_buf_len_wr_enbl_for_cur_cmd(&mut self) -> DmatxBufLenWrEnblForCurCmdW<I2cs2cSpec> {
        DmatxBufLenWrEnblForCurCmdW::new(self, 15)
    }
    #[doc = "Bits 16:27 - DMA Rx buffer length (Byte)"]
    #[inline(always)]
    pub fn dmarx_buf_len_byte(&mut self) -> DmarxBufLenByteW<I2cs2cSpec> {
        DmarxBufLenByteW::new(self, 16)
    }
    #[doc = "Bit 31 - DMA Rx buffer length write enable for current command"]
    #[inline(always)]
    pub fn dmarx_buf_len_wr_enbl_for_cur_cmd(&mut self) -> DmarxBufLenWrEnblForCurCmdW<I2cs2cSpec> {
        DmarxBufLenWrEnblForCurCmdW::new(self, 31)
    }
}
#[doc = "\newver{Slave DMA Transfer Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs2c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs2c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs2cSpec;
impl crate::RegisterSpec for I2cs2cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs2c::R`](R) reader structure"]
impl crate::Readable for I2cs2cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cs2c::W`](W) writer structure"]
impl crate::Writable for I2cs2cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS2C to value 0"]
impl crate::Resettable for I2cs2cSpec {}
