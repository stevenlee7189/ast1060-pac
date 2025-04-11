#[doc = "Register `SPIPF528` reader"]
pub type R = crate::R<Spipf528Spec>;
#[doc = "Register `SPIPF528` writer"]
pub type W = crate::W<Spipf528Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf528::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf528::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf528Spec;
impl crate::RegisterSpec for Spipf528Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf528::R`](R) reader structure"]
impl crate::Readable for Spipf528Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf528::W`](W) writer structure"]
impl crate::Writable for Spipf528Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF528 to value 0"]
impl crate::Resettable for Spipf528Spec {}
