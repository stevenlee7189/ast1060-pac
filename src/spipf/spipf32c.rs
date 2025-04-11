#[doc = "Register `SPIPF32C` reader"]
pub type R = crate::R<Spipf32cSpec>;
#[doc = "Register `SPIPF32C` writer"]
pub type W = crate::W<Spipf32cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf32c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf32c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf32cSpec;
impl crate::RegisterSpec for Spipf32cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf32c::R`](R) reader structure"]
impl crate::Readable for Spipf32cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf32c::W`](W) writer structure"]
impl crate::Writable for Spipf32cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF32C to value 0"]
impl crate::Resettable for Spipf32cSpec {}
