#[doc = "Register `SPIPF58C` reader"]
pub type R = crate::R<Spipf58cSpec>;
#[doc = "Register `SPIPF58C` writer"]
pub type W = crate::W<Spipf58cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf58c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf58c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf58cSpec;
impl crate::RegisterSpec for Spipf58cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf58c::R`](R) reader structure"]
impl crate::Readable for Spipf58cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf58c::W`](W) writer structure"]
impl crate::Writable for Spipf58cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF58C to value 0"]
impl crate::Resettable for Spipf58cSpec {}
