#[doc = "Register `FMC094` reader"]
pub type R = crate::R<Fmc094Spec>;
#[doc = "Register `FMC094` writer"]
pub type W = crate::W<Fmc094Spec>;
#[doc = "Field `SPICLKHCLK2DelayCycleForDataInputLatchPoint` reader - SPICLK = HCLK/2, delay cycle for data input latch point"]
pub type Spiclkhclk2delayCycleForDataInputLatchPointR = crate::FieldReader;
#[doc = "Field `SPICLKHCLK2DelayCycleForDataInputLatchPoint` writer - SPICLK = HCLK/2, delay cycle for data input latch point"]
pub type Spiclkhclk2delayCycleForDataInputLatchPointW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPICLKHCLK3DelayCycleForDataInputLatchPoint` reader - SPICLK = HCLK/3, delay cycle for data input latch point"]
pub type Spiclkhclk3delayCycleForDataInputLatchPointR = crate::FieldReader;
#[doc = "Field `SPICLKHCLK3DelayCycleForDataInputLatchPoint` writer - SPICLK = HCLK/3, delay cycle for data input latch point"]
pub type Spiclkhclk3delayCycleForDataInputLatchPointW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPICLKHCLK4DelayCycleForDataInputLatchPoint` reader - SPICLK = HCLK/4, delay cycle for data input latch point"]
pub type Spiclkhclk4delayCycleForDataInputLatchPointR = crate::FieldReader;
#[doc = "Field `SPICLKHCLK4DelayCycleForDataInputLatchPoint` writer - SPICLK = HCLK/4, delay cycle for data input latch point"]
pub type Spiclkhclk4delayCycleForDataInputLatchPointW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPICLKHCLK5DelayCycleForDataInputLatchPoint` reader - SPICLK = HCLK/5, delay cycle for data input latch point"]
pub type Spiclkhclk5delayCycleForDataInputLatchPointR = crate::FieldReader;
#[doc = "Field `SPICLKHCLK5DelayCycleForDataInputLatchPoint` writer - SPICLK = HCLK/5, delay cycle for data input latch point"]
pub type Spiclkhclk5delayCycleForDataInputLatchPointW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SPICLK = HCLK/2, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk2delay_cycle_for_data_input_latch_point(
        &self,
    ) -> Spiclkhclk2delayCycleForDataInputLatchPointR {
        Spiclkhclk2delayCycleForDataInputLatchPointR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SPICLK = HCLK/3, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk3delay_cycle_for_data_input_latch_point(
        &self,
    ) -> Spiclkhclk3delayCycleForDataInputLatchPointR {
        Spiclkhclk3delayCycleForDataInputLatchPointR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPICLK = HCLK/4, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk4delay_cycle_for_data_input_latch_point(
        &self,
    ) -> Spiclkhclk4delayCycleForDataInputLatchPointR {
        Spiclkhclk4delayCycleForDataInputLatchPointR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SPICLK = HCLK/5, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk5delay_cycle_for_data_input_latch_point(
        &self,
    ) -> Spiclkhclk5delayCycleForDataInputLatchPointR {
        Spiclkhclk5delayCycleForDataInputLatchPointR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SPICLK = HCLK/2, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk2delay_cycle_for_data_input_latch_point(
        &mut self,
    ) -> Spiclkhclk2delayCycleForDataInputLatchPointW<Fmc094Spec> {
        Spiclkhclk2delayCycleForDataInputLatchPointW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SPICLK = HCLK/3, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk3delay_cycle_for_data_input_latch_point(
        &mut self,
    ) -> Spiclkhclk3delayCycleForDataInputLatchPointW<Fmc094Spec> {
        Spiclkhclk3delayCycleForDataInputLatchPointW::new(self, 8)
    }
    #[doc = "Bits 16:23 - SPICLK = HCLK/4, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk4delay_cycle_for_data_input_latch_point(
        &mut self,
    ) -> Spiclkhclk4delayCycleForDataInputLatchPointW<Fmc094Spec> {
        Spiclkhclk4delayCycleForDataInputLatchPointW::new(self, 16)
    }
    #[doc = "Bits 24:31 - SPICLK = HCLK/5, delay cycle for data input latch point"]
    #[inline(always)]
    pub fn spiclkhclk5delay_cycle_for_data_input_latch_point(
        &mut self,
    ) -> Spiclkhclk5delayCycleForDataInputLatchPointW<Fmc094Spec> {
        Spiclkhclk5delayCycleForDataInputLatchPointW::new(self, 24)
    }
}
#[doc = "CE0 SPI Flash Read Timing Compensation\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc094Spec;
impl crate::RegisterSpec for Fmc094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc094::R`](R) reader structure"]
impl crate::Readable for Fmc094Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc094::W`](W) writer structure"]
impl crate::Writable for Fmc094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC094 to value 0"]
impl crate::Resettable for Fmc094Spec {}
