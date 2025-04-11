#[doc = "Register `SPIPF750` reader"]
pub type R = crate::R<Spipf750Spec>;
#[doc = "Register `SPIPF750` writer"]
pub type W = crate::W<Spipf750Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf750::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf750::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf750Spec;
impl crate::RegisterSpec for Spipf750Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf750::R`](R) reader structure"]
impl crate::Readable for Spipf750Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf750::W`](W) writer structure"]
impl crate::Writable for Spipf750Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF750 to value 0"]
impl crate::Resettable for Spipf750Spec {}
