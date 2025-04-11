#[doc = "Register `SPIPF278` reader"]
pub type R = crate::R<Spipf278Spec>;
#[doc = "Register `SPIPF278` writer"]
pub type W = crate::W<Spipf278Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf278::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf278::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf278Spec;
impl crate::RegisterSpec for Spipf278Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf278::R`](R) reader structure"]
impl crate::Readable for Spipf278Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf278::W`](W) writer structure"]
impl crate::Writable for Spipf278Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF278 to value 0"]
impl crate::Resettable for Spipf278Spec {}
