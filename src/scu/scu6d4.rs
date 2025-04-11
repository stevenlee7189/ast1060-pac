#[doc = "Register `SCU6D4` reader"]
pub type R = crate::R<Scu6d4Spec>;
#[doc = "Register `SCU6D4` writer"]
pub type W = crate::W<Scu6d4Spec>;
impl W {}
#[doc = "Multi-function Pin Control \\#30\n\nYou can [`read`](crate::Reg::read) this register and get [`scu6d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu6d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu6d4Spec;
impl crate::RegisterSpec for Scu6d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu6d4::R`](R) reader structure"]
impl crate::Readable for Scu6d4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu6d4::W`](W) writer structure"]
impl crate::Writable for Scu6d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU6D4 to value 0"]
impl crate::Resettable for Scu6d4Spec {}
