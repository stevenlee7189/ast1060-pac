#[doc = "Register `SPIPF828` reader"]
pub type R = crate::R<Spipf828Spec>;
#[doc = "Register `SPIPF828` writer"]
pub type W = crate::W<Spipf828Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf828::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf828::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf828Spec;
impl crate::RegisterSpec for Spipf828Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf828::R`](R) reader structure"]
impl crate::Readable for Spipf828Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf828::W`](W) writer structure"]
impl crate::Writable for Spipf828Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF828 to value 0"]
impl crate::Resettable for Spipf828Spec {}
