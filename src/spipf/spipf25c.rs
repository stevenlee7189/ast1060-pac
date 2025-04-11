#[doc = "Register `SPIPF25C` reader"]
pub type R = crate::R<Spipf25cSpec>;
#[doc = "Register `SPIPF25C` writer"]
pub type W = crate::W<Spipf25cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf25c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf25c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf25cSpec;
impl crate::RegisterSpec for Spipf25cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf25c::R`](R) reader structure"]
impl crate::Readable for Spipf25cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf25c::W`](W) writer structure"]
impl crate::Writable for Spipf25cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF25C to value 0"]
impl crate::Resettable for Spipf25cSpec {}
