#[doc = "Register `SPIPF4FC` reader"]
pub type R = crate::R<Spipf4fcSpec>;
#[doc = "Register `SPIPF4FC` writer"]
pub type W = crate::W<Spipf4fcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf4fcSpec;
impl crate::RegisterSpec for Spipf4fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf4fc::R`](R) reader structure"]
impl crate::Readable for Spipf4fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf4fc::W`](W) writer structure"]
impl crate::Writable for Spipf4fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF4FC to value 0"]
impl crate::Resettable for Spipf4fcSpec {}
