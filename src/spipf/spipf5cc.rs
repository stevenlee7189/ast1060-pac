#[doc = "Register `SPIPF5CC` reader"]
pub type R = crate::R<Spipf5ccSpec>;
#[doc = "Register `SPIPF5CC` writer"]
pub type W = crate::W<Spipf5ccSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf5ccSpec;
impl crate::RegisterSpec for Spipf5ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf5cc::R`](R) reader structure"]
impl crate::Readable for Spipf5ccSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf5cc::W`](W) writer structure"]
impl crate::Writable for Spipf5ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF5CC to value 0"]
impl crate::Resettable for Spipf5ccSpec {}
