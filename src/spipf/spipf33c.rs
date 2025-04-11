#[doc = "Register `SPIPF33C` reader"]
pub type R = crate::R<Spipf33cSpec>;
#[doc = "Register `SPIPF33C` writer"]
pub type W = crate::W<Spipf33cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf33c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf33c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf33cSpec;
impl crate::RegisterSpec for Spipf33cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf33c::R`](R) reader structure"]
impl crate::Readable for Spipf33cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf33c::W`](W) writer structure"]
impl crate::Writable for Spipf33cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF33C to value 0"]
impl crate::Resettable for Spipf33cSpec {}
