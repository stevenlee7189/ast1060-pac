#[doc = "Register `SPIPF330` reader"]
pub type R = crate::R<Spipf330Spec>;
#[doc = "Register `SPIPF330` writer"]
pub type W = crate::W<Spipf330Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf330::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf330::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf330Spec;
impl crate::RegisterSpec for Spipf330Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf330::R`](R) reader structure"]
impl crate::Readable for Spipf330Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf330::W`](W) writer structure"]
impl crate::Writable for Spipf330Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF330 to value 0"]
impl crate::Resettable for Spipf330Spec {}
