#[doc = "Register `SPIPF418` reader"]
pub type R = crate::R<Spipf418Spec>;
#[doc = "Register `SPIPF418` writer"]
pub type W = crate::W<Spipf418Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf418::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf418::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf418Spec;
impl crate::RegisterSpec for Spipf418Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf418::R`](R) reader structure"]
impl crate::Readable for Spipf418Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf418::W`](W) writer structure"]
impl crate::Writable for Spipf418Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF418 to value 0"]
impl crate::Resettable for Spipf418Spec {}
