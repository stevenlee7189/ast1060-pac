#[doc = "Register `SPIPF838` reader"]
pub type R = crate::R<Spipf838Spec>;
#[doc = "Register `SPIPF838` writer"]
pub type W = crate::W<Spipf838Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf838::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf838::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf838Spec;
impl crate::RegisterSpec for Spipf838Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf838::R`](R) reader structure"]
impl crate::Readable for Spipf838Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf838::W`](W) writer structure"]
impl crate::Writable for Spipf838Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF838 to value 0"]
impl crate::Resettable for Spipf838Spec {}
