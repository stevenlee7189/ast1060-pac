#[doc = "Register `SPIPF450` reader"]
pub type R = crate::R<Spipf450Spec>;
#[doc = "Register `SPIPF450` writer"]
pub type W = crate::W<Spipf450Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf450::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf450::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf450Spec;
impl crate::RegisterSpec for Spipf450Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf450::R`](R) reader structure"]
impl crate::Readable for Spipf450Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf450::W`](W) writer structure"]
impl crate::Writable for Spipf450Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF450 to value 0"]
impl crate::Resettable for Spipf450Spec {}
