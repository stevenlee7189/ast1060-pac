#[doc = "Register `SPIPF600` reader"]
pub type R = crate::R<Spipf600Spec>;
#[doc = "Register `SPIPF600` writer"]
pub type W = crate::W<Spipf600Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf600::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf600::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf600Spec;
impl crate::RegisterSpec for Spipf600Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf600::R`](R) reader structure"]
impl crate::Readable for Spipf600Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf600::W`](W) writer structure"]
impl crate::Writable for Spipf600Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF600 to value 0"]
impl crate::Resettable for Spipf600Spec {}
