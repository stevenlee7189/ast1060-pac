#[doc = "Register `SPIPF174` reader"]
pub type R = crate::R<Spipf174Spec>;
#[doc = "Register `SPIPF174` writer"]
pub type W = crate::W<Spipf174Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf174::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf174::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf174Spec;
impl crate::RegisterSpec for Spipf174Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf174::R`](R) reader structure"]
impl crate::Readable for Spipf174Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf174::W`](W) writer structure"]
impl crate::Writable for Spipf174Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF174 to value 0"]
impl crate::Resettable for Spipf174Spec {}
