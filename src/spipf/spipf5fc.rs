#[doc = "Register `SPIPF5FC` reader"]
pub type R = crate::R<Spipf5fcSpec>;
#[doc = "Register `SPIPF5FC` writer"]
pub type W = crate::W<Spipf5fcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf5fcSpec;
impl crate::RegisterSpec for Spipf5fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf5fc::R`](R) reader structure"]
impl crate::Readable for Spipf5fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf5fc::W`](W) writer structure"]
impl crate::Writable for Spipf5fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF5FC to value 0"]
impl crate::Resettable for Spipf5fcSpec {}
