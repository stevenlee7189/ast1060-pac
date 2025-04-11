#[doc = "Register `I2CS28` reader"]
pub type R = crate::R<I2cs28Spec>;
#[doc = "Register `I2CS28` writer"]
pub type W = crate::W<I2cs28Spec>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `SlaveTxCmd` reader - Slave Transmit Command"]
pub type SlaveTxCmdR = crate::BitReader;
#[doc = "Field `SlaveTxCmd` writer - Slave Transmit Command"]
pub type SlaveTxCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SlaveRxCmdLast` reader - Slave Receive Command Last"]
pub type SlaveRxCmdLastR = crate::BitReader;
#[doc = "Field `SlaveRxCmdLast` writer - Slave Receive Command Last"]
pub type SlaveRxCmdLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlaveTxPoolDataBuf` reader - Enable Slave Tx Pool Data buffer"]
pub type EnblSlaveTxPoolDataBufR = crate::BitReader;
#[doc = "Field `EnblSlaveTxPoolDataBuf` writer - Enable Slave Tx Pool Data buffer"]
pub type EnblSlaveTxPoolDataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlaveRxPoolDataBuf` reader - Enable Slave Rx Pool Data buffer"]
pub type EnblSlaveRxPoolDataBufR = crate::BitReader;
#[doc = "Field `EnblSlaveRxPoolDataBuf` writer - Enable Slave Rx Pool Data buffer"]
pub type EnblSlaveRxPoolDataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlaveTxDMADataBuf` reader - Enable Slave Tx DMA Data buffer"]
pub type EnblSlaveTxDmadataBufR = crate::BitReader;
#[doc = "Field `EnblSlaveTxDMADataBuf` writer - Enable Slave Tx DMA Data buffer"]
pub type EnblSlaveTxDmadataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlaveRxDMADataBuf` reader - Enable Slave Rx DMA Data buffer"]
pub type EnblSlaveRxDmadataBufR = crate::BitReader;
#[doc = "Field `EnblSlaveRxDMADataBuf` writer - Enable Slave Rx DMA Data buffer"]
pub type EnblSlaveRxDmadataBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblIssuingI2CSMBusSlaveAlertSig` reader - Enable issuing I2C/SMBus Slave Alert signal"]
pub type EnblIssuingI2csmbusSlaveAlertSigR = crate::BitReader;
#[doc = "Field `EnblIssuingI2CSMBusSlaveAlertSig` writer - Enable issuing I2C/SMBus Slave Alert signal"]
pub type EnblIssuingI2csmbusSlaveAlertSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `EnblAutoNACKToActiveAddr` reader - Enable Auto NACK to Active address"]
pub type EnblAutoNacktoActiveAddrR = crate::BitReader;
#[doc = "Field `EnblAutoNACKToActiveAddr` writer - Enable Auto NACK to Active address"]
pub type EnblAutoNacktoActiveAddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblAutoNACKToNonActiveAddr` reader - Enable Auto NACK to Non-Active address"]
pub type EnblAutoNacktoNonActiveAddrR = crate::BitReader;
#[doc = "Field `EnblAutoNACKToNonActiveAddr` writer - Enable Auto NACK to Non-Active address"]
pub type EnblAutoNacktoNonActiveAddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlavePktOpMode` reader - Enable slave Packet operation mode"]
pub type EnblSlavePktOpModeR = crate::BitReader;
#[doc = "Field `EnblSlavePktOpMode` writer - Enable slave Packet operation mode"]
pub type EnblSlavePktOpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ActiveAddrSelForPktOp` reader - Active address selection for Packet operation"]
pub type ActiveAddrSelForPktOpR = crate::FieldReader;
#[doc = "Field `ActiveAddrSelForPktOp` writer - Active address selection for Packet operation"]
pub type ActiveAddrSelForPktOpW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
#[doc = "Field `Wr1CtrlForI2CS28ForCurCmd` reader - Write '1' control for I2CS28 for current command"]
pub type Wr1ctrlForI2cs28forCurCmdR = crate::BitReader;
#[doc = "Field `Wr1CtrlForI2CS28ForCurCmd` writer - Write '1' control for I2CS28 for current command"]
pub type Wr1ctrlForI2cs28forCurCmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Slave Transmit Command"]
    #[inline(always)]
    pub fn slave_tx_cmd(&self) -> SlaveTxCmdR {
        SlaveTxCmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Receive Command Last"]
    #[inline(always)]
    pub fn slave_rx_cmd_last(&self) -> SlaveRxCmdLastR {
        SlaveRxCmdLastR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Slave Tx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_tx_pool_data_buf(&self) -> EnblSlaveTxPoolDataBufR {
        EnblSlaveTxPoolDataBufR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Slave Rx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_rx_pool_data_buf(&self) -> EnblSlaveRxPoolDataBufR {
        EnblSlaveRxPoolDataBufR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Slave Tx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_tx_dmadata_buf(&self) -> EnblSlaveTxDmadataBufR {
        EnblSlaveTxDmadataBufR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Slave Rx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_rx_dmadata_buf(&self) -> EnblSlaveRxDmadataBufR {
        EnblSlaveRxDmadataBufR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable issuing I2C/SMBus Slave Alert signal"]
    #[inline(always)]
    pub fn enbl_issuing_i2csmbus_slave_alert_sig(&self) -> EnblIssuingI2csmbusSlaveAlertSigR {
        EnblIssuingI2csmbusSlaveAlertSigR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Enable Auto NACK to Active address"]
    #[inline(always)]
    pub fn enbl_auto_nackto_active_addr(&self) -> EnblAutoNacktoActiveAddrR {
        EnblAutoNacktoActiveAddrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable Auto NACK to Non-Active address"]
    #[inline(always)]
    pub fn enbl_auto_nackto_non_active_addr(&self) -> EnblAutoNacktoNonActiveAddrR {
        EnblAutoNacktoNonActiveAddrR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable slave Packet operation mode"]
    #[inline(always)]
    pub fn enbl_slave_pkt_op_mode(&self) -> EnblSlavePktOpModeR {
        EnblSlavePktOpModeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Active address selection for Packet operation"]
    #[inline(always)]
    pub fn active_addr_sel_for_pkt_op(&self) -> ActiveAddrSelForPktOpR {
        ActiveAddrSelForPktOpR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:30 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 20) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - Write '1' control for I2CS28 for current command"]
    #[inline(always)]
    pub fn wr1ctrl_for_i2cs28for_cur_cmd(&self) -> Wr1ctrlForI2cs28forCurCmdR {
        Wr1ctrlForI2cs28forCurCmdR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Slave Transmit Command"]
    #[inline(always)]
    pub fn slave_tx_cmd(&mut self) -> SlaveTxCmdW<I2cs28Spec> {
        SlaveTxCmdW::new(self, 2)
    }
    #[doc = "Bit 4 - Slave Receive Command Last"]
    #[inline(always)]
    pub fn slave_rx_cmd_last(&mut self) -> SlaveRxCmdLastW<I2cs28Spec> {
        SlaveRxCmdLastW::new(self, 4)
    }
    #[doc = "Bit 6 - Enable Slave Tx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_tx_pool_data_buf(&mut self) -> EnblSlaveTxPoolDataBufW<I2cs28Spec> {
        EnblSlaveTxPoolDataBufW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Slave Rx Pool Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_rx_pool_data_buf(&mut self) -> EnblSlaveRxPoolDataBufW<I2cs28Spec> {
        EnblSlaveRxPoolDataBufW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable Slave Tx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_tx_dmadata_buf(&mut self) -> EnblSlaveTxDmadataBufW<I2cs28Spec> {
        EnblSlaveTxDmadataBufW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Slave Rx DMA Data buffer"]
    #[inline(always)]
    pub fn enbl_slave_rx_dmadata_buf(&mut self) -> EnblSlaveRxDmadataBufW<I2cs28Spec> {
        EnblSlaveRxDmadataBufW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable issuing I2C/SMBus Slave Alert signal"]
    #[inline(always)]
    pub fn enbl_issuing_i2csmbus_slave_alert_sig(
        &mut self,
    ) -> EnblIssuingI2csmbusSlaveAlertSigW<I2cs28Spec> {
        EnblIssuingI2csmbusSlaveAlertSigW::new(self, 10)
    }
    #[doc = "Bit 14 - Enable Auto NACK to Active address"]
    #[inline(always)]
    pub fn enbl_auto_nackto_active_addr(&mut self) -> EnblAutoNacktoActiveAddrW<I2cs28Spec> {
        EnblAutoNacktoActiveAddrW::new(self, 14)
    }
    #[doc = "Bit 15 - Enable Auto NACK to Non-Active address"]
    #[inline(always)]
    pub fn enbl_auto_nackto_non_active_addr(&mut self) -> EnblAutoNacktoNonActiveAddrW<I2cs28Spec> {
        EnblAutoNacktoNonActiveAddrW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable slave Packet operation mode"]
    #[inline(always)]
    pub fn enbl_slave_pkt_op_mode(&mut self) -> EnblSlavePktOpModeW<I2cs28Spec> {
        EnblSlavePktOpModeW::new(self, 16)
    }
    #[doc = "Bits 17:18 - Active address selection for Packet operation"]
    #[inline(always)]
    pub fn active_addr_sel_for_pkt_op(&mut self) -> ActiveAddrSelForPktOpW<I2cs28Spec> {
        ActiveAddrSelForPktOpW::new(self, 17)
    }
    #[doc = "Bit 31 - Write '1' control for I2CS28 for current command"]
    #[inline(always)]
    pub fn wr1ctrl_for_i2cs28for_cur_cmd(&mut self) -> Wr1ctrlForI2cs28forCurCmdW<I2cs28Spec> {
        Wr1ctrlForI2cs28forCurCmdW::new(self, 31)
    }
}
#[doc = "Slave Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs28Spec;
impl crate::RegisterSpec for I2cs28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs28::R`](R) reader structure"]
impl crate::Readable for I2cs28Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cs28::W`](W) writer structure"]
impl crate::Writable for I2cs28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS28 to value 0"]
impl crate::Resettable for I2cs28Spec {}
