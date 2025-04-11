#[doc = "Register `SPIPF85C` reader"]
pub type R = crate::R<Spipf85cSpec>;
#[doc = "Register `SPIPF85C` writer"]
pub type W = crate::W<Spipf85cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf85c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf85c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf85cSpec;
impl crate::RegisterSpec for Spipf85cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf85c::R`](R) reader structure"]
impl crate::Readable for Spipf85cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf85c::W`](W) writer structure"]
impl crate::Writable for Spipf85cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF85C to value 0"]
impl crate::Resettable for Spipf85cSpec {}
