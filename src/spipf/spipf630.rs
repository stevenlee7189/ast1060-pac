#[doc = "Register `SPIPF630` reader"]
pub type R = crate::R<Spipf630Spec>;
#[doc = "Register `SPIPF630` writer"]
pub type W = crate::W<Spipf630Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf630::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf630::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf630Spec;
impl crate::RegisterSpec for Spipf630Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf630::R`](R) reader structure"]
impl crate::Readable for Spipf630Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf630::W`](W) writer structure"]
impl crate::Writable for Spipf630Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF630 to value 0"]
impl crate::Resettable for Spipf630Spec {}
