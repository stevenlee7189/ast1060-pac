#[doc = "Register `SPIPF388` reader"]
pub type R = crate::R<Spipf388Spec>;
#[doc = "Register `SPIPF388` writer"]
pub type W = crate::W<Spipf388Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf388::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf388::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf388Spec;
impl crate::RegisterSpec for Spipf388Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf388::R`](R) reader structure"]
impl crate::Readable for Spipf388Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf388::W`](W) writer structure"]
impl crate::Writable for Spipf388Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF388 to value 0"]
impl crate::Resettable for Spipf388Spec {}
