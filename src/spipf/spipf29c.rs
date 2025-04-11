#[doc = "Register `SPIPF29C` reader"]
pub type R = crate::R<Spipf29cSpec>;
#[doc = "Register `SPIPF29C` writer"]
pub type W = crate::W<Spipf29cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf29c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf29c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf29cSpec;
impl crate::RegisterSpec for Spipf29cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf29c::R`](R) reader structure"]
impl crate::Readable for Spipf29cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf29c::W`](W) writer structure"]
impl crate::Writable for Spipf29cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF29C to value 0"]
impl crate::Resettable for Spipf29cSpec {}
