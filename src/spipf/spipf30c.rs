#[doc = "Register `SPIPF30C` reader"]
pub type R = crate::R<Spipf30cSpec>;
#[doc = "Register `SPIPF30C` writer"]
pub type W = crate::W<Spipf30cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf30c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf30c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf30cSpec;
impl crate::RegisterSpec for Spipf30cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf30c::R`](R) reader structure"]
impl crate::Readable for Spipf30cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf30c::W`](W) writer structure"]
impl crate::Writable for Spipf30cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF30C to value 0"]
impl crate::Resettable for Spipf30cSpec {}
