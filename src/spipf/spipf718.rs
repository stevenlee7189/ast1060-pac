#[doc = "Register `SPIPF718` reader"]
pub type R = crate::R<Spipf718Spec>;
#[doc = "Register `SPIPF718` writer"]
pub type W = crate::W<Spipf718Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf718::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf718::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf718Spec;
impl crate::RegisterSpec for Spipf718Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf718::R`](R) reader structure"]
impl crate::Readable for Spipf718Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf718::W`](W) writer structure"]
impl crate::Writable for Spipf718Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF718 to value 0"]
impl crate::Resettable for Spipf718Spec {}
