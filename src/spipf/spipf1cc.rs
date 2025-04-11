#[doc = "Register `SPIPF1CC` reader"]
pub type R = crate::R<Spipf1ccSpec>;
#[doc = "Register `SPIPF1CC` writer"]
pub type W = crate::W<Spipf1ccSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1ccSpec;
impl crate::RegisterSpec for Spipf1ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1cc::R`](R) reader structure"]
impl crate::Readable for Spipf1ccSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf1cc::W`](W) writer structure"]
impl crate::Writable for Spipf1ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1CC to value 0"]
impl crate::Resettable for Spipf1ccSpec {}
