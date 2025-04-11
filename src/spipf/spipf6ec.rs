#[doc = "Register `SPIPF6EC` reader"]
pub type R = crate::R<Spipf6ecSpec>;
#[doc = "Register `SPIPF6EC` writer"]
pub type W = crate::W<Spipf6ecSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf6ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf6ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf6ecSpec;
impl crate::RegisterSpec for Spipf6ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf6ec::R`](R) reader structure"]
impl crate::Readable for Spipf6ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf6ec::W`](W) writer structure"]
impl crate::Writable for Spipf6ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF6EC to value 0"]
impl crate::Resettable for Spipf6ecSpec {}
