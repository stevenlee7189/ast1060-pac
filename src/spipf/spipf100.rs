#[doc = "Register `SPIPF100` reader"]
pub type R = crate::R<Spipf100Spec>;
#[doc = "Register `SPIPF100` writer"]
pub type W = crate::W<Spipf100Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf100Spec;
impl crate::RegisterSpec for Spipf100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf100::R`](R) reader structure"]
impl crate::Readable for Spipf100Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf100::W`](W) writer structure"]
impl crate::Writable for Spipf100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF100 to value 0"]
impl crate::Resettable for Spipf100Spec {}
