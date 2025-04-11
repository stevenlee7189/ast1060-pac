#[doc = "Register `SPIPF37C` reader"]
pub type R = crate::R<Spipf37cSpec>;
#[doc = "Register `SPIPF37C` writer"]
pub type W = crate::W<Spipf37cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf37c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf37c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf37cSpec;
impl crate::RegisterSpec for Spipf37cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf37c::R`](R) reader structure"]
impl crate::Readable for Spipf37cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf37c::W`](W) writer structure"]
impl crate::Writable for Spipf37cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF37C to value 0"]
impl crate::Resettable for Spipf37cSpec {}
