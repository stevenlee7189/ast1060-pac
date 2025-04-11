#[doc = "Register `SPIPF328` reader"]
pub type R = crate::R<Spipf328Spec>;
#[doc = "Register `SPIPF328` writer"]
pub type W = crate::W<Spipf328Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf328::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf328::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf328Spec;
impl crate::RegisterSpec for Spipf328Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf328::R`](R) reader structure"]
impl crate::Readable for Spipf328Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf328::W`](W) writer structure"]
impl crate::Writable for Spipf328Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF328 to value 0"]
impl crate::Resettable for Spipf328Spec {}
