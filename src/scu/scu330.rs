#[doc = "Register `SCU330` reader"]
pub type R = crate::R<Scu330Spec>;
#[doc = "Register `SCU330` writer"]
pub type W = crate::W<Scu330Spec>;
#[doc = "Field `EnblRingOscillator` reader - Enable Ring Oscillator"]
pub type EnblRingOscillatorR = crate::BitReader;
#[doc = "Field `EnblRingOscillator` writer - Enable Ring Oscillator"]
pub type EnblRingOscillatorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OscillatorCounterEnbl` reader - Oscillator Counter Enable"]
pub type OscillatorCounterEnblR = crate::BitReader;
#[doc = "Field `OscillatorCounterEnbl` writer - Oscillator Counter Enable"]
pub type OscillatorCounterEnblW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ClkSrcSelForClkFrequencyMeasurement` reader - Clock source selection for clock frequency measurement"]
pub type ClkSrcSelForClkFrequencyMeasurementR = crate::FieldReader;
#[doc = "Field `ClkSrcSelForClkFrequencyMeasurement` writer - Clock source selection for clock frequency measurement"]
pub type ClkSrcSelForClkFrequencyMeasurementW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ClkFrequencyMeasurementFinished` reader - Clock frequency measurement finished"]
pub type ClkFrequencyMeasurementFinishedR = crate::BitReader;
#[doc = "Field `ClkFrequencyMeasurementCompareResult` reader - Clock frequency measurement compare result"]
pub type ClkFrequencyMeasurementCompareResultR = crate::BitReader;
#[doc = "Field `EnblOSCCounterResultOutputToPin` reader - Enable OSC counter result output to pin"]
pub type EnblOsccounterResultOutputToPinR = crate::BitReader;
#[doc = "Field `EnblOSCCounterResultOutputToPin` writer - Enable OSC counter result output to pin"]
pub type EnblOsccounterResultOutputToPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DelayRingStageCtrl` reader - Delay ring stage control"]
pub type DelayRingStageCtrlR = crate::FieldReader;
#[doc = "Field `DelayRingStageCtrl` writer - Delay ring stage control"]
pub type DelayRingStageCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `ValueOfFrequencyMeasurementCounter` reader - Value of frequency measurement counter"]
pub type ValueOfFrequencyMeasurementCounterR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable Ring Oscillator"]
    #[inline(always)]
    pub fn enbl_ring_oscillator(&self) -> EnblRingOscillatorR {
        EnblRingOscillatorR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Oscillator Counter Enable"]
    #[inline(always)]
    pub fn oscillator_counter_enbl(&self) -> OscillatorCounterEnblR {
        OscillatorCounterEnblR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Clock source selection for clock frequency measurement"]
    #[inline(always)]
    pub fn clk_src_sel_for_clk_frequency_measurement(
        &self,
    ) -> ClkSrcSelForClkFrequencyMeasurementR {
        ClkSrcSelForClkFrequencyMeasurementR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Clock frequency measurement finished"]
    #[inline(always)]
    pub fn clk_frequency_measurement_finished(&self) -> ClkFrequencyMeasurementFinishedR {
        ClkFrequencyMeasurementFinishedR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock frequency measurement compare result"]
    #[inline(always)]
    pub fn clk_frequency_measurement_compare_result(
        &self,
    ) -> ClkFrequencyMeasurementCompareResultR {
        ClkFrequencyMeasurementCompareResultR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable OSC counter result output to pin"]
    #[inline(always)]
    pub fn enbl_osccounter_result_output_to_pin(&self) -> EnblOsccounterResultOutputToPinR {
        EnblOsccounterResultOutputToPinR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Delay ring stage control"]
    #[inline(always)]
    pub fn delay_ring_stage_ctrl(&self) -> DelayRingStageCtrlR {
        DelayRingStageCtrlR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:29 - Value of frequency measurement counter"]
    #[inline(always)]
    pub fn value_of_frequency_measurement_counter(&self) -> ValueOfFrequencyMeasurementCounterR {
        ValueOfFrequencyMeasurementCounterR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Ring Oscillator"]
    #[inline(always)]
    pub fn enbl_ring_oscillator(&mut self) -> EnblRingOscillatorW<Scu330Spec> {
        EnblRingOscillatorW::new(self, 0)
    }
    #[doc = "Bit 1 - Oscillator Counter Enable"]
    #[inline(always)]
    pub fn oscillator_counter_enbl(&mut self) -> OscillatorCounterEnblW<Scu330Spec> {
        OscillatorCounterEnblW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Clock source selection for clock frequency measurement"]
    #[inline(always)]
    pub fn clk_src_sel_for_clk_frequency_measurement(
        &mut self,
    ) -> ClkSrcSelForClkFrequencyMeasurementW<Scu330Spec> {
        ClkSrcSelForClkFrequencyMeasurementW::new(self, 2)
    }
    #[doc = "Bit 8 - Enable OSC counter result output to pin"]
    #[inline(always)]
    pub fn enbl_osccounter_result_output_to_pin(
        &mut self,
    ) -> EnblOsccounterResultOutputToPinW<Scu330Spec> {
        EnblOsccounterResultOutputToPinW::new(self, 8)
    }
    #[doc = "Bits 9:15 - Delay ring stage control"]
    #[inline(always)]
    pub fn delay_ring_stage_ctrl(&mut self) -> DelayRingStageCtrlW<Scu330Spec> {
        DelayRingStageCtrlW::new(self, 9)
    }
}
#[doc = "Frequency Counter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu330::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu330::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu330Spec;
impl crate::RegisterSpec for Scu330Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu330::R`](R) reader structure"]
impl crate::Readable for Scu330Spec {}
#[doc = "`write(|w| ..)` method takes [`scu330::W`](W) writer structure"]
impl crate::Writable for Scu330Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU330 to value 0"]
impl crate::Resettable for Scu330Spec {}
