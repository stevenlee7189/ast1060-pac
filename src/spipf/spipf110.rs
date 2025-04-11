#[doc = "Register `SPIPF110` reader"]
pub type R = crate::R<Spipf110Spec>;
#[doc = "Register `SPIPF110` writer"]
pub type W = crate::W<Spipf110Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf110Spec;
impl crate::RegisterSpec for Spipf110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf110::R`](R) reader structure"]
impl crate::Readable for Spipf110Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf110::W`](W) writer structure"]
impl crate::Writable for Spipf110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF110 to value 0"]
impl crate::Resettable for Spipf110Spec {}
