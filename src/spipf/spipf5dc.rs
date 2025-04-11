#[doc = "Register `SPIPF5DC` reader"]
pub type R = crate::R<Spipf5dcSpec>;
#[doc = "Register `SPIPF5DC` writer"]
pub type W = crate::W<Spipf5dcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf5dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf5dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf5dcSpec;
impl crate::RegisterSpec for Spipf5dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf5dc::R`](R) reader structure"]
impl crate::Readable for Spipf5dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf5dc::W`](W) writer structure"]
impl crate::Writable for Spipf5dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF5DC to value 0"]
impl crate::Resettable for Spipf5dcSpec {}
