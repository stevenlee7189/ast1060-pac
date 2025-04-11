#[doc = "Register `SPIPF720` reader"]
pub type R = crate::R<Spipf720Spec>;
#[doc = "Register `SPIPF720` writer"]
pub type W = crate::W<Spipf720Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf720::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf720::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf720Spec;
impl crate::RegisterSpec for Spipf720Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf720::R`](R) reader structure"]
impl crate::Readable for Spipf720Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf720::W`](W) writer structure"]
impl crate::Writable for Spipf720Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF720 to value 0"]
impl crate::Resettable for Spipf720Spec {}
