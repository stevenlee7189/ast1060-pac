#[doc = "Register `SPIPF844` reader"]
pub type R = crate::R<Spipf844Spec>;
#[doc = "Register `SPIPF844` writer"]
pub type W = crate::W<Spipf844Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf844::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf844::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf844Spec;
impl crate::RegisterSpec for Spipf844Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf844::R`](R) reader structure"]
impl crate::Readable for Spipf844Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf844::W`](W) writer structure"]
impl crate::Writable for Spipf844Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF844 to value 0"]
impl crate::Resettable for Spipf844Spec {}
