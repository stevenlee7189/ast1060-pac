#[doc = "Register `SPIPF758` reader"]
pub type R = crate::R<Spipf758Spec>;
#[doc = "Register `SPIPF758` writer"]
pub type W = crate::W<Spipf758Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf758::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf758::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf758Spec;
impl crate::RegisterSpec for Spipf758Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf758::R`](R) reader structure"]
impl crate::Readable for Spipf758Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf758::W`](W) writer structure"]
impl crate::Writable for Spipf758Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF758 to value 0"]
impl crate::Resettable for Spipf758Spec {}
