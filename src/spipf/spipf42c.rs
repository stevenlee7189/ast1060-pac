#[doc = "Register `SPIPF42C` reader"]
pub type R = crate::R<Spipf42cSpec>;
#[doc = "Register `SPIPF42C` writer"]
pub type W = crate::W<Spipf42cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf42c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf42c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf42cSpec;
impl crate::RegisterSpec for Spipf42cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf42c::R`](R) reader structure"]
impl crate::Readable for Spipf42cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf42c::W`](W) writer structure"]
impl crate::Writable for Spipf42cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF42C to value 0"]
impl crate::Resettable for Spipf42cSpec {}
