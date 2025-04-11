#[doc = "Register `SPIPF374` reader"]
pub type R = crate::R<Spipf374Spec>;
#[doc = "Register `SPIPF374` writer"]
pub type W = crate::W<Spipf374Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf374::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf374::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf374Spec;
impl crate::RegisterSpec for Spipf374Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf374::R`](R) reader structure"]
impl crate::Readable for Spipf374Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf374::W`](W) writer structure"]
impl crate::Writable for Spipf374Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF374 to value 0"]
impl crate::Resettable for Spipf374Spec {}
