#[doc = "Register `SCU4D8` reader"]
pub type R = crate::R<Scu4d8Spec>;
#[doc = "Register `SCU4D8` writer"]
pub type W = crate::W<Scu4d8Spec>;
impl W {}
#[doc = "Multi-function Pin Control \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4d8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4d8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4d8Spec;
impl crate::RegisterSpec for Scu4d8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4d8::R`](R) reader structure"]
impl crate::Readable for Scu4d8Spec {}
#[doc = "`write(|w| ..)` method takes [`scu4d8::W`](W) writer structure"]
impl crate::Writable for Scu4d8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4D8 to value 0"]
impl crate::Resettable for Scu4d8Spec {}
