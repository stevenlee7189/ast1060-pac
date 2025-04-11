#[doc = "Register `SCU534` reader"]
pub type R = crate::R<Scu534Spec>;
#[doc = "Register `SCU534` writer"]
pub type W = crate::W<Scu534Spec>;
#[doc = "Field `RandomNumberData` reader - Random number data"]
pub type RandomNumberDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Random number data"]
    #[inline(always)]
    pub fn random_number_data(&self) -> RandomNumberDataR {
        RandomNumberDataR::new(self.bits)
    }
}
impl W {}
#[doc = "Random Number Generator 2 Data Output\n\nYou can [`read`](crate::Reg::read) this register and get [`scu534::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu534::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu534Spec;
impl crate::RegisterSpec for Scu534Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu534::R`](R) reader structure"]
impl crate::Readable for Scu534Spec {}
#[doc = "`write(|w| ..)` method takes [`scu534::W`](W) writer structure"]
impl crate::Writable for Scu534Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU534 to value 0"]
impl crate::Resettable for Scu534Spec {}
