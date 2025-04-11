#[doc = "Register `SPIPF470` reader"]
pub type R = crate::R<Spipf470Spec>;
#[doc = "Register `SPIPF470` writer"]
pub type W = crate::W<Spipf470Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf470::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf470::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf470Spec;
impl crate::RegisterSpec for Spipf470Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf470::R`](R) reader structure"]
impl crate::Readable for Spipf470Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf470::W`](W) writer structure"]
impl crate::Writable for Spipf470Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF470 to value 0"]
impl crate::Resettable for Spipf470Spec {}
