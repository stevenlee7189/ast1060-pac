#[doc = "Register `SPIPF54C` reader"]
pub type R = crate::R<Spipf54cSpec>;
#[doc = "Register `SPIPF54C` writer"]
pub type W = crate::W<Spipf54cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf54c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf54c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf54cSpec;
impl crate::RegisterSpec for Spipf54cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf54c::R`](R) reader structure"]
impl crate::Readable for Spipf54cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf54c::W`](W) writer structure"]
impl crate::Writable for Spipf54cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF54C to value 0"]
impl crate::Resettable for Spipf54cSpec {}
