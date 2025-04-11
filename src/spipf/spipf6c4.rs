#[doc = "Register `SPIPF6C4` reader"]
pub type R = crate::R<Spipf6c4Spec>;
#[doc = "Register `SPIPF6C4` writer"]
pub type W = crate::W<Spipf6c4Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6c4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6c4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf6c4Spec;
impl crate::RegisterSpec for Spipf6c4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf6c4::R`](R) reader structure"]
impl crate::Readable for Spipf6c4Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf6c4::W`](W) writer structure"]
impl crate::Writable for Spipf6c4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF6C4 to value 0"]
impl crate::Resettable for Spipf6c4Spec {}
