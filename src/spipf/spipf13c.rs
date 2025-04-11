#[doc = "Register `SPIPF13C` reader"]
pub type R = crate::R<Spipf13cSpec>;
#[doc = "Register `SPIPF13C` writer"]
pub type W = crate::W<Spipf13cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf13c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf13c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf13cSpec;
impl crate::RegisterSpec for Spipf13cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf13c::R`](R) reader structure"]
impl crate::Readable for Spipf13cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf13c::W`](W) writer structure"]
impl crate::Writable for Spipf13cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF13C to value 0"]
impl crate::Resettable for Spipf13cSpec {}
