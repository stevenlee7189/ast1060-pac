#[doc = "Register `SPIPF710` reader"]
pub type R = crate::R<Spipf710Spec>;
#[doc = "Register `SPIPF710` writer"]
pub type W = crate::W<Spipf710Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf710::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf710::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf710Spec;
impl crate::RegisterSpec for Spipf710Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf710::R`](R) reader structure"]
impl crate::Readable for Spipf710Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf710::W`](W) writer structure"]
impl crate::Writable for Spipf710Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF710 to value 0"]
impl crate::Resettable for Spipf710Spec {}
