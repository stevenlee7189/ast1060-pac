#[doc = "Register `SPIPF80C` reader"]
pub type R = crate::R<Spipf80cSpec>;
#[doc = "Register `SPIPF80C` writer"]
pub type W = crate::W<Spipf80cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf80c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf80c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf80cSpec;
impl crate::RegisterSpec for Spipf80cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf80c::R`](R) reader structure"]
impl crate::Readable for Spipf80cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf80c::W`](W) writer structure"]
impl crate::Writable for Spipf80cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF80C to value 0"]
impl crate::Resettable for Spipf80cSpec {}
