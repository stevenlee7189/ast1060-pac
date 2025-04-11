#[doc = "Register `SPIPF434` reader"]
pub type R = crate::R<Spipf434Spec>;
#[doc = "Register `SPIPF434` writer"]
pub type W = crate::W<Spipf434Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf434::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf434::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf434Spec;
impl crate::RegisterSpec for Spipf434Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf434::R`](R) reader structure"]
impl crate::Readable for Spipf434Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf434::W`](W) writer structure"]
impl crate::Writable for Spipf434Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF434 to value 0"]
impl crate::Resettable for Spipf434Spec {}
