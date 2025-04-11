#[doc = "Register `SPIPF4AC` reader"]
pub type R = crate::R<Spipf4acSpec>;
#[doc = "Register `SPIPF4AC` writer"]
pub type W = crate::W<Spipf4acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf4ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf4ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf4acSpec;
impl crate::RegisterSpec for Spipf4acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf4ac::R`](R) reader structure"]
impl crate::Readable for Spipf4acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf4ac::W`](W) writer structure"]
impl crate::Writable for Spipf4acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF4AC to value 0"]
impl crate::Resettable for Spipf4acSpec {}
