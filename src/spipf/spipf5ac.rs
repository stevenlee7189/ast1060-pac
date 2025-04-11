#[doc = "Register `SPIPF5AC` reader"]
pub type R = crate::R<Spipf5acSpec>;
#[doc = "Register `SPIPF5AC` writer"]
pub type W = crate::W<Spipf5acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf5acSpec;
impl crate::RegisterSpec for Spipf5acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf5ac::R`](R) reader structure"]
impl crate::Readable for Spipf5acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf5ac::W`](W) writer structure"]
impl crate::Writable for Spipf5acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF5AC to value 0"]
impl crate::Resettable for Spipf5acSpec {}
