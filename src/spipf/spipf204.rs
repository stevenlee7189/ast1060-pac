#[doc = "Register `SPIPF204` reader"]
pub type R = crate::R<Spipf204Spec>;
#[doc = "Register `SPIPF204` writer"]
pub type W = crate::W<Spipf204Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf204Spec;
impl crate::RegisterSpec for Spipf204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf204::R`](R) reader structure"]
impl crate::Readable for Spipf204Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf204::W`](W) writer structure"]
impl crate::Writable for Spipf204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF204 to value 0"]
impl crate::Resettable for Spipf204Spec {}
