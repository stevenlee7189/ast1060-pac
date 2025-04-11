#[doc = "Register `SPIPF79C` reader"]
pub type R = crate::R<Spipf79cSpec>;
#[doc = "Register `SPIPF79C` writer"]
pub type W = crate::W<Spipf79cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf79c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf79c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf79cSpec;
impl crate::RegisterSpec for Spipf79cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf79c::R`](R) reader structure"]
impl crate::Readable for Spipf79cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf79c::W`](W) writer structure"]
impl crate::Writable for Spipf79cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF79C to value 0"]
impl crate::Resettable for Spipf79cSpec {}
