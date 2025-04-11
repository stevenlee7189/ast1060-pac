#[doc = "Register `SPIPF350` reader"]
pub type R = crate::R<Spipf350Spec>;
#[doc = "Register `SPIPF350` writer"]
pub type W = crate::W<Spipf350Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf350::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf350::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf350Spec;
impl crate::RegisterSpec for Spipf350Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf350::R`](R) reader structure"]
impl crate::Readable for Spipf350Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf350::W`](W) writer structure"]
impl crate::Writable for Spipf350Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF350 to value 0"]
impl crate::Resettable for Spipf350Spec {}
