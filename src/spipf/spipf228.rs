#[doc = "Register `SPIPF228` reader"]
pub type R = crate::R<Spipf228Spec>;
#[doc = "Register `SPIPF228` writer"]
pub type W = crate::W<Spipf228Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf228::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf228::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf228Spec;
impl crate::RegisterSpec for Spipf228Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf228::R`](R) reader structure"]
impl crate::Readable for Spipf228Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf228::W`](W) writer structure"]
impl crate::Writable for Spipf228Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF228 to value 0"]
impl crate::Resettable for Spipf228Spec {}
