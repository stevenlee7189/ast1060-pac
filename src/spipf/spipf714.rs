#[doc = "Register `SPIPF714` reader"]
pub type R = crate::R<Spipf714Spec>;
#[doc = "Register `SPIPF714` writer"]
pub type W = crate::W<Spipf714Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf714::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf714::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf714Spec;
impl crate::RegisterSpec for Spipf714Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf714::R`](R) reader structure"]
impl crate::Readable for Spipf714Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf714::W`](W) writer structure"]
impl crate::Writable for Spipf714Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF714 to value 0"]
impl crate::Resettable for Spipf714Spec {}
