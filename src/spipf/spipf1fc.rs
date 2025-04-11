#[doc = "Register `SPIPF1FC` reader"]
pub type R = crate::R<Spipf1fcSpec>;
#[doc = "Register `SPIPF1FC` writer"]
pub type W = crate::W<Spipf1fcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1fcSpec;
impl crate::RegisterSpec for Spipf1fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1fc::R`](R) reader structure"]
impl crate::Readable for Spipf1fcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf1fc::W`](W) writer structure"]
impl crate::Writable for Spipf1fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1FC to value 0"]
impl crate::Resettable for Spipf1fcSpec {}
