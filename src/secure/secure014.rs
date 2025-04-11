#[doc = "Register `SECURE014` reader"]
pub type R = crate::R<Secure014Spec>;
#[doc = "Register `SECURE014` writer"]
pub type W = crate::W<Secure014Spec>;
#[doc = "OTP compare status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpcompareSts {
    #[doc = "0: Quad DW read compare fail"]
    QuadDwReadCompareFail = 0,
    #[doc = "1: Quad DW read compare pass"]
    QuadDwReadComparePass = 1,
}
impl From<OtpcompareSts> for bool {
    #[inline(always)]
    fn from(variant: OtpcompareSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPCompareSts` reader - OTP compare status"]
pub type OtpcompareStsR = crate::BitReader<OtpcompareSts>;
impl OtpcompareStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpcompareSts {
        match self.bits {
            false => OtpcompareSts::QuadDwReadCompareFail,
            true => OtpcompareSts::QuadDwReadComparePass,
        }
    }
    #[doc = "Quad DW read compare fail"]
    #[inline(always)]
    pub fn is_quad_dw_read_compare_fail(&self) -> bool {
        *self == OtpcompareSts::QuadDwReadCompareFail
    }
    #[doc = "Quad DW read compare pass"]
    #[inline(always)]
    pub fn is_quad_dw_read_compare_pass(&self) -> bool {
        *self == OtpcompareSts::QuadDwReadComparePass
    }
}
#[doc = "OTP memory status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpmemorySts {
    #[doc = "0: OTP memory busy"]
    OtpMemoryBusy = 0,
    #[doc = "1: OTP memory idle"]
    OtpMemoryIdle = 1,
}
impl From<OtpmemorySts> for bool {
    #[inline(always)]
    fn from(variant: OtpmemorySts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPMemorySts` reader - OTP memory status"]
pub type OtpmemoryStsR = crate::BitReader<OtpmemorySts>;
impl OtpmemoryStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpmemorySts {
        match self.bits {
            false => OtpmemorySts::OtpMemoryBusy,
            true => OtpmemorySts::OtpMemoryIdle,
        }
    }
    #[doc = "OTP memory busy"]
    #[inline(always)]
    pub fn is_otp_memory_busy(&self) -> bool {
        *self == OtpmemorySts::OtpMemoryBusy
    }
    #[doc = "OTP memory idle"]
    #[inline(always)]
    pub fn is_otp_memory_idle(&self) -> bool {
        *self == OtpmemorySts::OtpMemoryIdle
    }
}
#[doc = "OTP controller status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpctrlSts {
    #[doc = "0: OTP controller busy"]
    OtpControllerBusy = 0,
    #[doc = "1: OTP controller idle"]
    OtpControllerIdle = 1,
}
impl From<OtpctrlSts> for bool {
    #[inline(always)]
    fn from(variant: OtpctrlSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPCtrlSts` reader - OTP controller status"]
pub type OtpctrlStsR = crate::BitReader<OtpctrlSts>;
impl OtpctrlStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpctrlSts {
        match self.bits {
            false => OtpctrlSts::OtpControllerBusy,
            true => OtpctrlSts::OtpControllerIdle,
        }
    }
    #[doc = "OTP controller busy"]
    #[inline(always)]
    pub fn is_otp_controller_busy(&self) -> bool {
        *self == OtpctrlSts::OtpControllerBusy
    }
    #[doc = "OTP controller idle"]
    #[inline(always)]
    pub fn is_otp_controller_idle(&self) -> bool {
        *self == OtpctrlSts::OtpControllerIdle
    }
}
#[doc = "OTP internal charge pump status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpintChargePumpSts {
    #[doc = "0: OTP internal charge pump not ready"]
    OtpInternalChargePumpNotReady = 0,
    #[doc = "1: OTP internal charge pump ready"]
    OtpInternalChargePumpReady = 1,
}
impl From<OtpintChargePumpSts> for bool {
    #[inline(always)]
    fn from(variant: OtpintChargePumpSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPIntChargePumpSts` reader - OTP internal charge pump status"]
pub type OtpintChargePumpStsR = crate::BitReader<OtpintChargePumpSts>;
impl OtpintChargePumpStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpintChargePumpSts {
        match self.bits {
            false => OtpintChargePumpSts::OtpInternalChargePumpNotReady,
            true => OtpintChargePumpSts::OtpInternalChargePumpReady,
        }
    }
    #[doc = "OTP internal charge pump not ready"]
    #[inline(always)]
    pub fn is_otp_internal_charge_pump_not_ready(&self) -> bool {
        *self == OtpintChargePumpSts::OtpInternalChargePumpNotReady
    }
    #[doc = "OTP internal charge pump ready"]
    #[inline(always)]
    pub fn is_otp_internal_charge_pump_ready(&self) -> bool {
        *self == OtpintChargePumpSts::OtpInternalChargePumpReady
    }
}
#[doc = "RSA engine status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RsaengSts {
    #[doc = "0: RSA engine busy"]
    RsaEngineBusy = 0,
    #[doc = "1: RSA engine ready"]
    RsaEngineReady = 1,
}
impl From<RsaengSts> for bool {
    #[inline(always)]
    fn from(variant: RsaengSts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSAEngSts` reader - RSA engine status"]
pub type RsaengStsR = crate::BitReader<RsaengSts>;
impl RsaengStsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RsaengSts {
        match self.bits {
            false => RsaengSts::RsaEngineBusy,
            true => RsaengSts::RsaEngineReady,
        }
    }
    #[doc = "RSA engine busy"]
    #[inline(always)]
    pub fn is_rsa_engine_busy(&self) -> bool {
        *self == RsaengSts::RsaEngineBusy
    }
    #[doc = "RSA engine ready"]
    #[inline(always)]
    pub fn is_rsa_engine_ready(&self) -> bool {
        *self == RsaengSts::RsaEngineReady
    }
}
#[doc = "ewverBoot from Uart Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EwverBootFromUartMode {
    #[doc = "0: Boot from Uart mode"]
    BootFromUartMode = 0,
    #[doc = "1: Boot from SPI or eMMC mode"]
    BootFromSpiOrEmmcMode = 1,
}
impl From<EwverBootFromUartMode> for bool {
    #[inline(always)]
    fn from(variant: EwverBootFromUartMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EwverBootFromUartMode` reader - ewverBoot from Uart Mode"]
pub type EwverBootFromUartModeR = crate::BitReader<EwverBootFromUartMode>;
impl EwverBootFromUartModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EwverBootFromUartMode {
        match self.bits {
            false => EwverBootFromUartMode::BootFromUartMode,
            true => EwverBootFromUartMode::BootFromSpiOrEmmcMode,
        }
    }
    #[doc = "Boot from Uart mode"]
    #[inline(always)]
    pub fn is_boot_from_uart_mode(&self) -> bool {
        *self == EwverBootFromUartMode::BootFromUartMode
    }
    #[doc = "Boot from SPI or eMMC mode"]
    #[inline(always)]
    pub fn is_boot_from_spi_or_emmc_mode(&self) -> bool {
        *self == EwverBootFromUartMode::BootFromSpiOrEmmcMode
    }
}
#[doc = "Secure Boot Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecBootMode {
    #[doc = "0: Normal Mode"]
    NormalMode = 0,
    #[doc = "1: Secure Boot Mode is enabled"]
    SecureBootModeIsEnabled = 1,
}
impl From<SecBootMode> for bool {
    #[inline(always)]
    fn from(variant: SecBootMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecBootMode` reader - Secure Boot Mode"]
pub type SecBootModeR = crate::BitReader<SecBootMode>;
impl SecBootModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecBootMode {
        match self.bits {
            false => SecBootMode::NormalMode,
            true => SecBootMode::SecureBootModeIsEnabled,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == SecBootMode::NormalMode
    }
    #[doc = "Secure Boot Mode is enabled"]
    #[inline(always)]
    pub fn is_secure_boot_mode_is_enabled(&self) -> bool {
        *self == SecBootMode::SecureBootModeIsEnabled
    }
}
#[doc = "Low Security Key Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LowSecurityKeyMode {
    #[doc = "0: Normal Mode"]
    NormalMode = 0,
    #[doc = "1: Low Security Mode is enabled"]
    LowSecurityModeIsEnabled = 1,
}
impl From<LowSecurityKeyMode> for bool {
    #[inline(always)]
    fn from(variant: LowSecurityKeyMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LowSecurityKeyMode` reader - Low Security Key Mode"]
pub type LowSecurityKeyModeR = crate::BitReader<LowSecurityKeyMode>;
impl LowSecurityKeyModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LowSecurityKeyMode {
        match self.bits {
            false => LowSecurityKeyMode::NormalMode,
            true => LowSecurityKeyMode::LowSecurityModeIsEnabled,
        }
    }
    #[doc = "Normal Mode"]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == LowSecurityKeyMode::NormalMode
    }
    #[doc = "Low Security Mode is enabled"]
    #[inline(always)]
    pub fn is_low_security_mode_is_enabled(&self) -> bool {
        *self == LowSecurityKeyMode::LowSecurityModeIsEnabled
    }
}
#[doc = "OTP Program Protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpprogramProtect {
    #[doc = "0: SEC10 OTP address is not protected"]
    Sec10OtpAddressIsNotProtected = 0,
    #[doc = "1: SEC10 OTP address is write protected"]
    Sec10OtpAddressIsWriteProtected = 1,
}
impl From<OtpprogramProtect> for bool {
    #[inline(always)]
    fn from(variant: OtpprogramProtect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPProgramProtect` reader - OTP Program Protect"]
pub type OtpprogramProtectR = crate::BitReader<OtpprogramProtect>;
impl OtpprogramProtectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpprogramProtect {
        match self.bits {
            false => OtpprogramProtect::Sec10OtpAddressIsNotProtected,
            true => OtpprogramProtect::Sec10OtpAddressIsWriteProtected,
        }
    }
    #[doc = "SEC10 OTP address is not protected"]
    #[inline(always)]
    pub fn is_sec10_otp_address_is_not_protected(&self) -> bool {
        *self == OtpprogramProtect::Sec10OtpAddressIsNotProtected
    }
    #[doc = "SEC10 OTP address is write protected"]
    #[inline(always)]
    pub fn is_sec10_otp_address_is_write_protected(&self) -> bool {
        *self == OtpprogramProtect::Sec10OtpAddressIsWriteProtected
    }
}
#[doc = "OTP Program Protected\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtpprogramProtected {
    #[doc = "0: Last OTP program command is completed"]
    LastOtpProgramCommandIsCompleted = 0,
    #[doc = "1: Last OTP program command is write protected"]
    LastOtpProgramCommandIsWriteProtected = 1,
}
impl From<OtpprogramProtected> for bool {
    #[inline(always)]
    fn from(variant: OtpprogramProtected) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTPProgramProtected` reader - OTP Program Protected"]
pub type OtpprogramProtectedR = crate::BitReader<OtpprogramProtected>;
impl OtpprogramProtectedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtpprogramProtected {
        match self.bits {
            false => OtpprogramProtected::LastOtpProgramCommandIsCompleted,
            true => OtpprogramProtected::LastOtpProgramCommandIsWriteProtected,
        }
    }
    #[doc = "Last OTP program command is completed"]
    #[inline(always)]
    pub fn is_last_otp_program_command_is_completed(&self) -> bool {
        *self == OtpprogramProtected::LastOtpProgramCommandIsCompleted
    }
    #[doc = "Last OTP program command is write protected"]
    #[inline(always)]
    pub fn is_last_otp_program_command_is_write_protected(&self) -> bool {
        *self == OtpprogramProtected::LastOtpProgramCommandIsWriteProtected
    }
}
#[doc = "Secure Boot Crypto Engine Busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecBootCryptoEngBusy {
    #[doc = "0: Crypto Engine is idle"]
    CryptoEngineIsIdle = 0,
    #[doc = "1: Crypto Engine is busy"]
    CryptoEngineIsBusy = 1,
}
impl From<SecBootCryptoEngBusy> for bool {
    #[inline(always)]
    fn from(variant: SecBootCryptoEngBusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecBootCryptoEngBusy` reader - Secure Boot Crypto Engine Busy"]
pub type SecBootCryptoEngBusyR = crate::BitReader<SecBootCryptoEngBusy>;
impl SecBootCryptoEngBusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecBootCryptoEngBusy {
        match self.bits {
            false => SecBootCryptoEngBusy::CryptoEngineIsIdle,
            true => SecBootCryptoEngBusy::CryptoEngineIsBusy,
        }
    }
    #[doc = "Crypto Engine is idle"]
    #[inline(always)]
    pub fn is_crypto_engine_is_idle(&self) -> bool {
        *self == SecBootCryptoEngBusy::CryptoEngineIsIdle
    }
    #[doc = "Crypto Engine is busy"]
    #[inline(always)]
    pub fn is_crypto_engine_is_busy(&self) -> bool {
        *self == SecBootCryptoEngBusy::CryptoEngineIsBusy
    }
}
#[doc = "Secure Boot Crypto Engine Key Expansion Done\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecBootCryptoEngKeyExpansionDone {
    #[doc = "0: Crypto Engine Key Expansion is busy"]
    CryptoEngineKeyExpansionIsBusy = 0,
    #[doc = "1: Crypto Engine Key Expansion is done"]
    CryptoEngineKeyExpansionIsDone = 1,
}
impl From<SecBootCryptoEngKeyExpansionDone> for bool {
    #[inline(always)]
    fn from(variant: SecBootCryptoEngKeyExpansionDone) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SecBootCryptoEngKeyExpansionDone` reader - Secure Boot Crypto Engine Key Expansion Done"]
pub type SecBootCryptoEngKeyExpansionDoneR = crate::BitReader<SecBootCryptoEngKeyExpansionDone>;
impl SecBootCryptoEngKeyExpansionDoneR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SecBootCryptoEngKeyExpansionDone {
        match self.bits {
            false => SecBootCryptoEngKeyExpansionDone::CryptoEngineKeyExpansionIsBusy,
            true => SecBootCryptoEngKeyExpansionDone::CryptoEngineKeyExpansionIsDone,
        }
    }
    #[doc = "Crypto Engine Key Expansion is busy"]
    #[inline(always)]
    pub fn is_crypto_engine_key_expansion_is_busy(&self) -> bool {
        *self == SecBootCryptoEngKeyExpansionDone::CryptoEngineKeyExpansionIsBusy
    }
    #[doc = "Crypto Engine Key Expansion is done"]
    #[inline(always)]
    pub fn is_crypto_engine_key_expansion_is_done(&self) -> bool {
        *self == SecBootCryptoEngKeyExpansionDone::CryptoEngineKeyExpansionIsDone
    }
}
#[doc = "ewverABR Image Source when Boot from SPI\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EwverAbrimageSourceWhenBootFromSpi {
    #[doc = "0: First ABR Image Source when Boot from SPI"]
    FirstAbrImageSourceWhenBootFromSpi = 0,
    #[doc = "1: Second ABR Image Source when Boot from SPI"]
    SecondAbrImageSourceWhenBootFromSpi = 1,
}
impl From<EwverAbrimageSourceWhenBootFromSpi> for bool {
    #[inline(always)]
    fn from(variant: EwverAbrimageSourceWhenBootFromSpi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EwverABRImageSourceWhenBootFromSPI` reader - ewverABR Image Source when Boot from SPI"]
pub type EwverAbrimageSourceWhenBootFromSpiR = crate::BitReader<EwverAbrimageSourceWhenBootFromSpi>;
impl EwverAbrimageSourceWhenBootFromSpiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EwverAbrimageSourceWhenBootFromSpi {
        match self.bits {
            false => EwverAbrimageSourceWhenBootFromSpi::FirstAbrImageSourceWhenBootFromSpi,
            true => EwverAbrimageSourceWhenBootFromSpi::SecondAbrImageSourceWhenBootFromSpi,
        }
    }
    #[doc = "First ABR Image Source when Boot from SPI"]
    #[inline(always)]
    pub fn is_first_abr_image_source_when_boot_from_spi(&self) -> bool {
        *self == EwverAbrimageSourceWhenBootFromSpi::FirstAbrImageSourceWhenBootFromSpi
    }
    #[doc = "Second ABR Image Source when Boot from SPI"]
    #[inline(always)]
    pub fn is_second_abr_image_source_when_boot_from_spi(&self) -> bool {
        *self == EwverAbrimageSourceWhenBootFromSpi::SecondAbrImageSourceWhenBootFromSpi
    }
}
#[doc = "ewverABR Image Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EwverAbrimageSource {
    #[doc = "0: First ABR Image"]
    FirstAbrImage = 0,
    #[doc = "1: Second ABR Image"]
    SecondAbrImage = 1,
}
impl From<EwverAbrimageSource> for bool {
    #[inline(always)]
    fn from(variant: EwverAbrimageSource) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EwverABRImageSource` reader - ewverABR Image Source"]
pub type EwverAbrimageSourceR = crate::BitReader<EwverAbrimageSource>;
impl EwverAbrimageSourceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EwverAbrimageSource {
        match self.bits {
            false => EwverAbrimageSource::FirstAbrImage,
            true => EwverAbrimageSource::SecondAbrImage,
        }
    }
    #[doc = "First ABR Image"]
    #[inline(always)]
    pub fn is_first_abr_image(&self) -> bool {
        *self == EwverAbrimageSource::FirstAbrImage
    }
    #[doc = "Second ABR Image"]
    #[inline(always)]
    pub fn is_second_abr_image(&self) -> bool {
        *self == EwverAbrimageSource::SecondAbrImage
    }
}
#[doc = "Field `EwverEnblBootSPIOrEMMCABRSecondBoottrapenbspiabr` reader - ewverEnable boot SPI or eMMC ABR (second boot)(trap_en_bspiabr"]
pub type EwverEnblBootSpiorEmmcabrsecondBoottrapenbspiabrR = crate::BitReader;
#[doc = "Field `Reserved0` reader - Reserved(0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - OTP compare status"]
    #[inline(always)]
    pub fn otpcompare_sts(&self) -> OtpcompareStsR {
        OtpcompareStsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OTP memory status"]
    #[inline(always)]
    pub fn otpmemory_sts(&self) -> OtpmemoryStsR {
        OtpmemoryStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OTP controller status"]
    #[inline(always)]
    pub fn otpctrl_sts(&self) -> OtpctrlStsR {
        OtpctrlStsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OTP internal charge pump status"]
    #[inline(always)]
    pub fn otpint_charge_pump_sts(&self) -> OtpintChargePumpStsR {
        OtpintChargePumpStsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSA engine status"]
    #[inline(always)]
    pub fn rsaeng_sts(&self) -> RsaengStsR {
        RsaengStsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ewverBoot from Uart Mode"]
    #[inline(always)]
    pub fn ewver_boot_from_uart_mode(&self) -> EwverBootFromUartModeR {
        EwverBootFromUartModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Secure Boot Mode"]
    #[inline(always)]
    pub fn sec_boot_mode(&self) -> SecBootModeR {
        SecBootModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Low Security Key Mode"]
    #[inline(always)]
    pub fn low_security_key_mode(&self) -> LowSecurityKeyModeR {
        LowSecurityKeyModeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - OTP Program Protect"]
    #[inline(always)]
    pub fn otpprogram_protect(&self) -> OtpprogramProtectR {
        OtpprogramProtectR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - OTP Program Protected"]
    #[inline(always)]
    pub fn otpprogram_protected(&self) -> OtpprogramProtectedR {
        OtpprogramProtectedR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Secure Boot Crypto Engine Busy"]
    #[inline(always)]
    pub fn sec_boot_crypto_eng_busy(&self) -> SecBootCryptoEngBusyR {
        SecBootCryptoEngBusyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Secure Boot Crypto Engine Key Expansion Done"]
    #[inline(always)]
    pub fn sec_boot_crypto_eng_key_expansion_done(&self) -> SecBootCryptoEngKeyExpansionDoneR {
        SecBootCryptoEngKeyExpansionDoneR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ewverABR Image Source when Boot from SPI"]
    #[inline(always)]
    pub fn ewver_abrimage_source_when_boot_from_spi(&self) -> EwverAbrimageSourceWhenBootFromSpiR {
        EwverAbrimageSourceWhenBootFromSpiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ewverABR Image Source"]
    #[inline(always)]
    pub fn ewver_abrimage_source(&self) -> EwverAbrimageSourceR {
        EwverAbrimageSourceR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ewverEnable boot SPI or eMMC ABR (second boot)(trap_en_bspiabr"]
    #[inline(always)]
    pub fn ewver_enbl_boot_spior_emmcabrsecond_boottrapenbspiabr(
        &self,
    ) -> EwverEnblBootSpiorEmmcabrsecondBoottrapenbspiabrR {
        EwverEnblBootSpiorEmmcabrsecondBoottrapenbspiabrR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:31 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {}
#[doc = "Secure Engine Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`secure014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`secure014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Secure014Spec;
impl crate::RegisterSpec for Secure014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure014::R`](R) reader structure"]
impl crate::Readable for Secure014Spec {}
#[doc = "`write(|w| ..)` method takes [`secure014::W`](W) writer structure"]
impl crate::Writable for Secure014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SECURE014 to value 0"]
impl crate::Resettable for Secure014Spec {}
