#[doc = "Register `SPIPF230` reader"]
pub type R = crate::R<Spipf230Spec>;
#[doc = "Register `SPIPF230` writer"]
pub type W = crate::W<Spipf230Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf230::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf230::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf230Spec;
impl crate::RegisterSpec for Spipf230Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf230::R`](R) reader structure"]
impl crate::Readable for Spipf230Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf230::W`](W) writer structure"]
impl crate::Writable for Spipf230Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF230 to value 0"]
impl crate::Resettable for Spipf230Spec {}
