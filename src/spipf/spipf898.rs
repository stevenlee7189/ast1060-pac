#[doc = "Register `SPIPF898` reader"]
pub type R = crate::R<Spipf898Spec>;
#[doc = "Register `SPIPF898` writer"]
pub type W = crate::W<Spipf898Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf898::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf898::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf898Spec;
impl crate::RegisterSpec for Spipf898Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf898::R`](R) reader structure"]
impl crate::Readable for Spipf898Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf898::W`](W) writer structure"]
impl crate::Writable for Spipf898Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF898 to value 0"]
impl crate::Resettable for Spipf898Spec {}
