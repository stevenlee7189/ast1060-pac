#[doc = "Register `SPIPF354` reader"]
pub type R = crate::R<Spipf354Spec>;
#[doc = "Register `SPIPF354` writer"]
pub type W = crate::W<Spipf354Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf354::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf354::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf354Spec;
impl crate::RegisterSpec for Spipf354Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf354::R`](R) reader structure"]
impl crate::Readable for Spipf354Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf354::W`](W) writer structure"]
impl crate::Writable for Spipf354Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF354 to value 0"]
impl crate::Resettable for Spipf354Spec {}
