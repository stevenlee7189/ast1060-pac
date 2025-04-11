#[doc = "Register `SPIPF784` reader"]
pub type R = crate::R<Spipf784Spec>;
#[doc = "Register `SPIPF784` writer"]
pub type W = crate::W<Spipf784Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf784::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf784::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf784Spec;
impl crate::RegisterSpec for Spipf784Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf784::R`](R) reader structure"]
impl crate::Readable for Spipf784Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf784::W`](W) writer structure"]
impl crate::Writable for Spipf784Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF784 to value 0"]
impl crate::Resettable for Spipf784Spec {}
