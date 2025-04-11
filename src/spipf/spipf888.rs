#[doc = "Register `SPIPF888` reader"]
pub type R = crate::R<Spipf888Spec>;
#[doc = "Register `SPIPF888` writer"]
pub type W = crate::W<Spipf888Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf888::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf888::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf888Spec;
impl crate::RegisterSpec for Spipf888Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf888::R`](R) reader structure"]
impl crate::Readable for Spipf888Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf888::W`](W) writer structure"]
impl crate::Writable for Spipf888Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF888 to value 0"]
impl crate::Resettable for Spipf888Spec {}
