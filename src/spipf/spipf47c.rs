#[doc = "Register `SPIPF47C` reader"]
pub type R = crate::R<Spipf47cSpec>;
#[doc = "Register `SPIPF47C` writer"]
pub type W = crate::W<Spipf47cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf47c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf47c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf47cSpec;
impl crate::RegisterSpec for Spipf47cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf47c::R`](R) reader structure"]
impl crate::Readable for Spipf47cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf47c::W`](W) writer structure"]
impl crate::Writable for Spipf47cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF47C to value 0"]
impl crate::Resettable for Spipf47cSpec {}
