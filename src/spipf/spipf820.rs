#[doc = "Register `SPIPF820` reader"]
pub type R = crate::R<Spipf820Spec>;
#[doc = "Register `SPIPF820` writer"]
pub type W = crate::W<Spipf820Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf820::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf820::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf820Spec;
impl crate::RegisterSpec for Spipf820Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf820::R`](R) reader structure"]
impl crate::Readable for Spipf820Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf820::W`](W) writer structure"]
impl crate::Writable for Spipf820Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF820 to value 0"]
impl crate::Resettable for Spipf820Spec {}
