#[doc = "Register `I2CC00` reader"]
pub type R = crate::R<I2cc00Spec>;
#[doc = "Register `I2CC00` writer"]
pub type W = crate::W<I2cc00Spec>;
#[doc = "Field `EnblMasterFn` reader - Enable master function"]
pub type EnblMasterFnR = crate::BitReader;
#[doc = "Field `EnblMasterFn` writer - Enable master function"]
pub type EnblMasterFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlaveFn` reader - Enable slave function"]
pub type EnblSlaveFnR = crate::BitReader;
#[doc = "Field `EnblSlaveFn` writer - Enable slave function"]
pub type EnblSlaveFnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheGenCallAddr00000000` reader - defeatureEnable I2C/SMBus to respond the General Call Address (0000_0000)"]
pub type DefeatureEnblI2csmbusToRespondTheGenCallAddr00000000R = crate::BitReader;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheGenCallAddr00000000` writer - defeatureEnable I2C/SMBus to respond the General Call Address (0000_0000)"]
pub type DefeatureEnblI2csmbusToRespondTheGenCallAddr00000000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheARPHostAddr0001000` reader - defeatureEnable I2C/SMBus to respond the ARP Host Address (0001_000)"]
pub type DefeatureEnblI2csmbusToRespondTheArphostAddr0001000R = crate::BitReader;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheARPHostAddr0001000` writer - defeatureEnable I2C/SMBus to respond the ARP Host Address (0001_000)"]
pub type DefeatureEnblI2csmbusToRespondTheArphostAddr0001000W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheAlertAddr0001100` reader - defeatureEnable I2C/SMBus to respond the Alert Address (0001_100)"]
pub type DefeatureEnblI2csmbusToRespondTheAlertAddr0001100R = crate::BitReader;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheAlertAddr0001100` writer - defeatureEnable I2C/SMBus to respond the Alert Address (0001_100)"]
pub type DefeatureEnblI2csmbusToRespondTheAlertAddr0001100W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheDefaultAddr1100001` reader - defeatureEnable I2C/SMBus to respond the Default Address (1100_001)"]
pub type DefeatureEnblI2csmbusToRespondTheDefaultAddr1100001R = crate::BitReader;
#[doc = "Field `DefeatureEnblI2CSMBusToRespondTheDefaultAddr1100001` writer - defeatureEnable I2C/SMBus to respond the Default Address (1100_001)"]
pub type DefeatureEnblI2csmbusToRespondTheDefaultAddr1100001W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHighSpeedMasterMode` reader - Enable High Speed master mode"]
pub type EnblHighSpeedMasterModeR = crate::BitReader;
#[doc = "Field `EnblHighSpeedMasterMode` writer - Enable High Speed master mode"]
pub type EnblHighSpeedMasterModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSCLSigToDirDriveHighFor1TMasterOnly` reader - Enable SCL signal to direct drive high for 1T (Master Only)"]
pub type EnblSclsigToDirDriveHighFor1tmasterOnlyR = crate::BitReader;
#[doc = "Field `EnblSCLSigToDirDriveHighFor1TMasterOnly` writer - Enable SCL signal to direct drive high for 1T (Master Only)"]
pub type EnblSclsigToDirDriveHighFor1tmasterOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSDASigToDirDriveHighFor1T` reader - Enable SDA signal to direct drive high for 1T"]
pub type EnblSdasigToDirDriveHighFor1tR = crate::BitReader;
#[doc = "Field `EnblSDASigToDirDriveHighFor1T` writer - Enable SDA signal to direct drive high for 1T"]
pub type EnblSdasigToDirDriveHighFor1tW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSlaveAddrRangeMode` reader - newverEnable slave address range mode"]
pub type EnblSlaveAddrRangeModeR = crate::BitReader;
#[doc = "Field `EnblSlaveAddrRangeMode` writer - newverEnable slave address range mode"]
pub type EnblSlaveAddrRangeModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::FieldReader;
#[doc = "Field `EnblSCLDirDriveModeForMasterFnOnly` reader - Enable SCL direct drive mode (for master function only)"]
pub type EnblScldirDriveModeForMasterFnOnlyR = crate::BitReader;
#[doc = "Field `EnblSCLDirDriveModeForMasterFnOnly` writer - Enable SCL direct drive mode (for master function only)"]
pub type EnblScldirDriveModeForMasterFnOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DisMultimasterCapabilityForMasterFnOnly` reader - Disable multi-master capability (for master function only)"]
pub type DisMultimasterCapabilityForMasterFnOnlyR = crate::BitReader;
#[doc = "Field `DisMultimasterCapabilityForMasterFnOnly` writer - Disable multi-master capability (for master function only)"]
pub type DisMultimasterCapabilityForMasterFnOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblMasterAutoSDALockRecoveryCapabilityForSingleMasterCaseOnly` reader - Enable master auto SDA lock recovery capability (for single master case only)"]
pub type EnblMasterAutoSdalockRecoveryCapabilityForSingleMasterCaseOnlyR = crate::BitReader;
#[doc = "Field `EnblMasterAutoSDALockRecoveryCapabilityForSingleMasterCaseOnly` writer - Enable master auto SDA lock recovery capability (for single master case only)"]
pub type EnblMasterAutoSdalockRecoveryCapabilityForSingleMasterCaseOnlyW<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `EnblBusAutoreleaseWhenSCLLowSDALowOrSlaveModeInactiveTimeout` reader - Enable bus auto-release when SCL low, SDA low, or slave mode inactive timeout"]
pub type EnblBusAutoreleaseWhenScllowSdalowOrSlaveModeInactiveTimeoutR = crate::BitReader;
#[doc = "Field `EnblBusAutoreleaseWhenSCLLowSDALowOrSlaveModeInactiveTimeout` writer - Enable bus auto-release when SCL low, SDA low, or slave mode inactive timeout"]
pub type EnblBusAutoreleaseWhenScllowSdalowOrSlaveModeInactiveTimeoutW<'a, REG> =
    crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 0 - Enable master function"]
    #[inline(always)]
    pub fn enbl_master_fn(&self) -> EnblMasterFnR {
        EnblMasterFnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable slave function"]
    #[inline(always)]
    pub fn enbl_slave_fn(&self) -> EnblSlaveFnR {
        EnblSlaveFnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - defeatureEnable I2C/SMBus to respond the General Call Address (0000_0000)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_gen_call_addr00000000(
        &self,
    ) -> DefeatureEnblI2csmbusToRespondTheGenCallAddr00000000R {
        DefeatureEnblI2csmbusToRespondTheGenCallAddr00000000R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - defeatureEnable I2C/SMBus to respond the ARP Host Address (0001_000)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_arphost_addr0001000(
        &self,
    ) -> DefeatureEnblI2csmbusToRespondTheArphostAddr0001000R {
        DefeatureEnblI2csmbusToRespondTheArphostAddr0001000R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - defeatureEnable I2C/SMBus to respond the Alert Address (0001_100)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_alert_addr0001100(
        &self,
    ) -> DefeatureEnblI2csmbusToRespondTheAlertAddr0001100R {
        DefeatureEnblI2csmbusToRespondTheAlertAddr0001100R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - defeatureEnable I2C/SMBus to respond the Default Address (1100_001)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_default_addr1100001(
        &self,
    ) -> DefeatureEnblI2csmbusToRespondTheDefaultAddr1100001R {
        DefeatureEnblI2csmbusToRespondTheDefaultAddr1100001R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable High Speed master mode"]
    #[inline(always)]
    pub fn enbl_high_speed_master_mode(&self) -> EnblHighSpeedMasterModeR {
        EnblHighSpeedMasterModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable SCL signal to direct drive high for 1T (Master Only)"]
    #[inline(always)]
    pub fn enbl_sclsig_to_dir_drive_high_for1tmaster_only(
        &self,
    ) -> EnblSclsigToDirDriveHighFor1tmasterOnlyR {
        EnblSclsigToDirDriveHighFor1tmasterOnlyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable SDA signal to direct drive high for 1T"]
    #[inline(always)]
    pub fn enbl_sdasig_to_dir_drive_high_for1t(&self) -> EnblSdasigToDirDriveHighFor1tR {
        EnblSdasigToDirDriveHighFor1tR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - newverEnable slave address range mode"]
    #[inline(always)]
    pub fn enbl_slave_addr_range_mode(&self) -> EnblSlaveAddrRangeModeR {
        EnblSlaveAddrRangeModeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Enable SCL direct drive mode (for master function only)"]
    #[inline(always)]
    pub fn enbl_scldir_drive_mode_for_master_fn_only(&self) -> EnblScldirDriveModeForMasterFnOnlyR {
        EnblScldirDriveModeForMasterFnOnlyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disable multi-master capability (for master function only)"]
    #[inline(always)]
    pub fn dis_multimaster_capability_for_master_fn_only(
        &self,
    ) -> DisMultimasterCapabilityForMasterFnOnlyR {
        DisMultimasterCapabilityForMasterFnOnlyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable master auto SDA lock recovery capability (for single master case only)"]
    #[inline(always)]
    pub fn enbl_master_auto_sdalock_recovery_capability_for_single_master_case_only(
        &self,
    ) -> EnblMasterAutoSdalockRecoveryCapabilityForSingleMasterCaseOnlyR {
        EnblMasterAutoSdalockRecoveryCapabilityForSingleMasterCaseOnlyR::new(
            ((self.bits >> 16) & 1) != 0,
        )
    }
    #[doc = "Bit 17 - Enable bus auto-release when SCL low, SDA low, or slave mode inactive timeout"]
    #[inline(always)]
    pub fn enbl_bus_autorelease_when_scllow_sdalow_or_slave_mode_inactive_timeout(
        &self,
    ) -> EnblBusAutoreleaseWhenScllowSdalowOrSlaveModeInactiveTimeoutR {
        EnblBusAutoreleaseWhenScllowSdalowOrSlaveModeInactiveTimeoutR::new(
            ((self.bits >> 17) & 1) != 0,
        )
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
    #[doc = "Bit 0 - Enable master function"]
    #[inline(always)]
    pub fn enbl_master_fn(&mut self) -> EnblMasterFnW<I2cc00Spec> {
        EnblMasterFnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable slave function"]
    #[inline(always)]
    pub fn enbl_slave_fn(&mut self) -> EnblSlaveFnW<I2cc00Spec> {
        EnblSlaveFnW::new(self, 1)
    }
    #[doc = "Bit 2 - defeatureEnable I2C/SMBus to respond the General Call Address (0000_0000)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_gen_call_addr00000000(
        &mut self,
    ) -> DefeatureEnblI2csmbusToRespondTheGenCallAddr00000000W<I2cc00Spec> {
        DefeatureEnblI2csmbusToRespondTheGenCallAddr00000000W::new(self, 2)
    }
    #[doc = "Bit 3 - defeatureEnable I2C/SMBus to respond the ARP Host Address (0001_000)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_arphost_addr0001000(
        &mut self,
    ) -> DefeatureEnblI2csmbusToRespondTheArphostAddr0001000W<I2cc00Spec> {
        DefeatureEnblI2csmbusToRespondTheArphostAddr0001000W::new(self, 3)
    }
    #[doc = "Bit 4 - defeatureEnable I2C/SMBus to respond the Alert Address (0001_100)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_alert_addr0001100(
        &mut self,
    ) -> DefeatureEnblI2csmbusToRespondTheAlertAddr0001100W<I2cc00Spec> {
        DefeatureEnblI2csmbusToRespondTheAlertAddr0001100W::new(self, 4)
    }
    #[doc = "Bit 5 - defeatureEnable I2C/SMBus to respond the Default Address (1100_001)"]
    #[inline(always)]
    pub fn defeature_enbl_i2csmbus_to_respond_the_default_addr1100001(
        &mut self,
    ) -> DefeatureEnblI2csmbusToRespondTheDefaultAddr1100001W<I2cc00Spec> {
        DefeatureEnblI2csmbusToRespondTheDefaultAddr1100001W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable High Speed master mode"]
    #[inline(always)]
    pub fn enbl_high_speed_master_mode(&mut self) -> EnblHighSpeedMasterModeW<I2cc00Spec> {
        EnblHighSpeedMasterModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable SCL signal to direct drive high for 1T (Master Only)"]
    #[inline(always)]
    pub fn enbl_sclsig_to_dir_drive_high_for1tmaster_only(
        &mut self,
    ) -> EnblSclsigToDirDriveHighFor1tmasterOnlyW<I2cc00Spec> {
        EnblSclsigToDirDriveHighFor1tmasterOnlyW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable SDA signal to direct drive high for 1T"]
    #[inline(always)]
    pub fn enbl_sdasig_to_dir_drive_high_for1t(
        &mut self,
    ) -> EnblSdasigToDirDriveHighFor1tW<I2cc00Spec> {
        EnblSdasigToDirDriveHighFor1tW::new(self, 8)
    }
    #[doc = "Bit 9 - newverEnable slave address range mode"]
    #[inline(always)]
    pub fn enbl_slave_addr_range_mode(&mut self) -> EnblSlaveAddrRangeModeW<I2cc00Spec> {
        EnblSlaveAddrRangeModeW::new(self, 9)
    }
    #[doc = "Bit 14 - Enable SCL direct drive mode (for master function only)"]
    #[inline(always)]
    pub fn enbl_scldir_drive_mode_for_master_fn_only(
        &mut self,
    ) -> EnblScldirDriveModeForMasterFnOnlyW<I2cc00Spec> {
        EnblScldirDriveModeForMasterFnOnlyW::new(self, 14)
    }
    #[doc = "Bit 15 - Disable multi-master capability (for master function only)"]
    #[inline(always)]
    pub fn dis_multimaster_capability_for_master_fn_only(
        &mut self,
    ) -> DisMultimasterCapabilityForMasterFnOnlyW<I2cc00Spec> {
        DisMultimasterCapabilityForMasterFnOnlyW::new(self, 15)
    }
    #[doc = "Bit 16 - Enable master auto SDA lock recovery capability (for single master case only)"]
    #[inline(always)]
    pub fn enbl_master_auto_sdalock_recovery_capability_for_single_master_case_only(
        &mut self,
    ) -> EnblMasterAutoSdalockRecoveryCapabilityForSingleMasterCaseOnlyW<I2cc00Spec> {
        EnblMasterAutoSdalockRecoveryCapabilityForSingleMasterCaseOnlyW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable bus auto-release when SCL low, SDA low, or slave mode inactive timeout"]
    #[inline(always)]
    pub fn enbl_bus_autorelease_when_scllow_sdalow_or_slave_mode_inactive_timeout(
        &mut self,
    ) -> EnblBusAutoreleaseWhenScllowSdalowOrSlaveModeInactiveTimeoutW<I2cc00Spec> {
        EnblBusAutoreleaseWhenScllowSdalowOrSlaveModeInactiveTimeoutW::new(self, 17)
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
