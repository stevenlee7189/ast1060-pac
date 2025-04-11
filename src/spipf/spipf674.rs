#[doc = "Register `SPIPF674` reader"]
pub type R = crate::R<Spipf674Spec>;
#[doc = "Register `SPIPF674` writer"]
pub type W = crate::W<Spipf674Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf674::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf674::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf674Spec;
impl crate::RegisterSpec for Spipf674Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf674::R`](R) reader structure"]
impl crate::Readable for Spipf674Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf674::W`](W) writer structure"]
impl crate::Writable for Spipf674Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF674 to value 0"]
impl crate::Resettable for Spipf674Spec {}
