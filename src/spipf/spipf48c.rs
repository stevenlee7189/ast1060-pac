#[doc = "Register `SPIPF48C` reader"]
pub type R = crate::R<Spipf48cSpec>;
#[doc = "Register `SPIPF48C` writer"]
pub type W = crate::W<Spipf48cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf48c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf48c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf48cSpec;
impl crate::RegisterSpec for Spipf48cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf48c::R`](R) reader structure"]
impl crate::Readable for Spipf48cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf48c::W`](W) writer structure"]
impl crate::Writable for Spipf48cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF48C to value 0"]
impl crate::Resettable for Spipf48cSpec {}
