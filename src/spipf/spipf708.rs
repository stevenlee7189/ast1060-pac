#[doc = "Register `SPIPF708` reader"]
pub type R = crate::R<Spipf708Spec>;
#[doc = "Register `SPIPF708` writer"]
pub type W = crate::W<Spipf708Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf708::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf708::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf708Spec;
impl crate::RegisterSpec for Spipf708Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf708::R`](R) reader structure"]
impl crate::Readable for Spipf708Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf708::W`](W) writer structure"]
impl crate::Writable for Spipf708Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF708 to value 0"]
impl crate::Resettable for Spipf708Spec {}
