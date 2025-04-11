#[doc = "Register `SPIPF598` reader"]
pub type R = crate::R<Spipf598Spec>;
#[doc = "Register `SPIPF598` writer"]
pub type W = crate::W<Spipf598Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf598::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf598::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf598Spec;
impl crate::RegisterSpec for Spipf598Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf598::R`](R) reader structure"]
impl crate::Readable for Spipf598Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf598::W`](W) writer structure"]
impl crate::Writable for Spipf598Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF598 to value 0"]
impl crate::Resettable for Spipf598Spec {}
