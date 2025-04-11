#[doc = "Register `SPIPF414` reader"]
pub type R = crate::R<Spipf414Spec>;
#[doc = "Register `SPIPF414` writer"]
pub type W = crate::W<Spipf414Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf414::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf414::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf414Spec;
impl crate::RegisterSpec for Spipf414Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf414::R`](R) reader structure"]
impl crate::Readable for Spipf414Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf414::W`](W) writer structure"]
impl crate::Writable for Spipf414Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF414 to value 0"]
impl crate::Resettable for Spipf414Spec {}
