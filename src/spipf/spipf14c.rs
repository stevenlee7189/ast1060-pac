#[doc = "Register `SPIPF14C` reader"]
pub type R = crate::R<Spipf14cSpec>;
#[doc = "Register `SPIPF14C` writer"]
pub type W = crate::W<Spipf14cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf14c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf14c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf14cSpec;
impl crate::RegisterSpec for Spipf14cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf14c::R`](R) reader structure"]
impl crate::Readable for Spipf14cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf14c::W`](W) writer structure"]
impl crate::Writable for Spipf14cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF14C to value 0"]
impl crate::Resettable for Spipf14cSpec {}
