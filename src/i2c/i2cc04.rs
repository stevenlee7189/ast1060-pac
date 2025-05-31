#[doc = "Register `I2CC04` reader"]
pub type R = crate::R<I2cc04Spec>;
#[doc = "Register `I2CC04` writer"]
pub type W = crate::W<I2cc04Spec>;
#[doc = "Field `BaseClkDivisorTBaseClk` reader - Base Clock divisor (tBaseClk)"]
pub type BaseClkDivisorTbaseClkR = crate::FieldReader;
#[doc = "Field `BaseClkDivisorTBaseClk` writer - Base Clock divisor (tBaseClk)"]
pub type BaseClkDivisorTbaseClkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TimeoutBaseClkDivisorToutBaseClk` reader - Timeout base clock divisor (toutBaseClk)"]
pub type TimeoutBaseClkDivisorToutBaseClkR = crate::FieldReader;
#[doc = "Field `TimeoutBaseClkDivisorToutBaseClk` writer - Timeout base clock divisor (toutBaseClk)"]
pub type TimeoutBaseClkDivisorToutBaseClkW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HoldTimeOfMasterslaveDataTHDDAT` reader - Hold time of master/slave data (tHDDAT)"]
pub type HoldTimeOfMasterslaveDataThddatR = crate::FieldReader;
#[doc = "Field `HoldTimeOfMasterslaveDataTHDDAT` writer - Hold time of master/slave data (tHDDAT)"]
pub type HoldTimeOfMasterslaveDataThddatW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CyclesOfMasterSCLClklowPulseWidthTCKLow` reader - Cycles of master SCL clock-low pulse width (tCKLow)"]
pub type CyclesOfMasterSclclklowPulseWidthTcklowR = crate::FieldReader;
#[doc = "Field `CyclesOfMasterSCLClklowPulseWidthTCKLow` writer - Cycles of master SCL clock-low pulse width (tCKLow)"]
pub type CyclesOfMasterSclclklowPulseWidthTcklowW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CyclesOfMasterSCLClkhighPulseWidthTCKHigh` reader - Cycles of master SCL clock-high pulse width (tCKHigh)"]
pub type CyclesOfMasterSclclkhighPulseWidthTckhighR = crate::FieldReader;
#[doc = "Field `CyclesOfMasterSCLClkhighPulseWidthTCKHigh` writer - Cycles of master SCL clock-high pulse width (tCKHigh)"]
pub type CyclesOfMasterSclclkhighPulseWidthTckhighW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CyclesOfMasterSCLClkhighMinimumPulseWidthTCKHighMin` reader - Cycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
pub type CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinR = crate::FieldReader;
#[doc = "Field `CyclesOfMasterSCLClkhighMinimumPulseWidthTCKHighMin` writer - Cycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
pub type CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinW<'a, REG> =
    crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TimeoutTimer` reader - Timeout timer"]
pub type TimeoutTimerR = crate::FieldReader;
#[doc = "Field `TimeoutTimer` writer - Timeout timer"]
pub type TimeoutTimerW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Base Clock divisor (tBaseClk)"]
    #[inline(always)]
    pub fn base_clk_divisor_tbase_clk(&self) -> BaseClkDivisorTbaseClkR {
        BaseClkDivisorTbaseClkR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Timeout base clock divisor (toutBaseClk)"]
    #[inline(always)]
    pub fn timeout_base_clk_divisor_tout_base_clk(&self) -> TimeoutBaseClkDivisorToutBaseClkR {
        TimeoutBaseClkDivisorToutBaseClkR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Hold time of master/slave data (tHDDAT)"]
    #[inline(always)]
    pub fn hold_time_of_masterslave_data_thddat(&self) -> HoldTimeOfMasterslaveDataThddatR {
        HoldTimeOfMasterslaveDataThddatR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Cycles of master SCL clock-low pulse width (tCKLow)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclklow_pulse_width_tcklow(
        &self,
    ) -> CyclesOfMasterSclclklowPulseWidthTcklowR {
        CyclesOfMasterSclclklowPulseWidthTcklowR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Cycles of master SCL clock-high pulse width (tCKHigh)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclkhigh_pulse_width_tckhigh(
        &self,
    ) -> CyclesOfMasterSclclkhighPulseWidthTckhighR {
        CyclesOfMasterSclclkhighPulseWidthTckhighR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Cycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclkhigh_minimum_pulse_width_tckhigh_min(
        &self,
    ) -> CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinR {
        CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:28 - Timeout timer"]
    #[inline(always)]
    pub fn timeout_timer(&self) -> TimeoutTimerR {
        TimeoutTimerR::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bits 29:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Base Clock divisor (tBaseClk)"]
    #[inline(always)]
    pub fn base_clk_divisor_tbase_clk(&mut self) -> BaseClkDivisorTbaseClkW<I2cc04Spec> {
        BaseClkDivisorTbaseClkW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Timeout base clock divisor (toutBaseClk)"]
    #[inline(always)]
    pub fn timeout_base_clk_divisor_tout_base_clk(
        &mut self,
    ) -> TimeoutBaseClkDivisorToutBaseClkW<I2cc04Spec> {
        TimeoutBaseClkDivisorToutBaseClkW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Hold time of master/slave data (tHDDAT)"]
    #[inline(always)]
    pub fn hold_time_of_masterslave_data_thddat(
        &mut self,
    ) -> HoldTimeOfMasterslaveDataThddatW<I2cc04Spec> {
        HoldTimeOfMasterslaveDataThddatW::new(self, 10)
    }
    #[doc = "Bits 12:15 - Cycles of master SCL clock-low pulse width (tCKLow)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclklow_pulse_width_tcklow(
        &mut self,
    ) -> CyclesOfMasterSclclklowPulseWidthTcklowW<I2cc04Spec> {
        CyclesOfMasterSclclklowPulseWidthTcklowW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Cycles of master SCL clock-high pulse width (tCKHigh)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclkhigh_pulse_width_tckhigh(
        &mut self,
    ) -> CyclesOfMasterSclclkhighPulseWidthTckhighW<I2cc04Spec> {
        CyclesOfMasterSclclkhighPulseWidthTckhighW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Cycles of master SCL clock-high minimum pulse width (tCKHighMin)"]
    #[inline(always)]
    pub fn cycles_of_master_sclclkhigh_minimum_pulse_width_tckhigh_min(
        &mut self,
    ) -> CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinW<I2cc04Spec> {
        CyclesOfMasterSclclkhighMinimumPulseWidthTckhighMinW::new(self, 20)
    }
    #[doc = "Bits 24:28 - Timeout timer"]
    #[inline(always)]
    pub fn timeout_timer(&mut self) -> TimeoutTimerW<I2cc04Spec> {
        TimeoutTimerW::new(self, 24)
    }
}
#[doc = "Master/Slave Clock and AC Timing Control Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cc04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cc04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cc04Spec;
impl crate::RegisterSpec for I2cc04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cc04::R`](R) reader structure"]
impl crate::Readable for I2cc04Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cc04::W`](W) writer structure"]
impl crate::Writable for I2cc04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CC04 to value 0"]
impl crate::Resettable for I2cc04Spec {}
