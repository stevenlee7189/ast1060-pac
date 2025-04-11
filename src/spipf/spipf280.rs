#[doc = "Register `SPIPF280` reader"]
pub type R = crate::R<Spipf280Spec>;
#[doc = "Register `SPIPF280` writer"]
pub type W = crate::W<Spipf280Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf280::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf280::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf280Spec;
impl crate::RegisterSpec for Spipf280Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf280::R`](R) reader structure"]
impl crate::Readable for Spipf280Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf280::W`](W) writer structure"]
impl crate::Writable for Spipf280Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF280 to value 0"]
impl crate::Resettable for Spipf280Spec {}
