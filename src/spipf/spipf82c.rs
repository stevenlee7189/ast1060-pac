#[doc = "Register `SPIPF82C` reader"]
pub type R = crate::R<Spipf82cSpec>;
#[doc = "Register `SPIPF82C` writer"]
pub type W = crate::W<Spipf82cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf82c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf82c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf82cSpec;
impl crate::RegisterSpec for Spipf82cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf82c::R`](R) reader structure"]
impl crate::Readable for Spipf82cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf82c::W`](W) writer structure"]
impl crate::Writable for Spipf82cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF82C to value 0"]
impl crate::Resettable for Spipf82cSpec {}
