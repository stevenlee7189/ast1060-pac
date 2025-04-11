#[doc = "Register `SPIPF260` reader"]
pub type R = crate::R<Spipf260Spec>;
#[doc = "Register `SPIPF260` writer"]
pub type W = crate::W<Spipf260Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf260::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf260::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf260Spec;
impl crate::RegisterSpec for Spipf260Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf260::R`](R) reader structure"]
impl crate::Readable for Spipf260Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf260::W`](W) writer structure"]
impl crate::Writable for Spipf260Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF260 to value 0"]
impl crate::Resettable for Spipf260Spec {}
