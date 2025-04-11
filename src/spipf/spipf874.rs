#[doc = "Register `SPIPF874` reader"]
pub type R = crate::R<Spipf874Spec>;
#[doc = "Register `SPIPF874` writer"]
pub type W = crate::W<Spipf874Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf874::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf874::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf874Spec;
impl crate::RegisterSpec for Spipf874Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf874::R`](R) reader structure"]
impl crate::Readable for Spipf874Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf874::W`](W) writer structure"]
impl crate::Writable for Spipf874Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF874 to value 0"]
impl crate::Resettable for Spipf874Spec {}
