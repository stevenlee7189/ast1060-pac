#[doc = "Register `SPIPF304` reader"]
pub type R = crate::R<Spipf304Spec>;
#[doc = "Register `SPIPF304` writer"]
pub type W = crate::W<Spipf304Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf304::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf304::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf304Spec;
impl crate::RegisterSpec for Spipf304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf304::R`](R) reader structure"]
impl crate::Readable for Spipf304Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf304::W`](W) writer structure"]
impl crate::Writable for Spipf304Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF304 to value 0"]
impl crate::Resettable for Spipf304Spec {}
