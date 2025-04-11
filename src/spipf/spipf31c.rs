#[doc = "Register `SPIPF31C` reader"]
pub type R = crate::R<Spipf31cSpec>;
#[doc = "Register `SPIPF31C` writer"]
pub type W = crate::W<Spipf31cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf31c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf31c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf31cSpec;
impl crate::RegisterSpec for Spipf31cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf31c::R`](R) reader structure"]
impl crate::Readable for Spipf31cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf31c::W`](W) writer structure"]
impl crate::Writable for Spipf31cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF31C to value 0"]
impl crate::Resettable for Spipf31cSpec {}
