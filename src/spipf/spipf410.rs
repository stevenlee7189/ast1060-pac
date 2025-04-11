#[doc = "Register `SPIPF410` reader"]
pub type R = crate::R<Spipf410Spec>;
#[doc = "Register `SPIPF410` writer"]
pub type W = crate::W<Spipf410Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf410::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf410::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf410Spec;
impl crate::RegisterSpec for Spipf410Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf410::R`](R) reader structure"]
impl crate::Readable for Spipf410Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf410::W`](W) writer structure"]
impl crate::Writable for Spipf410Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF410 to value 0"]
impl crate::Resettable for Spipf410Spec {}
