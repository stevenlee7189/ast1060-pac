#[doc = "Register `SPIPF59C` reader"]
pub type R = crate::R<Spipf59cSpec>;
#[doc = "Register `SPIPF59C` writer"]
pub type W = crate::W<Spipf59cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf59c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf59c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf59cSpec;
impl crate::RegisterSpec for Spipf59cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf59c::R`](R) reader structure"]
impl crate::Readable for Spipf59cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf59c::W`](W) writer structure"]
impl crate::Writable for Spipf59cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF59C to value 0"]
impl crate::Resettable for Spipf59cSpec {}
