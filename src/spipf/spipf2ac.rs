#[doc = "Register `SPIPF2AC` reader"]
pub type R = crate::R<Spipf2acSpec>;
#[doc = "Register `SPIPF2AC` writer"]
pub type W = crate::W<Spipf2acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf2ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf2ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf2acSpec;
impl crate::RegisterSpec for Spipf2acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf2ac::R`](R) reader structure"]
impl crate::Readable for Spipf2acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf2ac::W`](W) writer structure"]
impl crate::Writable for Spipf2acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF2AC to value 0"]
impl crate::Resettable for Spipf2acSpec {}
