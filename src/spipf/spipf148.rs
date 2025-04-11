#[doc = "Register `SPIPF148` reader"]
pub type R = crate::R<Spipf148Spec>;
#[doc = "Register `SPIPF148` writer"]
pub type W = crate::W<Spipf148Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf148::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf148::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf148Spec;
impl crate::RegisterSpec for Spipf148Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf148::R`](R) reader structure"]
impl crate::Readable for Spipf148Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf148::W`](W) writer structure"]
impl crate::Writable for Spipf148Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF148 to value 0"]
impl crate::Resettable for Spipf148Spec {}
