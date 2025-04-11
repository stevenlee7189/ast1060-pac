#[doc = "Register `SPIPF62C` reader"]
pub type R = crate::R<Spipf62cSpec>;
#[doc = "Register `SPIPF62C` writer"]
pub type W = crate::W<Spipf62cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf62c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf62c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf62cSpec;
impl crate::RegisterSpec for Spipf62cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf62c::R`](R) reader structure"]
impl crate::Readable for Spipf62cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf62c::W`](W) writer structure"]
impl crate::Writable for Spipf62cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF62C to value 0"]
impl crate::Resettable for Spipf62cSpec {}
