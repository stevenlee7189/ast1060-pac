#[doc = "Register `SPIPF288` reader"]
pub type R = crate::R<Spipf288Spec>;
#[doc = "Register `SPIPF288` writer"]
pub type W = crate::W<Spipf288Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf288::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf288::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf288Spec;
impl crate::RegisterSpec for Spipf288Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf288::R`](R) reader structure"]
impl crate::Readable for Spipf288Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf288::W`](W) writer structure"]
impl crate::Writable for Spipf288Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF288 to value 0"]
impl crate::Resettable for Spipf288Spec {}
