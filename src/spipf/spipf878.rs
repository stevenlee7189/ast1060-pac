#[doc = "Register `SPIPF878` reader"]
pub type R = crate::R<Spipf878Spec>;
#[doc = "Register `SPIPF878` writer"]
pub type W = crate::W<Spipf878Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf878::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf878::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf878Spec;
impl crate::RegisterSpec for Spipf878Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf878::R`](R) reader structure"]
impl crate::Readable for Spipf878Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf878::W`](W) writer structure"]
impl crate::Writable for Spipf878Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF878 to value 0"]
impl crate::Resettable for Spipf878Spec {}
