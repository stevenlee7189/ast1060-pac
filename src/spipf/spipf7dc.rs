#[doc = "Register `SPIPF7DC` reader"]
pub type R = crate::R<Spipf7dcSpec>;
#[doc = "Register `SPIPF7DC` writer"]
pub type W = crate::W<Spipf7dcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf7dcSpec;
impl crate::RegisterSpec for Spipf7dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf7dc::R`](R) reader structure"]
impl crate::Readable for Spipf7dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf7dc::W`](W) writer structure"]
impl crate::Writable for Spipf7dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF7DC to value 0"]
impl crate::Resettable for Spipf7dcSpec {}
