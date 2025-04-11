#[doc = "Register `SPIPF360` reader"]
pub type R = crate::R<Spipf360Spec>;
#[doc = "Register `SPIPF360` writer"]
pub type W = crate::W<Spipf360Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf360::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf360::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf360Spec;
impl crate::RegisterSpec for Spipf360Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf360::R`](R) reader structure"]
impl crate::Readable for Spipf360Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf360::W`](W) writer structure"]
impl crate::Writable for Spipf360Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF360 to value 0"]
impl crate::Resettable for Spipf360Spec {}
