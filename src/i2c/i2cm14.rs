#[doc = "Register `I2CM14` reader"]
pub type R = crate::R<I2cm14Spec>;
#[doc = "Register `I2CM14` writer"]
pub type W = crate::W<I2cm14Spec>;
#[doc = "Field `WCTxEndedWithACKRespINTSts` reader - (WC) Transmit ended with ACK response interrupt status"]
pub type WctxEndedWithAckrespIntstsR = crate::BitReader;
#[doc = "Field `WCTxEndedWithACKRespINTSts` writer - (WC) Transmit ended with ACK response interrupt status"]
pub type WctxEndedWithAckrespIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCTxEndedWithNACKRespINTSts` reader - (WC) Transmit ended with NACK response interrupt status"]
pub type WctxEndedWithNackrespIntstsR = crate::BitReader;
#[doc = "Field `WCTxEndedWithNACKRespINTSts` writer - (WC) Transmit ended with NACK response interrupt status"]
pub type WctxEndedWithNackrespIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCRxDoneINTSts` reader - (WC) Receive Done interrupt status"]
pub type WcrxDoneIntstsR = crate::BitReader;
#[doc = "Field `WCRxDoneINTSts` writer - (WC) Receive Done interrupt status"]
pub type WcrxDoneIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCMasterArbLossINTSts` reader - (WC) Master Arbitration Loss interrupt status"]
pub type WcmasterArbLossIntstsR = crate::BitReader;
#[doc = "Field `WCMasterArbLossINTSts` writer - (WC) Master Arbitration Loss interrupt status"]
pub type WcmasterArbLossIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCNormalStopCondDetINTSts` reader - (WC) Normal Stop Condition Detection interrupt status"]
pub type WcnormalStopCondDetIntstsR = crate::BitReader;
#[doc = "Field `WCNormalStopCondDetINTSts` writer - (WC) Normal Stop Condition Detection interrupt status"]
pub type WcnormalStopCondDetIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCAbnStartStopCondDetINTSts` reader - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
pub type WcabnStartStopCondDetIntstsR = crate::BitReader;
#[doc = "Field `WCAbnStartStopCondDetINTSts` writer - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
pub type WcabnStartStopCondDetIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCSCLClklowTimeoutINTSts` reader - (WC) SCL clock-low timeout interrupt status"]
pub type WcsclclklowTimeoutIntstsR = crate::BitReader;
#[doc = "Field `WCSCLClklowTimeoutINTSts` writer - (WC) SCL clock-low timeout interrupt status"]
pub type WcsclclklowTimeoutIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `WCSMBusDevAlertINTSts` reader - (WC) SMBus Device Alert interrupt status"]
pub type WcsmbusDevAlertIntstsR = crate::BitReader;
#[doc = "Field `WCSMBusDevAlertINTSts` writer - (WC) SMBus Device Alert interrupt status"]
pub type WcsmbusDevAlertIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCBusRecoverDoneINTSts` reader - (WC) Bus Recover Done interrupt status"]
pub type WcbusRecoverDoneIntstsR = crate::BitReader;
#[doc = "Field `WCBusRecoverDoneINTSts` writer - (WC) Bus Recover Done interrupt status"]
pub type WcbusRecoverDoneIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCSDADatalowTimeoutINTSts` reader - (WC) SDA data-low timeout interrupt status"]
pub type WcsdadatalowTimeoutIntstsR = crate::BitReader;
#[doc = "Field `WCSDADatalowTimeoutINTSts` writer - (WC) SDA data-low timeout interrupt status"]
pub type WcsdadatalowTimeoutIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCBusRecoverFailSts` reader - (WC) newverBus Recover Fail status"]
pub type WcbusRecoverFailStsR = crate::BitReader;
#[doc = "Field `WCBusRecoverFailSts` writer - (WC) newverBus Recover Fail status"]
pub type WcbusRecoverFailStsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCPktCmdDoneINTSts` reader - (WC) newverPacket command done interrupt status"]
pub type WcpktCmdDoneIntstsR = crate::BitReader;
#[doc = "Field `WCPktCmdDoneINTSts` writer - (WC) newverPacket command done interrupt status"]
pub type WcpktCmdDoneIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCPktCmdFailINTSts` reader - (WC) newverPacket command fail interrupt status"]
pub type WcpktCmdFailIntstsR = crate::BitReader;
#[doc = "Field `WCPktCmdFailINTSts` writer - (WC) newverPacket command fail interrupt status"]
pub type WcpktCmdFailIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WCPktCmdTimeoutINTSts` reader - (WC) newverPacket command timeout interrupt status"]
pub type WcpktCmdTimeoutIntstsR = crate::BitReader;
#[doc = "Field `WCPktCmdTimeoutINTSts` writer - (WC) newverPacket command timeout interrupt status"]
pub type WcpktCmdTimeoutIntstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PktOpStateMachine` reader - Packet Operation State Machine"]
pub type PktOpStateMachineR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - (WC) Transmit ended with ACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_ackresp_intsts(&self) -> WctxEndedWithAckrespIntstsR {
        WctxEndedWithAckrespIntstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - (WC) Transmit ended with NACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_nackresp_intsts(&self) -> WctxEndedWithNackrespIntstsR {
        WctxEndedWithNackrespIntstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - (WC) Receive Done interrupt status"]
    #[inline(always)]
    pub fn wcrx_done_intsts(&self) -> WcrxDoneIntstsR {
        WcrxDoneIntstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - (WC) Master Arbitration Loss interrupt status"]
    #[inline(always)]
    pub fn wcmaster_arb_loss_intsts(&self) -> WcmasterArbLossIntstsR {
        WcmasterArbLossIntstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - (WC) Normal Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcnormal_stop_cond_det_intsts(&self) -> WcnormalStopCondDetIntstsR {
        WcnormalStopCondDetIntstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcabn_start_stop_cond_det_intsts(&self) -> WcabnStartStopCondDetIntstsR {
        WcabnStartStopCondDetIntstsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - (WC) SCL clock-low timeout interrupt status"]
    #[inline(always)]
    pub fn wcsclclklow_timeout_intsts(&self) -> WcsclclklowTimeoutIntstsR {
        WcsclclklowTimeoutIntstsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bit 12 - (WC) SMBus Device Alert interrupt status"]
    #[inline(always)]
    pub fn wcsmbus_dev_alert_intsts(&self) -> WcsmbusDevAlertIntstsR {
        WcsmbusDevAlertIntstsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - (WC) Bus Recover Done interrupt status"]
    #[inline(always)]
    pub fn wcbus_recover_done_intsts(&self) -> WcbusRecoverDoneIntstsR {
        WcbusRecoverDoneIntstsR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - (WC) SDA data-low timeout interrupt status"]
    #[inline(always)]
    pub fn wcsdadatalow_timeout_intsts(&self) -> WcsdadatalowTimeoutIntstsR {
        WcsdadatalowTimeoutIntstsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - (WC) newverBus Recover Fail status"]
    #[inline(always)]
    pub fn wcbus_recover_fail_sts(&self) -> WcbusRecoverFailStsR {
        WcbusRecoverFailStsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - (WC) newverPacket command done interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_done_intsts(&self) -> WcpktCmdDoneIntstsR {
        WcpktCmdDoneIntstsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - (WC) newverPacket command fail interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_fail_intsts(&self) -> WcpktCmdFailIntstsR {
        WcpktCmdFailIntstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - (WC) newverPacket command timeout interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_timeout_intsts(&self) -> WcpktCmdTimeoutIntstsR {
        WcpktCmdTimeoutIntstsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Packet Operation State Machine"]
    #[inline(always)]
    pub fn pkt_op_state_machine(&self) -> PktOpStateMachineR {
        PktOpStateMachineR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - (WC) Transmit ended with ACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_ackresp_intsts(&mut self) -> WctxEndedWithAckrespIntstsW<I2cm14Spec> {
        WctxEndedWithAckrespIntstsW::new(self, 0)
    }
    #[doc = "Bit 1 - (WC) Transmit ended with NACK response interrupt status"]
    #[inline(always)]
    pub fn wctx_ended_with_nackresp_intsts(&mut self) -> WctxEndedWithNackrespIntstsW<I2cm14Spec> {
        WctxEndedWithNackrespIntstsW::new(self, 1)
    }
    #[doc = "Bit 2 - (WC) Receive Done interrupt status"]
    #[inline(always)]
    pub fn wcrx_done_intsts(&mut self) -> WcrxDoneIntstsW<I2cm14Spec> {
        WcrxDoneIntstsW::new(self, 2)
    }
    #[doc = "Bit 3 - (WC) Master Arbitration Loss interrupt status"]
    #[inline(always)]
    pub fn wcmaster_arb_loss_intsts(&mut self) -> WcmasterArbLossIntstsW<I2cm14Spec> {
        WcmasterArbLossIntstsW::new(self, 3)
    }
    #[doc = "Bit 4 - (WC) Normal Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcnormal_stop_cond_det_intsts(&mut self) -> WcnormalStopCondDetIntstsW<I2cm14Spec> {
        WcnormalStopCondDetIntstsW::new(self, 4)
    }
    #[doc = "Bit 5 - (WC) Abnormal Start/Stop Condition Detection interrupt status"]
    #[inline(always)]
    pub fn wcabn_start_stop_cond_det_intsts(&mut self) -> WcabnStartStopCondDetIntstsW<I2cm14Spec> {
        WcabnStartStopCondDetIntstsW::new(self, 5)
    }
    #[doc = "Bit 6 - (WC) SCL clock-low timeout interrupt status"]
    #[inline(always)]
    pub fn wcsclclklow_timeout_intsts(&mut self) -> WcsclclklowTimeoutIntstsW<I2cm14Spec> {
        WcsclclklowTimeoutIntstsW::new(self, 6)
    }
    #[doc = "Bit 12 - (WC) SMBus Device Alert interrupt status"]
    #[inline(always)]
    pub fn wcsmbus_dev_alert_intsts(&mut self) -> WcsmbusDevAlertIntstsW<I2cm14Spec> {
        WcsmbusDevAlertIntstsW::new(self, 12)
    }
    #[doc = "Bit 13 - (WC) Bus Recover Done interrupt status"]
    #[inline(always)]
    pub fn wcbus_recover_done_intsts(&mut self) -> WcbusRecoverDoneIntstsW<I2cm14Spec> {
        WcbusRecoverDoneIntstsW::new(self, 13)
    }
    #[doc = "Bit 14 - (WC) SDA data-low timeout interrupt status"]
    #[inline(always)]
    pub fn wcsdadatalow_timeout_intsts(&mut self) -> WcsdadatalowTimeoutIntstsW<I2cm14Spec> {
        WcsdadatalowTimeoutIntstsW::new(self, 14)
    }
    #[doc = "Bit 15 - (WC) newverBus Recover Fail status"]
    #[inline(always)]
    pub fn wcbus_recover_fail_sts(&mut self) -> WcbusRecoverFailStsW<I2cm14Spec> {
        WcbusRecoverFailStsW::new(self, 15)
    }
    #[doc = "Bit 16 - (WC) newverPacket command done interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_done_intsts(&mut self) -> WcpktCmdDoneIntstsW<I2cm14Spec> {
        WcpktCmdDoneIntstsW::new(self, 16)
    }
    #[doc = "Bit 17 - (WC) newverPacket command fail interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_fail_intsts(&mut self) -> WcpktCmdFailIntstsW<I2cm14Spec> {
        WcpktCmdFailIntstsW::new(self, 17)
    }
    #[doc = "Bit 18 - (WC) newverPacket command timeout interrupt status"]
    #[inline(always)]
    pub fn wcpkt_cmd_timeout_intsts(&mut self) -> WcpktCmdTimeoutIntstsW<I2cm14Spec> {
        WcpktCmdTimeoutIntstsW::new(self, 18)
    }
}
#[doc = "Master Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cm14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cm14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cm14Spec;
impl crate::RegisterSpec for I2cm14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cm14::R`](R) reader structure"]
impl crate::Readable for I2cm14Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cm14::W`](W) writer structure"]
impl crate::Writable for I2cm14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CM14 to value 0"]
impl crate::Resettable for I2cm14Spec {}
