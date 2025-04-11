#[doc = "Register `SPIPF55C` reader"]
pub type R = crate::R<Spipf55cSpec>;
#[doc = "Register `SPIPF55C` writer"]
pub type W = crate::W<Spipf55cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf55c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf55c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf55cSpec;
impl crate::RegisterSpec for Spipf55cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf55c::R`](R) reader structure"]
impl crate::Readable for Spipf55cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf55c::W`](W) writer structure"]
impl crate::Writable for Spipf55cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF55C to value 0"]
impl crate::Resettable for Spipf55cSpec {}
