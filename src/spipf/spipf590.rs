#[doc = "Register `SPIPF590` reader"]
pub type R = crate::R<Spipf590Spec>;
#[doc = "Register `SPIPF590` writer"]
pub type W = crate::W<Spipf590Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf590::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf590::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf590Spec;
impl crate::RegisterSpec for Spipf590Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf590::R`](R) reader structure"]
impl crate::Readable for Spipf590Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf590::W`](W) writer structure"]
impl crate::Writable for Spipf590Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF590 to value 0"]
impl crate::Resettable for Spipf590Spec {}
