#[doc = "Register `SPIPF264` reader"]
pub type R = crate::R<Spipf264Spec>;
#[doc = "Register `SPIPF264` writer"]
pub type W = crate::W<Spipf264Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf264::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf264::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf264Spec;
impl crate::RegisterSpec for Spipf264Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf264::R`](R) reader structure"]
impl crate::Readable for Spipf264Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf264::W`](W) writer structure"]
impl crate::Writable for Spipf264Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF264 to value 0"]
impl crate::Resettable for Spipf264Spec {}
