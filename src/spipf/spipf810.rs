#[doc = "Register `SPIPF810` reader"]
pub type R = crate::R<Spipf810Spec>;
#[doc = "Register `SPIPF810` writer"]
pub type W = crate::W<Spipf810Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf810::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf810::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf810Spec;
impl crate::RegisterSpec for Spipf810Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf810::R`](R) reader structure"]
impl crate::Readable for Spipf810Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf810::W`](W) writer structure"]
impl crate::Writable for Spipf810Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF810 to value 0"]
impl crate::Resettable for Spipf810Spec {}
