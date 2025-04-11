#[doc = "Register `SPIPF798` reader"]
pub type R = crate::R<Spipf798Spec>;
#[doc = "Register `SPIPF798` writer"]
pub type W = crate::W<Spipf798Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf798::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf798::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf798Spec;
impl crate::RegisterSpec for Spipf798Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf798::R`](R) reader structure"]
impl crate::Readable for Spipf798Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf798::W`](W) writer structure"]
impl crate::Writable for Spipf798Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF798 to value 0"]
impl crate::Resettable for Spipf798Spec {}
