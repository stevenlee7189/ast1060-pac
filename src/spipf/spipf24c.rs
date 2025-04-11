#[doc = "Register `SPIPF24C` reader"]
pub type R = crate::R<Spipf24cSpec>;
#[doc = "Register `SPIPF24C` writer"]
pub type W = crate::W<Spipf24cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf24c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf24c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf24cSpec;
impl crate::RegisterSpec for Spipf24cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf24c::R`](R) reader structure"]
impl crate::Readable for Spipf24cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf24c::W`](W) writer structure"]
impl crate::Writable for Spipf24cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF24C to value 0"]
impl crate::Resettable for Spipf24cSpec {}
