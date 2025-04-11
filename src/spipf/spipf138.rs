#[doc = "Register `SPIPF138` reader"]
pub type R = crate::R<Spipf138Spec>;
#[doc = "Register `SPIPF138` writer"]
pub type W = crate::W<Spipf138Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf138::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf138::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf138Spec;
impl crate::RegisterSpec for Spipf138Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf138::R`](R) reader structure"]
impl crate::Readable for Spipf138Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf138::W`](W) writer structure"]
impl crate::Writable for Spipf138Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF138 to value 0"]
impl crate::Resettable for Spipf138Spec {}
