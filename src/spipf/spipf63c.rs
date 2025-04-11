#[doc = "Register `SPIPF63C` reader"]
pub type R = crate::R<Spipf63cSpec>;
#[doc = "Register `SPIPF63C` writer"]
pub type W = crate::W<Spipf63cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf63c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf63c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf63cSpec;
impl crate::RegisterSpec for Spipf63cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf63c::R`](R) reader structure"]
impl crate::Readable for Spipf63cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf63c::W`](W) writer structure"]
impl crate::Writable for Spipf63cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF63C to value 0"]
impl crate::Resettable for Spipf63cSpec {}
