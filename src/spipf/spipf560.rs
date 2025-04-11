#[doc = "Register `SPIPF560` reader"]
pub type R = crate::R<Spipf560Spec>;
#[doc = "Register `SPIPF560` writer"]
pub type W = crate::W<Spipf560Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf560::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf560::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf560Spec;
impl crate::RegisterSpec for Spipf560Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf560::R`](R) reader structure"]
impl crate::Readable for Spipf560Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf560::W`](W) writer structure"]
impl crate::Writable for Spipf560Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF560 to value 0"]
impl crate::Resettable for Spipf560Spec {}
