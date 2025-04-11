#[doc = "Register `SCU074` reader"]
pub type R = crate::R<Scu074Spec>;
#[doc = "Register `SCU074` writer"]
pub type W = crate::W<Scu074Spec>;
#[doc = "Field `PowerOnRstSRSTEventLog` reader - Power on reset SRST# event log"]
pub type PowerOnRstSrsteventLogR = crate::BitReader;
#[doc = "Field `PowerOnRstSRSTEventLog` writer - Power on reset SRST# event log"]
pub type PowerOnRstSrsteventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ExtRstEXTRSTEventLog` reader - External reset EXTRST# event log"]
pub type ExtRstExtrsteventLogR = crate::BitReader;
#[doc = "Field `ExtRstEXTRSTEventLog` writer - External reset EXTRST# event log"]
pub type ExtRstExtrsteventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BootFlashABROrAdrSwap` reader - Boot flash reset event log -- ABR or AdrSwap"]
pub type BootFlashAbrorAdrSwapR = crate::BitReader;
#[doc = "Field `BootFlashABROrAdrSwap` writer - Boot flash reset event log -- ABR or AdrSwap"]
pub type BootFlashAbrorAdrSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved(0)"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PWRSTNinRPoweronStrapEnblSig` reader - Reset event log -- PWRSTNinR Power-on strap enable signal"]
pub type PwrstninRpoweronStrapEnblSigR = crate::BitReader;
#[doc = "Field `PWRSTNinRPoweronStrapEnblSig` writer - Reset event log -- PWRSTNinR Power-on strap enable signal"]
pub type PwrstninRpoweronStrapEnblSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTNinTrapPoweronStrapEnblSig` reader - Reset event log -- PWRSTNinTrap Power-on strap enable signal"]
pub type PwrstninTrapPoweronStrapEnblSigR = crate::BitReader;
#[doc = "Field `PWRSTNinTrapPoweronStrapEnblSig` writer - Reset event log -- PWRSTNinTrap Power-on strap enable signal"]
pub type PwrstninTrapPoweronStrapEnblSigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTNinPLLPLLPoweronRst` reader - Reset event log -- PWRSTNinPLL PLL power-on reset"]
pub type PwrstninPllpllpoweronRstR = crate::BitReader;
#[doc = "Field `PWRSTNinPLLPLLPoweronRst` writer - Reset event log -- PWRSTNinPLL PLL power-on reset"]
pub type PwrstninPllpllpoweronRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRSTNPoweronRst` reader - Reset event log -- PWRST_N Power-on reset"]
pub type PwrstnpoweronRstR = crate::BitReader;
#[doc = "Field `PWRSTNPoweronRst` writer - Reset event log -- PWRST_N Power-on reset"]
pub type PwrstnpoweronRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARMRSTNARMCM4FCPURst` reader - Reset event log -- ARMRST_N ARM CM4F CPU reset"]
pub type Armrstnarmcm4fcpurstR = crate::BitReader;
#[doc = "Field `ARMRSTNARMCM4FCPURst` writer - Reset event log -- ARMRST_N ARM CM4F CPU reset"]
pub type Armrstnarmcm4fcpurstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOCHRSTNAHBSOCRst` reader - Reset event log -- SOC_HRST_N AHB SOC reset"]
pub type SochrstnahbsocrstR = crate::BitReader;
#[doc = "Field `SOCHRSTNAHBSOCRst` writer - Reset event log -- SOC_HRST_N AHB SOC reset"]
pub type SochrstnahbsocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBHRSTNAHBBusCtrlRst` reader - Reset event log -- AHB_HRST_N AHB Bus Controller reset"]
pub type AhbhrstnahbbusCtrlRstR = crate::BitReader;
#[doc = "Field `AHBHRSTNAHBBusCtrlRst` writer - Reset event log -- AHB_HRST_N AHB Bus Controller reset"]
pub type AhbhrstnahbbusCtrlRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT1SOCRstEventLog` reader - WDT1 SOC Reset event log"]
pub type Wdt1socrstEventLogR = crate::BitReader;
#[doc = "Field `WDT1SOCRstEventLog` writer - WDT1 SOC Reset event log"]
pub type Wdt1socrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT1FullRstEventLog` reader - WDT1 Full Reset event log"]
pub type Wdt1fullRstEventLogR = crate::BitReader;
#[doc = "Field `WDT1FullRstEventLog` writer - WDT1 Full Reset event log"]
pub type Wdt1fullRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT1ARMRstEventLog` reader - WDT1 ARM Reset event log"]
pub type Wdt1armrstEventLogR = crate::BitReader;
#[doc = "Field `WDT1ARMRstEventLog` writer - WDT1 ARM Reset event log"]
pub type Wdt1armrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT1SwRstEventLog` reader - WDT1 Software Reset event log"]
pub type Wdt1swRstEventLogR = crate::BitReader;
#[doc = "Field `WDT1SwRstEventLog` writer - WDT1 Software Reset event log"]
pub type Wdt1swRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT2SOCRstEventLog` reader - WDT2 SOC Reset event log"]
pub type Wdt2socrstEventLogR = crate::BitReader;
#[doc = "Field `WDT2SOCRstEventLog` writer - WDT2 SOC Reset event log"]
pub type Wdt2socrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT2FullRstEventLog` reader - WDT2 Full Reset event log"]
pub type Wdt2fullRstEventLogR = crate::BitReader;
#[doc = "Field `WDT2FullRstEventLog` writer - WDT2 Full Reset event log"]
pub type Wdt2fullRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT2ARMRstEventLog` reader - WDT2 ARM Reset event log"]
pub type Wdt2armrstEventLogR = crate::BitReader;
#[doc = "Field `WDT2ARMRstEventLog` writer - WDT2 ARM Reset event log"]
pub type Wdt2armrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT2SwRstEventLog` reader - WDT2 Software Reset event log"]
pub type Wdt2swRstEventLogR = crate::BitReader;
#[doc = "Field `WDT2SwRstEventLog` writer - WDT2 Software Reset event log"]
pub type Wdt2swRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT3SOCRstEventLog` reader - WDT3 SOC Reset event log"]
pub type Wdt3socrstEventLogR = crate::BitReader;
#[doc = "Field `WDT3SOCRstEventLog` writer - WDT3 SOC Reset event log"]
pub type Wdt3socrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT3FullRstEventLog` reader - WDT3 Full Reset event log"]
pub type Wdt3fullRstEventLogR = crate::BitReader;
#[doc = "Field `WDT3FullRstEventLog` writer - WDT3 Full Reset event log"]
pub type Wdt3fullRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT3ARMRstEventLog` reader - WDT3 ARM Reset event log"]
pub type Wdt3armrstEventLogR = crate::BitReader;
#[doc = "Field `WDT3ARMRstEventLog` writer - WDT3 ARM Reset event log"]
pub type Wdt3armrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT3SwRstEventLog` reader - WDT3 Software Reset event log"]
pub type Wdt3swRstEventLogR = crate::BitReader;
#[doc = "Field `WDT3SwRstEventLog` writer - WDT3 Software Reset event log"]
pub type Wdt3swRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT4SOCRstEventLog` reader - WDT4 SOC Reset event log"]
pub type Wdt4socrstEventLogR = crate::BitReader;
#[doc = "Field `WDT4SOCRstEventLog` writer - WDT4 SOC Reset event log"]
pub type Wdt4socrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT4FullRstEventLog` reader - WDT4 Full Reset event log"]
pub type Wdt4fullRstEventLogR = crate::BitReader;
#[doc = "Field `WDT4FullRstEventLog` writer - WDT4 Full Reset event log"]
pub type Wdt4fullRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT4ARMRstEventLog` reader - WDT4 ARM Reset event log"]
pub type Wdt4armrstEventLogR = crate::BitReader;
#[doc = "Field `WDT4ARMRstEventLog` writer - WDT4 ARM Reset event log"]
pub type Wdt4armrstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDT4SwRstEventLog` reader - WDT4 Software Reset event log"]
pub type Wdt4swRstEventLogR = crate::BitReader;
#[doc = "Field `WDT4SwRstEventLog` writer - WDT4 Software Reset event log"]
pub type Wdt4swRstEventLogW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Power on reset SRST# event log"]
    #[inline(always)]
    pub fn power_on_rst_srstevent_log(&self) -> PowerOnRstSrsteventLogR {
        PowerOnRstSrsteventLogR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External reset EXTRST# event log"]
    #[inline(always)]
    pub fn ext_rst_extrstevent_log(&self) -> ExtRstExtrsteventLogR {
        ExtRstExtrsteventLogR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Boot flash reset event log -- ABR or AdrSwap"]
    #[inline(always)]
    pub fn boot_flash_abror_adr_swap(&self) -> BootFlashAbrorAdrSwapR {
        BootFlashAbrorAdrSwapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Reset event log -- PWRSTNinR Power-on strap enable signal"]
    #[inline(always)]
    pub fn pwrstnin_rpoweron_strap_enbl_sig(&self) -> PwrstninRpoweronStrapEnblSigR {
        PwrstninRpoweronStrapEnblSigR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset event log -- PWRSTNinTrap Power-on strap enable signal"]
    #[inline(always)]
    pub fn pwrstnin_trap_poweron_strap_enbl_sig(&self) -> PwrstninTrapPoweronStrapEnblSigR {
        PwrstninTrapPoweronStrapEnblSigR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset event log -- PWRSTNinPLL PLL power-on reset"]
    #[inline(always)]
    pub fn pwrstnin_pllpllpoweron_rst(&self) -> PwrstninPllpllpoweronRstR {
        PwrstninPllpllpoweronRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset event log -- PWRST_N Power-on reset"]
    #[inline(always)]
    pub fn pwrstnpoweron_rst(&self) -> PwrstnpoweronRstR {
        PwrstnpoweronRstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset event log -- ARMRST_N ARM CM4F CPU reset"]
    #[inline(always)]
    pub fn armrstnarmcm4fcpurst(&self) -> Armrstnarmcm4fcpurstR {
        Armrstnarmcm4fcpurstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset event log -- SOC_HRST_N AHB SOC reset"]
    #[inline(always)]
    pub fn sochrstnahbsocrst(&self) -> SochrstnahbsocrstR {
        SochrstnahbsocrstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset event log -- AHB_HRST_N AHB Bus Controller reset"]
    #[inline(always)]
    pub fn ahbhrstnahbbus_ctrl_rst(&self) -> AhbhrstnahbbusCtrlRstR {
        AhbhrstnahbbusCtrlRstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - WDT1 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt1socrst_event_log(&self) -> Wdt1socrstEventLogR {
        Wdt1socrstEventLogR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - WDT1 Full Reset event log"]
    #[inline(always)]
    pub fn wdt1full_rst_event_log(&self) -> Wdt1fullRstEventLogR {
        Wdt1fullRstEventLogR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - WDT1 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt1armrst_event_log(&self) -> Wdt1armrstEventLogR {
        Wdt1armrstEventLogR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - WDT1 Software Reset event log"]
    #[inline(always)]
    pub fn wdt1sw_rst_event_log(&self) -> Wdt1swRstEventLogR {
        Wdt1swRstEventLogR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - WDT2 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt2socrst_event_log(&self) -> Wdt2socrstEventLogR {
        Wdt2socrstEventLogR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - WDT2 Full Reset event log"]
    #[inline(always)]
    pub fn wdt2full_rst_event_log(&self) -> Wdt2fullRstEventLogR {
        Wdt2fullRstEventLogR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - WDT2 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt2armrst_event_log(&self) -> Wdt2armrstEventLogR {
        Wdt2armrstEventLogR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - WDT2 Software Reset event log"]
    #[inline(always)]
    pub fn wdt2sw_rst_event_log(&self) -> Wdt2swRstEventLogR {
        Wdt2swRstEventLogR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - WDT3 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt3socrst_event_log(&self) -> Wdt3socrstEventLogR {
        Wdt3socrstEventLogR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WDT3 Full Reset event log"]
    #[inline(always)]
    pub fn wdt3full_rst_event_log(&self) -> Wdt3fullRstEventLogR {
        Wdt3fullRstEventLogR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - WDT3 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt3armrst_event_log(&self) -> Wdt3armrstEventLogR {
        Wdt3armrstEventLogR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - WDT3 Software Reset event log"]
    #[inline(always)]
    pub fn wdt3sw_rst_event_log(&self) -> Wdt3swRstEventLogR {
        Wdt3swRstEventLogR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - WDT4 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt4socrst_event_log(&self) -> Wdt4socrstEventLogR {
        Wdt4socrstEventLogR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - WDT4 Full Reset event log"]
    #[inline(always)]
    pub fn wdt4full_rst_event_log(&self) -> Wdt4fullRstEventLogR {
        Wdt4fullRstEventLogR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - WDT4 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt4armrst_event_log(&self) -> Wdt4armrstEventLogR {
        Wdt4armrstEventLogR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - WDT4 Software Reset event log"]
    #[inline(always)]
    pub fn wdt4sw_rst_event_log(&self) -> Wdt4swRstEventLogR {
        Wdt4swRstEventLogR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power on reset SRST# event log"]
    #[inline(always)]
    pub fn power_on_rst_srstevent_log(&mut self) -> PowerOnRstSrsteventLogW<Scu074Spec> {
        PowerOnRstSrsteventLogW::new(self, 0)
    }
    #[doc = "Bit 1 - External reset EXTRST# event log"]
    #[inline(always)]
    pub fn ext_rst_extrstevent_log(&mut self) -> ExtRstExtrsteventLogW<Scu074Spec> {
        ExtRstExtrsteventLogW::new(self, 1)
    }
    #[doc = "Bit 2 - Boot flash reset event log -- ABR or AdrSwap"]
    #[inline(always)]
    pub fn boot_flash_abror_adr_swap(&mut self) -> BootFlashAbrorAdrSwapW<Scu074Spec> {
        BootFlashAbrorAdrSwapW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu074Spec> {
        Reserved2W::new(self, 4)
    }
    #[doc = "Bit 8 - Reset event log -- PWRSTNinR Power-on strap enable signal"]
    #[inline(always)]
    pub fn pwrstnin_rpoweron_strap_enbl_sig(
        &mut self,
    ) -> PwrstninRpoweronStrapEnblSigW<Scu074Spec> {
        PwrstninRpoweronStrapEnblSigW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset event log -- PWRSTNinTrap Power-on strap enable signal"]
    #[inline(always)]
    pub fn pwrstnin_trap_poweron_strap_enbl_sig(
        &mut self,
    ) -> PwrstninTrapPoweronStrapEnblSigW<Scu074Spec> {
        PwrstninTrapPoweronStrapEnblSigW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset event log -- PWRSTNinPLL PLL power-on reset"]
    #[inline(always)]
    pub fn pwrstnin_pllpllpoweron_rst(&mut self) -> PwrstninPllpllpoweronRstW<Scu074Spec> {
        PwrstninPllpllpoweronRstW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset event log -- PWRST_N Power-on reset"]
    #[inline(always)]
    pub fn pwrstnpoweron_rst(&mut self) -> PwrstnpoweronRstW<Scu074Spec> {
        PwrstnpoweronRstW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset event log -- ARMRST_N ARM CM4F CPU reset"]
    #[inline(always)]
    pub fn armrstnarmcm4fcpurst(&mut self) -> Armrstnarmcm4fcpurstW<Scu074Spec> {
        Armrstnarmcm4fcpurstW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset event log -- SOC_HRST_N AHB SOC reset"]
    #[inline(always)]
    pub fn sochrstnahbsocrst(&mut self) -> SochrstnahbsocrstW<Scu074Spec> {
        SochrstnahbsocrstW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset event log -- AHB_HRST_N AHB Bus Controller reset"]
    #[inline(always)]
    pub fn ahbhrstnahbbus_ctrl_rst(&mut self) -> AhbhrstnahbbusCtrlRstW<Scu074Spec> {
        AhbhrstnahbbusCtrlRstW::new(self, 14)
    }
    #[doc = "Bit 16 - WDT1 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt1socrst_event_log(&mut self) -> Wdt1socrstEventLogW<Scu074Spec> {
        Wdt1socrstEventLogW::new(self, 16)
    }
    #[doc = "Bit 17 - WDT1 Full Reset event log"]
    #[inline(always)]
    pub fn wdt1full_rst_event_log(&mut self) -> Wdt1fullRstEventLogW<Scu074Spec> {
        Wdt1fullRstEventLogW::new(self, 17)
    }
    #[doc = "Bit 18 - WDT1 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt1armrst_event_log(&mut self) -> Wdt1armrstEventLogW<Scu074Spec> {
        Wdt1armrstEventLogW::new(self, 18)
    }
    #[doc = "Bit 19 - WDT1 Software Reset event log"]
    #[inline(always)]
    pub fn wdt1sw_rst_event_log(&mut self) -> Wdt1swRstEventLogW<Scu074Spec> {
        Wdt1swRstEventLogW::new(self, 19)
    }
    #[doc = "Bit 20 - WDT2 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt2socrst_event_log(&mut self) -> Wdt2socrstEventLogW<Scu074Spec> {
        Wdt2socrstEventLogW::new(self, 20)
    }
    #[doc = "Bit 21 - WDT2 Full Reset event log"]
    #[inline(always)]
    pub fn wdt2full_rst_event_log(&mut self) -> Wdt2fullRstEventLogW<Scu074Spec> {
        Wdt2fullRstEventLogW::new(self, 21)
    }
    #[doc = "Bit 22 - WDT2 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt2armrst_event_log(&mut self) -> Wdt2armrstEventLogW<Scu074Spec> {
        Wdt2armrstEventLogW::new(self, 22)
    }
    #[doc = "Bit 23 - WDT2 Software Reset event log"]
    #[inline(always)]
    pub fn wdt2sw_rst_event_log(&mut self) -> Wdt2swRstEventLogW<Scu074Spec> {
        Wdt2swRstEventLogW::new(self, 23)
    }
    #[doc = "Bit 24 - WDT3 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt3socrst_event_log(&mut self) -> Wdt3socrstEventLogW<Scu074Spec> {
        Wdt3socrstEventLogW::new(self, 24)
    }
    #[doc = "Bit 25 - WDT3 Full Reset event log"]
    #[inline(always)]
    pub fn wdt3full_rst_event_log(&mut self) -> Wdt3fullRstEventLogW<Scu074Spec> {
        Wdt3fullRstEventLogW::new(self, 25)
    }
    #[doc = "Bit 26 - WDT3 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt3armrst_event_log(&mut self) -> Wdt3armrstEventLogW<Scu074Spec> {
        Wdt3armrstEventLogW::new(self, 26)
    }
    #[doc = "Bit 27 - WDT3 Software Reset event log"]
    #[inline(always)]
    pub fn wdt3sw_rst_event_log(&mut self) -> Wdt3swRstEventLogW<Scu074Spec> {
        Wdt3swRstEventLogW::new(self, 27)
    }
    #[doc = "Bit 28 - WDT4 SOC Reset event log"]
    #[inline(always)]
    pub fn wdt4socrst_event_log(&mut self) -> Wdt4socrstEventLogW<Scu074Spec> {
        Wdt4socrstEventLogW::new(self, 28)
    }
    #[doc = "Bit 29 - WDT4 Full Reset event log"]
    #[inline(always)]
    pub fn wdt4full_rst_event_log(&mut self) -> Wdt4fullRstEventLogW<Scu074Spec> {
        Wdt4fullRstEventLogW::new(self, 29)
    }
    #[doc = "Bit 30 - WDT4 ARM Reset event log"]
    #[inline(always)]
    pub fn wdt4armrst_event_log(&mut self) -> Wdt4armrstEventLogW<Scu074Spec> {
        Wdt4armrstEventLogW::new(self, 30)
    }
    #[doc = "Bit 31 - WDT4 Software Reset event log"]
    #[inline(always)]
    pub fn wdt4sw_rst_event_log(&mut self) -> Wdt4swRstEventLogW<Scu074Spec> {
        Wdt4swRstEventLogW::new(self, 31)
    }
}
#[doc = "System Reset Event Log Register Set 2-1\n\nYou can [`read`](crate::Reg::read) this register and get [`scu074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu074Spec;
impl crate::RegisterSpec for Scu074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu074::R`](R) reader structure"]
impl crate::Readable for Scu074Spec {}
#[doc = "`write(|w| ..)` method takes [`scu074::W`](W) writer structure"]
impl crate::Writable for Scu074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU074 to value 0xff31"]
impl crate::Resettable for Scu074Spec {
    const RESET_VALUE: u32 = 0xff31;
}
