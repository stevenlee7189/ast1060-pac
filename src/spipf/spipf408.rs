#[doc = "Register `SPIPF408` reader"]
pub type R = crate::R<Spipf408Spec>;
#[doc = "Register `SPIPF408` writer"]
pub type W = crate::W<Spipf408Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf408::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf408::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf408Spec;
impl crate::RegisterSpec for Spipf408Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf408::R`](R) reader structure"]
impl crate::Readable for Spipf408Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf408::W`](W) writer structure"]
impl crate::Writable for Spipf408Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF408 to value 0"]
impl crate::Resettable for Spipf408Spec {}
