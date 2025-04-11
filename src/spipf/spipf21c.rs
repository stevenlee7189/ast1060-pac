#[doc = "Register `SPIPF21C` reader"]
pub type R = crate::R<Spipf21cSpec>;
#[doc = "Register `SPIPF21C` writer"]
pub type W = crate::W<Spipf21cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf21c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf21c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf21cSpec;
impl crate::RegisterSpec for Spipf21cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf21c::R`](R) reader structure"]
impl crate::Readable for Spipf21cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf21c::W`](W) writer structure"]
impl crate::Writable for Spipf21cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF21C to value 0"]
impl crate::Resettable for Spipf21cSpec {}
