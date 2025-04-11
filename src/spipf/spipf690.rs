#[doc = "Register `SPIPF690` reader"]
pub type R = crate::R<Spipf690Spec>;
#[doc = "Register `SPIPF690` writer"]
pub type W = crate::W<Spipf690Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf690::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf690::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf690Spec;
impl crate::RegisterSpec for Spipf690Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf690::R`](R) reader structure"]
impl crate::Readable for Spipf690Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf690::W`](W) writer structure"]
impl crate::Writable for Spipf690Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF690 to value 0"]
impl crate::Resettable for Spipf690Spec {}
