#[doc = "Register `SPIPF200` reader"]
pub type R = crate::R<Spipf200Spec>;
#[doc = "Register `SPIPF200` writer"]
pub type W = crate::W<Spipf200Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf200Spec;
impl crate::RegisterSpec for Spipf200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf200::R`](R) reader structure"]
impl crate::Readable for Spipf200Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf200::W`](W) writer structure"]
impl crate::Writable for Spipf200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF200 to value 0"]
impl crate::Resettable for Spipf200Spec {}
