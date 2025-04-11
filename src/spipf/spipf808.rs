#[doc = "Register `SPIPF808` reader"]
pub type R = crate::R<Spipf808Spec>;
#[doc = "Register `SPIPF808` writer"]
pub type W = crate::W<Spipf808Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf808::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf808::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf808Spec;
impl crate::RegisterSpec for Spipf808Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf808::R`](R) reader structure"]
impl crate::Readable for Spipf808Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf808::W`](W) writer structure"]
impl crate::Writable for Spipf808Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF808 to value 0"]
impl crate::Resettable for Spipf808Spec {}
