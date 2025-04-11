#[doc = "Register `SPIPF28C` reader"]
pub type R = crate::R<Spipf28cSpec>;
#[doc = "Register `SPIPF28C` writer"]
pub type W = crate::W<Spipf28cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf28c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf28c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf28cSpec;
impl crate::RegisterSpec for Spipf28cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf28c::R`](R) reader structure"]
impl crate::Readable for Spipf28cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf28c::W`](W) writer structure"]
impl crate::Writable for Spipf28cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF28C to value 0"]
impl crate::Resettable for Spipf28cSpec {}
