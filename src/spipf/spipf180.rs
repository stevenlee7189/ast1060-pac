#[doc = "Register `SPIPF180` reader"]
pub type R = crate::R<Spipf180Spec>;
#[doc = "Register `SPIPF180` writer"]
pub type W = crate::W<Spipf180Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf180::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf180::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf180Spec;
impl crate::RegisterSpec for Spipf180Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf180::R`](R) reader structure"]
impl crate::Readable for Spipf180Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf180::W`](W) writer structure"]
impl crate::Writable for Spipf180Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF180 to value 0"]
impl crate::Resettable for Spipf180Spec {}
