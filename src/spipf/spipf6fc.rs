#[doc = "Register `SPIPF6FC` reader"]
pub type R = crate::R<Spipf6fcSpec>;
#[doc = "Register `SPIPF6FC` writer"]
pub type W = crate::W<Spipf6fcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf6fcSpec;
impl crate::RegisterSpec for Spipf6fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf6fc::R`](R) reader structure"]
impl crate::Readable for Spipf6fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf6fc::W`](W) writer structure"]
impl crate::Writable for Spipf6fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF6FC to value 0"]
impl crate::Resettable for Spipf6fcSpec {}
