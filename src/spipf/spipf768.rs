#[doc = "Register `SPIPF768` reader"]
pub type R = crate::R<Spipf768Spec>;
#[doc = "Register `SPIPF768` writer"]
pub type W = crate::W<Spipf768Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf768::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf768::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf768Spec;
impl crate::RegisterSpec for Spipf768Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf768::R`](R) reader structure"]
impl crate::Readable for Spipf768Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf768::W`](W) writer structure"]
impl crate::Writable for Spipf768Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF768 to value 0"]
impl crate::Resettable for Spipf768Spec {}
