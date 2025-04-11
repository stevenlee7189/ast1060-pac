#[doc = "Register `SPIPF35C` reader"]
pub type R = crate::R<Spipf35cSpec>;
#[doc = "Register `SPIPF35C` writer"]
pub type W = crate::W<Spipf35cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf35c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf35c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf35cSpec;
impl crate::RegisterSpec for Spipf35cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf35c::R`](R) reader structure"]
impl crate::Readable for Spipf35cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf35c::W`](W) writer structure"]
impl crate::Writable for Spipf35cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF35C to value 0"]
impl crate::Resettable for Spipf35cSpec {}
