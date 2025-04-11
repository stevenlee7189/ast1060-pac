#[doc = "Register `SPIPF484` reader"]
pub type R = crate::R<Spipf484Spec>;
#[doc = "Register `SPIPF484` writer"]
pub type W = crate::W<Spipf484Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf484::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf484::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf484Spec;
impl crate::RegisterSpec for Spipf484Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf484::R`](R) reader structure"]
impl crate::Readable for Spipf484Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf484::W`](W) writer structure"]
impl crate::Writable for Spipf484Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF484 to value 0"]
impl crate::Resettable for Spipf484Spec {}
