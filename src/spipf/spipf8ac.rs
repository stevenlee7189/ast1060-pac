#[doc = "Register `SPIPF8AC` reader"]
pub type R = crate::R<Spipf8acSpec>;
#[doc = "Register `SPIPF8AC` writer"]
pub type W = crate::W<Spipf8acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf8ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf8ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf8acSpec;
impl crate::RegisterSpec for Spipf8acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf8ac::R`](R) reader structure"]
impl crate::Readable for Spipf8acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf8ac::W`](W) writer structure"]
impl crate::Writable for Spipf8acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF8AC to value 0"]
impl crate::Resettable for Spipf8acSpec {}
