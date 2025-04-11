#[doc = "Register `SPIPF308` reader"]
pub type R = crate::R<Spipf308Spec>;
#[doc = "Register `SPIPF308` writer"]
pub type W = crate::W<Spipf308Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf308::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf308::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf308Spec;
impl crate::RegisterSpec for Spipf308Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf308::R`](R) reader structure"]
impl crate::Readable for Spipf308Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf308::W`](W) writer structure"]
impl crate::Writable for Spipf308Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF308 to value 0"]
impl crate::Resettable for Spipf308Spec {}
