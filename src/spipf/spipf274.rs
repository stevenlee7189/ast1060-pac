#[doc = "Register `SPIPF274` reader"]
pub type R = crate::R<Spipf274Spec>;
#[doc = "Register `SPIPF274` writer"]
pub type W = crate::W<Spipf274Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf274::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf274::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf274Spec;
impl crate::RegisterSpec for Spipf274Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf274::R`](R) reader structure"]
impl crate::Readable for Spipf274Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf274::W`](W) writer structure"]
impl crate::Writable for Spipf274Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF274 to value 0"]
impl crate::Resettable for Spipf274Spec {}
