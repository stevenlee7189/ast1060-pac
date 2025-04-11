#[doc = "Register `SPIPF830` reader"]
pub type R = crate::R<Spipf830Spec>;
#[doc = "Register `SPIPF830` writer"]
pub type W = crate::W<Spipf830Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf830::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf830::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf830Spec;
impl crate::RegisterSpec for Spipf830Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf830::R`](R) reader structure"]
impl crate::Readable for Spipf830Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf830::W`](W) writer structure"]
impl crate::Writable for Spipf830Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF830 to value 0"]
impl crate::Resettable for Spipf830Spec {}
