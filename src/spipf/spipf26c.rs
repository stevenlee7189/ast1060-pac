#[doc = "Register `SPIPF26C` reader"]
pub type R = crate::R<Spipf26cSpec>;
#[doc = "Register `SPIPF26C` writer"]
pub type W = crate::W<Spipf26cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf26c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf26c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf26cSpec;
impl crate::RegisterSpec for Spipf26cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf26c::R`](R) reader structure"]
impl crate::Readable for Spipf26cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf26c::W`](W) writer structure"]
impl crate::Writable for Spipf26cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF26C to value 0"]
impl crate::Resettable for Spipf26cSpec {}
