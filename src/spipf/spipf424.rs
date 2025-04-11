#[doc = "Register `SPIPF424` reader"]
pub type R = crate::R<Spipf424Spec>;
#[doc = "Register `SPIPF424` writer"]
pub type W = crate::W<Spipf424Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf424::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf424::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf424Spec;
impl crate::RegisterSpec for Spipf424Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf424::R`](R) reader structure"]
impl crate::Readable for Spipf424Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf424::W`](W) writer structure"]
impl crate::Writable for Spipf424Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF424 to value 0"]
impl crate::Resettable for Spipf424Spec {}
