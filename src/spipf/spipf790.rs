#[doc = "Register `SPIPF790` reader"]
pub type R = crate::R<Spipf790Spec>;
#[doc = "Register `SPIPF790` writer"]
pub type W = crate::W<Spipf790Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf790::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf790::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf790Spec;
impl crate::RegisterSpec for Spipf790Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf790::R`](R) reader structure"]
impl crate::Readable for Spipf790Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf790::W`](W) writer structure"]
impl crate::Writable for Spipf790Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF790 to value 0"]
impl crate::Resettable for Spipf790Spec {}
