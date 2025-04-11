#[doc = "Register `SPIPF848` reader"]
pub type R = crate::R<Spipf848Spec>;
#[doc = "Register `SPIPF848` writer"]
pub type W = crate::W<Spipf848Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf848::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf848::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf848Spec;
impl crate::RegisterSpec for Spipf848Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf848::R`](R) reader structure"]
impl crate::Readable for Spipf848Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf848::W`](W) writer structure"]
impl crate::Writable for Spipf848Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF848 to value 0"]
impl crate::Resettable for Spipf848Spec {}
