#[doc = "Register `SCU510` reader"]
pub type R = crate::R<Scu510Spec>;
#[doc = "Register `SCU510` writer"]
pub type W = crate::W<Scu510Spec>;
#[doc = "Field `Reserved4` reader - 32])"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `Reserved4` writer - 32])"]
pub type Reserved4W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DisDebugInterfacesOTPSTRAP36` reader - Disable Debug Interfaces (hlinkOTPSTRAP\\[36\\])"]
pub type DisDebugInterfacesOtpstrap36R = crate::BitReader;
#[doc = "Field `DisDebugInterfacesOTPSTRAP36` writer - Disable Debug Interfaces (hlinkOTPSTRAP\\[36\\])"]
pub type DisDebugInterfacesOtpstrap36W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - 37])"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - 37])"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EnblBootFromUart5ByPinStrapOTPSTRAP40OrFWSPICK` reader - Enable Boot from Uart5 by Pin Strap (hlinkOTPSTRAP\\[40\\] or FWSPICK)"]
pub type EnblBootFromUart5byPinStrapOtpstrap40orFwspickR = crate::BitReader;
#[doc = "Field `EnblBootFromUart5ByPinStrapOTPSTRAP40OrFWSPICK` writer - Enable Boot from Uart5 by Pin Strap (hlinkOTPSTRAP\\[40\\] or FWSPICK)"]
pub type EnblBootFromUart5byPinStrapOtpstrap40orFwspickW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - 41])"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - 41])"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EnblBootSPIABRSecondBoottrapenbspiabrOTPSTRAP43` reader - Enable boot SPI ABR (second boot)(trap_en_bspiabr) (hlinkOTPSTRAP\\[43\\])"]
pub type EnblBootSpiabrsecondBoottrapenbspiabrOtpstrap43R = crate::BitReader;
#[doc = "Field `EnblBootSPIABRSecondBoottrapenbspiabrOTPSTRAP43` writer - Enable boot SPI ABR (second boot)(trap_en_bspiabr) (hlinkOTPSTRAP\\[43\\])"]
pub type EnblBootSpiabrsecondBoottrapenbspiabrOtpstrap43W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BootSPIABRModeTrapbspiabrmodeOTPSTRAP44` reader - Boot SPI ABR Mode (trap_bspi_abrmode) (hlinkOTPSTRAP\\[44\\])"]
pub type BootSpiabrmodeTrapbspiabrmodeOtpstrap44R = crate::BitReader;
#[doc = "Field `BootSPIABRModeTrapbspiabrmodeOTPSTRAP44` writer - Boot SPI ABR Mode (trap_bspi_abrmode) (hlinkOTPSTRAP\\[44\\])"]
pub type BootSpiabrmodeTrapbspiabrmodeOtpstrap44W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BootSPIFlashSizeTrapbspisize2` reader - 0\\]) (hlinkOTPSTRAP\\[47:45\\], FWSPICK, GPION2,GPION0)"]
pub type BootSpiflashSizeTrapbspisize2R = crate::FieldReader;
#[doc = "Field `BootSPIFlashSizeTrapbspisize2` writer - 0\\]) (hlinkOTPSTRAP\\[47:45\\], FWSPICK, GPION2,GPION0)"]
pub type BootSpiflashSizeTrapbspisize2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - 48])"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - 48])"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EnblBootSPIAuxiliaryCtrlPinsFWSPIDQ2FWSPIDQ3FWSPIABRFWSPIWPOTPSTRAP54` reader - Enable boot SPI auxiliary control pins FWSPIDQ2, FWSPIDQ3, FWSPIABR#, FWSPIWP# (hlinkOTPSTRAP\\[54\\])"]
pub type EnblBootSpiauxiliaryCtrlPinsFwspidq2fwspidq3fwspiabrfwspiwpotpstrap54R = crate::BitReader;
#[doc = "Field `EnblBootSPIAuxiliaryCtrlPinsFWSPIDQ2FWSPIDQ3FWSPIABRFWSPIWPOTPSTRAP54` writer - Enable boot SPI auxiliary control pins FWSPIDQ2, FWSPIDQ3, FWSPIABR#, FWSPIWP# (hlinkOTPSTRAP\\[54\\])"]
pub type EnblBootSpiauxiliaryCtrlPinsFwspidq2fwspidq3fwspiabrfwspiwpotpstrap54W<'a, REG> =
    crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDedicateGPIOStrapPinsOTPSTRAP62` reader - Enable Dedicate GPIO Strap Pins (hlinkOTPSTRAP\\[62\\])"]
pub type EnblDedicateGpiostrapPinsOtpstrap62R = crate::BitReader;
#[doc = "Field `EnblDedicateGPIOStrapPinsOTPSTRAP62` writer - Enable Dedicate GPIO Strap Pins (hlinkOTPSTRAP\\[62\\])"]
pub type EnblDedicateGpiostrapPinsOtpstrap62W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblSecBootByPinStrapGPIOM0` reader - Enable Secure Boot by Pin Strap (GPIOM0)"]
pub type EnblSecBootByPinStrapGpiom0R = crate::BitReader;
#[doc = "Field `EnblSecBootByPinStrapGPIOM0` writer - Enable Secure Boot by Pin Strap (GPIOM0)"]
pub type EnblSecBootByPinStrapGpiom0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 32])"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Disable Debug Interfaces (hlinkOTPSTRAP\\[36\\])"]
    #[inline(always)]
    pub fn dis_debug_interfaces_otpstrap36(&self) -> DisDebugInterfacesOtpstrap36R {
        DisDebugInterfacesOtpstrap36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - 37])"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Enable Boot from Uart5 by Pin Strap (hlinkOTPSTRAP\\[40\\] or FWSPICK)"]
    #[inline(always)]
    pub fn enbl_boot_from_uart5by_pin_strap_otpstrap40or_fwspick(
        &self,
    ) -> EnblBootFromUart5byPinStrapOtpstrap40orFwspickR {
        EnblBootFromUart5byPinStrapOtpstrap40orFwspickR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - 41])"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bit 11 - Enable boot SPI ABR (second boot)(trap_en_bspiabr) (hlinkOTPSTRAP\\[43\\])"]
    #[inline(always)]
    pub fn enbl_boot_spiabrsecond_boottrapenbspiabr_otpstrap43(
        &self,
    ) -> EnblBootSpiabrsecondBoottrapenbspiabrOtpstrap43R {
        EnblBootSpiabrsecondBoottrapenbspiabrOtpstrap43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Boot SPI ABR Mode (trap_bspi_abrmode) (hlinkOTPSTRAP\\[44\\])"]
    #[inline(always)]
    pub fn boot_spiabrmode_trapbspiabrmode_otpstrap44(
        &self,
    ) -> BootSpiabrmodeTrapbspiabrmodeOtpstrap44R {
        BootSpiabrmodeTrapbspiabrmodeOtpstrap44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - 0\\]) (hlinkOTPSTRAP\\[47:45\\], FWSPICK, GPION2,GPION0)"]
    #[inline(always)]
    pub fn boot_spiflash_size_trapbspisize2(&self) -> BootSpiflashSizeTrapbspisize2R {
        BootSpiflashSizeTrapbspisize2R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:21 - 48])"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 22 - Enable boot SPI auxiliary control pins FWSPIDQ2, FWSPIDQ3, FWSPIABR#, FWSPIWP# (hlinkOTPSTRAP\\[54\\])"]
    #[inline(always)]
    pub fn enbl_boot_spiauxiliary_ctrl_pins_fwspidq2fwspidq3fwspiabrfwspiwpotpstrap54(
        &self,
    ) -> EnblBootSpiauxiliaryCtrlPinsFwspidq2fwspidq3fwspiabrfwspiwpotpstrap54R {
        EnblBootSpiauxiliaryCtrlPinsFwspidq2fwspidq3fwspiabrfwspiwpotpstrap54R::new(
            ((self.bits >> 22) & 1) != 0,
        )
    }
    #[doc = "Bit 30 - Enable Dedicate GPIO Strap Pins (hlinkOTPSTRAP\\[62\\])"]
    #[inline(always)]
    pub fn enbl_dedicate_gpiostrap_pins_otpstrap62(&self) -> EnblDedicateGpiostrapPinsOtpstrap62R {
        EnblDedicateGpiostrapPinsOtpstrap62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Secure Boot by Pin Strap (GPIOM0)"]
    #[inline(always)]
    pub fn enbl_sec_boot_by_pin_strap_gpiom0(&self) -> EnblSecBootByPinStrapGpiom0R {
        EnblSecBootByPinStrapGpiom0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 32])"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<Scu510Spec> {
        Reserved4W::new(self, 0)
    }
    #[doc = "Bit 4 - Disable Debug Interfaces (hlinkOTPSTRAP\\[36\\])"]
    #[inline(always)]
    pub fn dis_debug_interfaces_otpstrap36(&mut self) -> DisDebugInterfacesOtpstrap36W<Scu510Spec> {
        DisDebugInterfacesOtpstrap36W::new(self, 4)
    }
    #[doc = "Bits 5:7 - 37])"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu510Spec> {
        Reserved3W::new(self, 5)
    }
    #[doc = "Bit 8 - Enable Boot from Uart5 by Pin Strap (hlinkOTPSTRAP\\[40\\] or FWSPICK)"]
    #[inline(always)]
    pub fn enbl_boot_from_uart5by_pin_strap_otpstrap40or_fwspick(
        &mut self,
    ) -> EnblBootFromUart5byPinStrapOtpstrap40orFwspickW<Scu510Spec> {
        EnblBootFromUart5byPinStrapOtpstrap40orFwspickW::new(self, 8)
    }
    #[doc = "Bits 9:10 - 41])"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu510Spec> {
        Reserved2W::new(self, 9)
    }
    #[doc = "Bit 11 - Enable boot SPI ABR (second boot)(trap_en_bspiabr) (hlinkOTPSTRAP\\[43\\])"]
    #[inline(always)]
    pub fn enbl_boot_spiabrsecond_boottrapenbspiabr_otpstrap43(
        &mut self,
    ) -> EnblBootSpiabrsecondBoottrapenbspiabrOtpstrap43W<Scu510Spec> {
        EnblBootSpiabrsecondBoottrapenbspiabrOtpstrap43W::new(self, 11)
    }
    #[doc = "Bit 12 - Boot SPI ABR Mode (trap_bspi_abrmode) (hlinkOTPSTRAP\\[44\\])"]
    #[inline(always)]
    pub fn boot_spiabrmode_trapbspiabrmode_otpstrap44(
        &mut self,
    ) -> BootSpiabrmodeTrapbspiabrmodeOtpstrap44W<Scu510Spec> {
        BootSpiabrmodeTrapbspiabrmodeOtpstrap44W::new(self, 12)
    }
    #[doc = "Bits 13:15 - 0\\]) (hlinkOTPSTRAP\\[47:45\\], FWSPICK, GPION2,GPION0)"]
    #[inline(always)]
    pub fn boot_spiflash_size_trapbspisize2(
        &mut self,
    ) -> BootSpiflashSizeTrapbspisize2W<Scu510Spec> {
        BootSpiflashSizeTrapbspisize2W::new(self, 13)
    }
    #[doc = "Bits 16:21 - 48])"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu510Spec> {
        Reserved1W::new(self, 16)
    }
    #[doc = "Bit 22 - Enable boot SPI auxiliary control pins FWSPIDQ2, FWSPIDQ3, FWSPIABR#, FWSPIWP# (hlinkOTPSTRAP\\[54\\])"]
    #[inline(always)]
    pub fn enbl_boot_spiauxiliary_ctrl_pins_fwspidq2fwspidq3fwspiabrfwspiwpotpstrap54(
        &mut self,
    ) -> EnblBootSpiauxiliaryCtrlPinsFwspidq2fwspidq3fwspiabrfwspiwpotpstrap54W<Scu510Spec> {
        EnblBootSpiauxiliaryCtrlPinsFwspidq2fwspidq3fwspiabrfwspiwpotpstrap54W::new(self, 22)
    }
    #[doc = "Bit 30 - Enable Dedicate GPIO Strap Pins (hlinkOTPSTRAP\\[62\\])"]
    #[inline(always)]
    pub fn enbl_dedicate_gpiostrap_pins_otpstrap62(
        &mut self,
    ) -> EnblDedicateGpiostrapPinsOtpstrap62W<Scu510Spec> {
        EnblDedicateGpiostrapPinsOtpstrap62W::new(self, 30)
    }
    #[doc = "Bit 31 - Enable Secure Boot by Pin Strap (GPIOM0)"]
    #[inline(always)]
    pub fn enbl_sec_boot_by_pin_strap_gpiom0(
        &mut self,
    ) -> EnblSecBootByPinStrapGpiom0W<Scu510Spec> {
        EnblSecBootByPinStrapGpiom0W::new(self, 31)
    }
}
#[doc = "Hardware Strap2 Register\\label{SCUREG:HWTRAPH\n\nYou can [`read`](crate::Reg::read) this register and get [`scu510::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu510::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu510Spec;
impl crate::RegisterSpec for Scu510Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu510::R`](R) reader structure"]
impl crate::Readable for Scu510Spec {}
#[doc = "`write(|w| ..)` method takes [`scu510::W`](W) writer structure"]
impl crate::Writable for Scu510Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU510 to value 0"]
impl crate::Resettable for Scu510Spec {}
