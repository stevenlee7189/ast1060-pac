#[doc = "Register `SPIPF72C` reader"]
pub type R = crate::R<Spipf72cSpec>;
#[doc = "Register `SPIPF72C` writer"]
pub type W = crate::W<Spipf72cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf72c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf72c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf72cSpec;
impl crate::RegisterSpec for Spipf72cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf72c::R`](R) reader structure"]
impl crate::Readable for Spipf72cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf72c::W`](W) writer structure"]
impl crate::Writable for Spipf72cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF72C to value 0"]
impl crate::Resettable for Spipf72cSpec {}
