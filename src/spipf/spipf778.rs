#[doc = "Register `SPIPF778` reader"]
pub type R = crate::R<Spipf778Spec>;
#[doc = "Register `SPIPF778` writer"]
pub type W = crate::W<Spipf778Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf778::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf778::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf778Spec;
impl crate::RegisterSpec for Spipf778Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf778::R`](R) reader structure"]
impl crate::Readable for Spipf778Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf778::W`](W) writer structure"]
impl crate::Writable for Spipf778Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF778 to value 0"]
impl crate::Resettable for Spipf778Spec {}
