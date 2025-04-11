#[doc = "Register `SPIPF234` reader"]
pub type R = crate::R<Spipf234Spec>;
#[doc = "Register `SPIPF234` writer"]
pub type W = crate::W<Spipf234Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf234::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf234::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf234Spec;
impl crate::RegisterSpec for Spipf234Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf234::R`](R) reader structure"]
impl crate::Readable for Spipf234Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf234::W`](W) writer structure"]
impl crate::Writable for Spipf234Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF234 to value 0"]
impl crate::Resettable for Spipf234Spec {}
