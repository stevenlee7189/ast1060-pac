#[doc = "Register `FMC090` reader"]
pub type R = crate::R<Fmc090Spec>;
#[doc = "Register `FMC090` writer"]
pub type W = crate::W<Fmc090Spec>;
#[doc = "Field `CheckSumCalculationResult` reader - CheckSum Calculation Result"]
pub type CheckSumCalculationResultR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - CheckSum Calculation Result"]
    #[inline(always)]
    pub fn check_sum_calculation_result(&self) -> CheckSumCalculationResultR {
        CheckSumCalculationResultR::new(self.bits)
    }
}
impl W {}
#[doc = "CheckSum Calculation Result\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc090Spec;
impl crate::RegisterSpec for Fmc090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc090::R`](R) reader structure"]
impl crate::Readable for Fmc090Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc090::W`](W) writer structure"]
impl crate::Writable for Fmc090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC090 to value 0"]
impl crate::Resettable for Fmc090Spec {}
