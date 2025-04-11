#[doc = "Register `SPIPF3EC` reader"]
pub type R = crate::R<Spipf3ecSpec>;
#[doc = "Register `SPIPF3EC` writer"]
pub type W = crate::W<Spipf3ecSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3ec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3ec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf3ecSpec;
impl crate::RegisterSpec for Spipf3ecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf3ec::R`](R) reader structure"]
impl crate::Readable for Spipf3ecSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf3ec::W`](W) writer structure"]
impl crate::Writable for Spipf3ecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF3EC to value 0"]
impl crate::Resettable for Spipf3ecSpec {}
