#[doc = "Register `SPIPF15C` reader"]
pub type R = crate::R<Spipf15cSpec>;
#[doc = "Register `SPIPF15C` writer"]
pub type W = crate::W<Spipf15cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf15cSpec;
impl crate::RegisterSpec for Spipf15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf15c::R`](R) reader structure"]
impl crate::Readable for Spipf15cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf15c::W`](W) writer structure"]
impl crate::Writable for Spipf15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF15C to value 0"]
impl crate::Resettable for Spipf15cSpec {}
