#[doc = "Register `I2CC00` reader"]
pub type R = crate::R<I2cc00Spec>;
#[doc = "Register `I2CC00` writer"]
pub type W = crate::W<I2cc00Spec>;
#[doc = "Field `SameAsHlinkI2CD00` reader - Same as hlinkI2CD00"]
pub type SameAsHlinkI2cd00R = crate::FieldReader<u32>;
#[doc = "Field `SameAsHlinkI2CD00` writer - Same as hlinkI2CD00"]
pub type SameAsHlinkI2cd00W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `MasterPktOpRetryCount` reader - newverMaster packet operation retry count"]
pub type MasterPktOpRetryCountR = crate::FieldReader;
#[doc = "Field `MasterPktOpRetryCount` writer - newverMaster packet operation retry count"]
pub type MasterPktOpRetryCountW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EnblSaveAddrByteIntoBufForSlavePktModeRxCmd` reader - newverEnable save address byte into buffer for Slave Packet mode receive command"]
pub type EnblSaveAddrByteIntoBufForSlavePktModeRxCmdR = crate::BitReader;
#[doc = "Field `EnblSaveAddrByteIntoBufForSlavePktModeRxCmd` writer - newverEnable save address byte into buffer for Slave Packet mode receive command"]
pub type EnblSaveAddrByteIntoBufForSlavePktModeRxCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisDMAAutoRecoveryWhenResidualTxCountEquals0` reader - newverDisableDMA AutoRecovery when residual Tx Count equals 0"]
pub type DisDmaautoRecoveryWhenResidualTxCountEquals0R = crate::BitReader;
#[doc = "Field `DisDMAAutoRecoveryWhenResidualTxCountEquals0` writer - newverDisableDMA AutoRecovery when residual Tx Count equals 0"]
pub type DisDmaautoRecoveryWhenResidualTxCountEquals0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblRBEarlyDone` reader - newverEnable RB early done"]
pub type EnblRbearlyDoneR = crate::BitReader;
#[doc = "Field `EnblRBEarlyDone` writer - newverEnable RB early done"]
pub type EnblRbearlyDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader<u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:17 - Same as hlinkI2CD00"]
    #[inline(always)]
    pub fn same_as_hlink_i2cd00(&self) -> SameAsHlinkI2cd00R {
        SameAsHlinkI2cd00R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - newverMaster packet operation retry count"]
    #[inline(always)]
    pub fn master_pkt_op_retry_count(&self) -> MasterPktOpRetryCountR {
        MasterPktOpRetryCountR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - newverEnable save address byte into buffer for Slave Packet mode receive command"]
    #[inline(always)]
    pub fn enbl_save_addr_byte_into_buf_for_slave_pkt_mode_rx_cmd(
        &self,
    ) -> EnblSaveAddrByteIntoBufForSlavePktModeRxCmdR {
        EnblSaveAddrByteIntoBufForSlavePktModeRxCmdR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - newverDisableDMA AutoRecovery when residual Tx Count equals 0"]
    #[inline(always)]
    pub fn dis_dmaauto_recovery_when_residual_tx_count_equals0(
        &self,
    ) -> DisDmaautoRecoveryWhenResidualTxCountEquals0R {
        DisDmaautoRecoveryWhenResidualTxCountEquals0R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - newverEnable RB early done"]
    #[inline(always)]
    pub fn enbl_rbearly_done(&self) -> EnblRbearlyDoneR {
        EnblRbearlyDoneR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
    #[doc = "Bits 23:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:17 - Same as hlinkI2CD00"]
    #[inline(always)]
    pub fn same_as_hlink_i2cd00(&mut self) -> SameAsHlinkI2cd00W<I2cc00Spec> {
        SameAsHlinkI2cd00W::new(self, 0)
    }
    #[doc = "Bits 18:19 - newverMaster packet operation retry count"]
    #[inline(always)]
    pub fn master_pkt_op_retry_count(&mut self) -> MasterPktOpRetryCountW<I2cc00Spec> {
        MasterPktOpRetryCountW::new(self, 18)
    }
    #[doc = "Bit 20 - newverEnable save address byte into buffer for Slave Packet mode receive command"]
    #[inline(always)]
    pub fn enbl_save_addr_byte_into_buf_for_slave_pkt_mode_rx_cmd(
        &mut self,
    ) -> EnblSaveAddrByteIntoBufForSlavePktModeRxCmdW<I2cc00Spec> {
        EnblSaveAddrByteIntoBufForSlavePktModeRxCmdW::new(self, 20)
    }
    #[doc = "Bit 21 - newverDisableDMA AutoRecovery when residual Tx Count equals 0"]
    #[inline(always)]
    pub fn dis_dmaauto_recovery_when_residual_tx_count_equals0(
        &mut self,
    ) -> DisDmaautoRecoveryWhenResidualTxCountEquals0W<I2cc00Spec> {
        DisDmaautoRecoveryWhenResidualTxCountEquals0W::new(self, 21)
    }
    #[doc = "Bit 22 - newverEnable RB early done"]
    #[inline(always)]
    pub fn enbl_rbearly_done(&mut self) -> EnblRbearlyDoneW<I2cc00Spec> {
        EnblRbearlyDoneW::new(self, 22)
    }
}
#[doc = "Master/Slave Function Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc00::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc00::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc00Spec;
impl crate::RegisterSpec for I2cc00Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc00::R`](R) reader structure"]
impl crate::Readable for I2cc00Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cc00::W`](W) writer structure"]
impl crate::Writable for I2cc00Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC00 to value 0"]
impl crate::Resettable for I2cc00Spec {}
