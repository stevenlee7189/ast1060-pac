#[doc = "Register `SPIPF140` reader"]
pub type R = crate::R<Spipf140Spec>;
#[doc = "Register `SPIPF140` writer"]
pub type W = crate::W<Spipf140Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf140::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf140::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf140Spec;
impl crate::RegisterSpec for Spipf140Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf140::R`](R) reader structure"]
impl crate::Readable for Spipf140Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf140::W`](W) writer structure"]
impl crate::Writable for Spipf140Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF140 to value 0"]
impl crate::Resettable for Spipf140Spec {}
