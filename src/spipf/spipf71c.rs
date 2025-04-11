#[doc = "Register `SPIPF71C` reader"]
pub type R = crate::R<Spipf71cSpec>;
#[doc = "Register `SPIPF71C` writer"]
pub type W = crate::W<Spipf71cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf71c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf71c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf71cSpec;
impl crate::RegisterSpec for Spipf71cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf71c::R`](R) reader structure"]
impl crate::Readable for Spipf71cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf71c::W`](W) writer structure"]
impl crate::Writable for Spipf71cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF71C to value 0"]
impl crate::Resettable for Spipf71cSpec {}
