#[doc = "Register `SPIPF564` reader"]
pub type R = crate::R<Spipf564Spec>;
#[doc = "Register `SPIPF564` writer"]
pub type W = crate::W<Spipf564Spec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf564::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf564::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf564Spec;
impl crate::RegisterSpec for Spipf564Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf564::R`](R) reader structure"]
impl crate::Readable for Spipf564Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf564::W`](W) writer structure"]
impl crate::Writable for Spipf564Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF564 to value 0"]
impl crate::Resettable for Spipf564Spec {}
