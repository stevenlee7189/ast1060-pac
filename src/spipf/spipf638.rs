#[doc = "Register `SPIPF638` reader"]
pub type R = crate::R<Spipf638Spec>;
#[doc = "Register `SPIPF638` writer"]
pub type W = crate::W<Spipf638Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf638::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf638::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf638Spec;
impl crate::RegisterSpec for Spipf638Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf638::R`](R) reader structure"]
impl crate::Readable for Spipf638Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf638::W`](W) writer structure"]
impl crate::Writable for Spipf638Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF638 to value 0"]
impl crate::Resettable for Spipf638Spec {}
