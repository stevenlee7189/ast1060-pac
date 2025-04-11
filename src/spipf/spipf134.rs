#[doc = "Register `SPIPF134` reader"]
pub type R = crate::R<Spipf134Spec>;
#[doc = "Register `SPIPF134` writer"]
pub type W = crate::W<Spipf134Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf134::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf134::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf134Spec;
impl crate::RegisterSpec for Spipf134Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf134::R`](R) reader structure"]
impl crate::Readable for Spipf134Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf134::W`](W) writer structure"]
impl crate::Writable for Spipf134Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF134 to value 0"]
impl crate::Resettable for Spipf134Spec {}
