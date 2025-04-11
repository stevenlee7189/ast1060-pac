#[doc = "Register `SCU094` reader"]
pub type R = crate::R<Scu094Spec>;
#[doc = "Register `SCU094` writer"]
pub type W = crate::W<Scu094Spec>;
#[doc = "Field `SCU090ClkStopCtrlClearRegSet2` reader - SCU090 Clock Stop Control Clear Register Set 2"]
pub type Scu090clkStopCtrlClearRegSet2R = crate::FieldReader<u32>;
#[doc = "Field `SCU090ClkStopCtrlClearRegSet2` writer - SCU090 Clock Stop Control Clear Register Set 2"]
pub type Scu090clkStopCtrlClearRegSet2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU090 Clock Stop Control Clear Register Set 2"]
    #[inline(always)]
    pub fn scu090clk_stop_ctrl_clear_reg_set2(&self) -> Scu090clkStopCtrlClearRegSet2R {
        Scu090clkStopCtrlClearRegSet2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU090 Clock Stop Control Clear Register Set 2"]
    #[inline(always)]
    pub fn scu090clk_stop_ctrl_clear_reg_set2(
        &mut self,
    ) -> Scu090clkStopCtrlClearRegSet2W<Scu094Spec> {
        Scu090clkStopCtrlClearRegSet2W::new(self, 0)
    }
}
#[doc = "Clock Stop Control Clear Register Set 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scu094::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu094::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu094Spec;
impl crate::RegisterSpec for Scu094Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu094::R`](R) reader structure"]
impl crate::Readable for Scu094Spec {}
#[doc = "`write(|w| ..)` method takes [`scu094::W`](W) writer structure"]
impl crate::Writable for Scu094Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU094 to value 0"]
impl crate::Resettable for Scu094Spec {}
