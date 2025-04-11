#[doc = "Register `SPIPF3AC` reader"]
pub type R = crate::R<Spipf3acSpec>;
#[doc = "Register `SPIPF3AC` writer"]
pub type W = crate::W<Spipf3acSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf3acSpec;
impl crate::RegisterSpec for Spipf3acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf3ac::R`](R) reader structure"]
impl crate::Readable for Spipf3acSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf3ac::W`](W) writer structure"]
impl crate::Writable for Spipf3acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF3AC to value 0"]
impl crate::Resettable for Spipf3acSpec {}
