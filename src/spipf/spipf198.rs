#[doc = "Register `SPIPF198` reader"]
pub type R = crate::R<Spipf198Spec>;
#[doc = "Register `SPIPF198` writer"]
pub type W = crate::W<Spipf198Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf198::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf198::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf198Spec;
impl crate::RegisterSpec for Spipf198Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf198::R`](R) reader structure"]
impl crate::Readable for Spipf198Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf198::W`](W) writer structure"]
impl crate::Writable for Spipf198Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF198 to value 0"]
impl crate::Resettable for Spipf198Spec {}
