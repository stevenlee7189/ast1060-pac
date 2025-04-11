#[doc = "Register `SPIPF620` reader"]
pub type R = crate::R<Spipf620Spec>;
#[doc = "Register `SPIPF620` writer"]
pub type W = crate::W<Spipf620Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf620::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf620::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf620Spec;
impl crate::RegisterSpec for Spipf620Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf620::R`](R) reader structure"]
impl crate::Readable for Spipf620Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf620::W`](W) writer structure"]
impl crate::Writable for Spipf620Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF620 to value 0"]
impl crate::Resettable for Spipf620Spec {}
