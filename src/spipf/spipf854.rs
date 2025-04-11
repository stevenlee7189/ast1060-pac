#[doc = "Register `SPIPF854` reader"]
pub type R = crate::R<Spipf854Spec>;
#[doc = "Register `SPIPF854` writer"]
pub type W = crate::W<Spipf854Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf854::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf854::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf854Spec;
impl crate::RegisterSpec for Spipf854Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf854::R`](R) reader structure"]
impl crate::Readable for Spipf854Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf854::W`](W) writer structure"]
impl crate::Writable for Spipf854Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF854 to value 0"]
impl crate::Resettable for Spipf854Spec {}
