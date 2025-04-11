#[doc = "Register `SPIPF11C` reader"]
pub type R = crate::R<Spipf11cSpec>;
#[doc = "Register `SPIPF11C` writer"]
pub type W = crate::W<Spipf11cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf11c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf11c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf11cSpec;
impl crate::RegisterSpec for Spipf11cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf11c::R`](R) reader structure"]
impl crate::Readable for Spipf11cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf11c::W`](W) writer structure"]
impl crate::Writable for Spipf11cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF11C to value 0"]
impl crate::Resettable for Spipf11cSpec {}
