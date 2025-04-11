#[doc = "Register `SPIPF298` reader"]
pub type R = crate::R<Spipf298Spec>;
#[doc = "Register `SPIPF298` writer"]
pub type W = crate::W<Spipf298Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf298::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf298::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf298Spec;
impl crate::RegisterSpec for Spipf298Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf298::R`](R) reader structure"]
impl crate::Readable for Spipf298Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf298::W`](W) writer structure"]
impl crate::Writable for Spipf298Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF298 to value 0"]
impl crate::Resettable for Spipf298Spec {}
