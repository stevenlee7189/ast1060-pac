#[doc = "Register `SPIPF648` reader"]
pub type R = crate::R<Spipf648Spec>;
#[doc = "Register `SPIPF648` writer"]
pub type W = crate::W<Spipf648Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf648::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf648::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf648Spec;
impl crate::RegisterSpec for Spipf648Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf648::R`](R) reader structure"]
impl crate::Readable for Spipf648Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf648::W`](W) writer structure"]
impl crate::Writable for Spipf648Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF648 to value 0"]
impl crate::Resettable for Spipf648Spec {}
