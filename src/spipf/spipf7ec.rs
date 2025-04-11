#[doc = "Register `SPIPF7EC` reader"]
pub type R = crate::R<Spipf7ecSpec>;
#[doc = "Register `SPIPF7EC` writer"]
pub type W = crate::W<Spipf7ecSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf7ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf7ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf7ecSpec;
impl crate::RegisterSpec for Spipf7ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf7ec::R`](R) reader structure"]
impl crate::Readable for Spipf7ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf7ec::W`](W) writer structure"]
impl crate::Writable for Spipf7ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF7EC to value 0"]
impl crate::Resettable for Spipf7ecSpec {}
