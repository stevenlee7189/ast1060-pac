#[doc = "Register `SPIPF1AC` reader"]
pub type R = crate::R<Spipf1acSpec>;
#[doc = "Register `SPIPF1AC` writer"]
pub type W = crate::W<Spipf1acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1acSpec;
impl crate::RegisterSpec for Spipf1acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1ac::R`](R) reader structure"]
impl crate::Readable for Spipf1acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf1ac::W`](W) writer structure"]
impl crate::Writable for Spipf1acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1AC to value 0"]
impl crate::Resettable for Spipf1acSpec {}
