#[doc = "Register `SPIPF7FC` reader"]
pub type R = crate::R<Spipf7fcSpec>;
#[doc = "Register `SPIPF7FC` writer"]
pub type W = crate::W<Spipf7fcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf7fcSpec;
impl crate::RegisterSpec for Spipf7fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf7fc::R`](R) reader structure"]
impl crate::Readable for Spipf7fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf7fc::W`](W) writer structure"]
impl crate::Writable for Spipf7fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF7FC to value 0"]
impl crate::Resettable for Spipf7fcSpec {}
