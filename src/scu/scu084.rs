#[doc = "Register `SCU084` reader"]
pub type R = crate::R<Scu084Spec>;
#[doc = "Register `SCU084` writer"]
pub type W = crate::W<Scu084Spec>;
#[doc = "Field `SCU080ClkStopCtrlClearReg` reader - SCU080 Clock Stop Control Clear Register"]
pub type Scu080clkStopCtrlClearRegR = crate::FieldReader<u32>;
#[doc = "Field `SCU080ClkStopCtrlClearReg` writer - SCU080 Clock Stop Control Clear Register"]
pub type Scu080clkStopCtrlClearRegW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU080 Clock Stop Control Clear Register"]
    #[inline(always)]
    pub fn scu080clk_stop_ctrl_clear_reg(&self) -> Scu080clkStopCtrlClearRegR {
        Scu080clkStopCtrlClearRegR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU080 Clock Stop Control Clear Register"]
    #[inline(always)]
    pub fn scu080clk_stop_ctrl_clear_reg(&mut self) -> Scu080clkStopCtrlClearRegW<Scu084Spec> {
        Scu080clkStopCtrlClearRegW::new(self, 0)
    }
}
#[doc = "Clock Stop Control Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu084::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu084::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu084Spec;
impl crate::RegisterSpec for Scu084Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu084::R`](R) reader structure"]
impl crate::Readable for Scu084Spec {}
#[doc = "`write(|w| ..)` method takes [`scu084::W`](W) writer structure"]
impl crate::Writable for Scu084Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU084 to value 0"]
impl crate::Resettable for Scu084Spec {}
