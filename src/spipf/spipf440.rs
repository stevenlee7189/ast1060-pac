#[doc = "Register `SPIPF440` reader"]
pub type R = crate::R<Spipf440Spec>;
#[doc = "Register `SPIPF440` writer"]
pub type W = crate::W<Spipf440Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf440::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf440::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf440Spec;
impl crate::RegisterSpec for Spipf440Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf440::R`](R) reader structure"]
impl crate::Readable for Spipf440Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf440::W`](W) writer structure"]
impl crate::Writable for Spipf440Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF440 to value 0"]
impl crate::Resettable for Spipf440Spec {}
