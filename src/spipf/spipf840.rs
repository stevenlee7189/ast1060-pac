#[doc = "Register `SPIPF840` reader"]
pub type R = crate::R<Spipf840Spec>;
#[doc = "Register `SPIPF840` writer"]
pub type W = crate::W<Spipf840Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf840::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf840::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf840Spec;
impl crate::RegisterSpec for Spipf840Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf840::R`](R) reader structure"]
impl crate::Readable for Spipf840Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf840::W`](W) writer structure"]
impl crate::Writable for Spipf840Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF840 to value 0"]
impl crate::Resettable for Spipf840Spec {}
