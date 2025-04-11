#[doc = "Register `SPIPF1DC` reader"]
pub type R = crate::R<Spipf1dcSpec>;
#[doc = "Register `SPIPF1DC` writer"]
pub type W = crate::W<Spipf1dcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf1dc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf1dc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf1dcSpec;
impl crate::RegisterSpec for Spipf1dcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf1dc::R`](R) reader structure"]
impl crate::Readable for Spipf1dcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf1dc::W`](W) writer structure"]
impl crate::Writable for Spipf1dcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF1DC to value 0"]
impl crate::Resettable for Spipf1dcSpec {}
