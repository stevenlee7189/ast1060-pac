#[doc = "Register `SPIPF130` reader"]
pub type R = crate::R<Spipf130Spec>;
#[doc = "Register `SPIPF130` writer"]
pub type W = crate::W<Spipf130Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf130::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf130::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf130Spec;
impl crate::RegisterSpec for Spipf130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf130::R`](R) reader structure"]
impl crate::Readable for Spipf130Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf130::W`](W) writer structure"]
impl crate::Writable for Spipf130Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF130 to value 0"]
impl crate::Resettable for Spipf130Spec {}
