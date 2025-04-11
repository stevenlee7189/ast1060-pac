#[doc = "Register `SPIPF700` reader"]
pub type R = crate::R<Spipf700Spec>;
#[doc = "Register `SPIPF700` writer"]
pub type W = crate::W<Spipf700Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf700::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf700::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf700Spec;
impl crate::RegisterSpec for Spipf700Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf700::R`](R) reader structure"]
impl crate::Readable for Spipf700Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf700::W`](W) writer structure"]
impl crate::Writable for Spipf700Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF700 to value 0"]
impl crate::Resettable for Spipf700Spec {}
