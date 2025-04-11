#[doc = "Register `SPIPF384` reader"]
pub type R = crate::R<Spipf384Spec>;
#[doc = "Register `SPIPF384` writer"]
pub type W = crate::W<Spipf384Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf384::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf384::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf384Spec;
impl crate::RegisterSpec for Spipf384Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf384::R`](R) reader structure"]
impl crate::Readable for Spipf384Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf384::W`](W) writer structure"]
impl crate::Writable for Spipf384Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF384 to value 0"]
impl crate::Resettable for Spipf384Spec {}
