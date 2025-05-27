#[doc = "Register `I2CC08` reader"]
pub type R = crate::R<I2cc08Spec>;
#[doc = "Register `I2CC08` writer"]
pub type W = crate::W<I2cc08Spec>;
#[doc = "Field `TxByteBuffer` reader - Transmit Byte Buffer"]
pub type TxByteBufferR = crate::FieldReader;
#[doc = "Field `TxByteBuffer` writer - Transmit Byte Buffer"]
pub type TxByteBufferW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RxByteBuffer` reader - Receive Byte Buffer"]
pub type RxByteBufferR = crate::FieldReader;
#[doc = "Field `BusBusyStatus` reader - Bus Busy Status"]
pub type BusBusyStatusR = crate::BitReader;
#[doc = "Field `SampledSDALineState` reader - Sampled SDA Line State"]
pub type SampledSdalineStateR = crate::BitReader;
#[doc = "Field `SampledSCLLineState` reader - Sampled SCL Line State"]
pub type SampledScllineStateR = crate::BitReader;
#[doc = "Field `XferModeStateMachine` reader - Transfer Mode State Machine"]
pub type XferModeStateMachineR = crate::FieldReader;
#[doc = "Field `XferModeTimingStage` reader - Transfer Mode Timing Stage"]
pub type XferModeTimingStageR = crate::FieldReader;
#[doc = "Field `SCLO` reader - SCL_O"]
pub type ScloR = crate::BitReader;
#[doc = "Field `SCLOE` reader - SCL_OE"]
pub type ScloeR = crate::BitReader;
#[doc = "Field `SDAO` reader - SDA_O"]
pub type SdaoR = crate::BitReader;
#[doc = "Field `SDAOE` reader - SDA_OE"]
pub type SdaoeR = crate::BitReader;
#[doc = "Field `XferDataDirection` reader - Transfer Data Direction"]
pub type XferDataDirectionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Transmit Byte Buffer"]
    #[inline(always)]
    pub fn tx_byte_buffer(&self) -> TxByteBufferR {
        TxByteBufferR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Receive Byte Buffer"]
    #[inline(always)]
    pub fn rx_byte_buffer(&self) -> RxByteBufferR {
        RxByteBufferR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Bus Busy Status"]
    #[inline(always)]
    pub fn bus_busy_status(&self) -> BusBusyStatusR {
        BusBusyStatusR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Sampled SDA Line State"]
    #[inline(always)]
    pub fn sampled_sdaline_state(&self) -> SampledSdalineStateR {
        SampledSdalineStateR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Sampled SCL Line State"]
    #[inline(always)]
    pub fn sampled_sclline_state(&self) -> SampledScllineStateR {
        SampledScllineStateR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - Transfer Mode State Machine"]
    #[inline(always)]
    pub fn xfer_mode_state_machine(&self) -> XferModeStateMachineR {
        XferModeStateMachineR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:24 - Transfer Mode Timing Stage"]
    #[inline(always)]
    pub fn xfer_mode_timing_stage(&self) -> XferModeTimingStageR {
        XferModeTimingStageR::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - SCL_O"]
    #[inline(always)]
    pub fn sclo(&self) -> ScloR {
        ScloR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCL_OE"]
    #[inline(always)]
    pub fn scloe(&self) -> ScloeR {
        ScloeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SDA_O"]
    #[inline(always)]
    pub fn sdao(&self) -> SdaoR {
        SdaoR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - SDA_OE"]
    #[inline(always)]
    pub fn sdaoe(&self) -> SdaoeR {
        SdaoeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Transfer Data Direction"]
    #[inline(always)]
    pub fn xfer_data_direction(&self) -> XferDataDirectionR {
        XferDataDirectionR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Byte Buffer"]
    #[inline(always)]
    pub fn tx_byte_buffer(&mut self) -> TxByteBufferW<I2cc08Spec> {
        TxByteBufferW::new(self, 0)
    }
}
#[doc = "Master/Slave Transmit/Receive Byte Buffer Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc08Spec;
impl crate::RegisterSpec for I2cc08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc08::R`](R) reader structure"]
impl crate::Readable for I2cc08Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cc08::W`](W) writer structure"]
impl crate::Writable for I2cc08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC08 to value 0x0a06_0000"]
impl crate::Resettable for I2cc08Spec {
    const RESET_VALUE: u32 = 0x0a06_0000;
}
