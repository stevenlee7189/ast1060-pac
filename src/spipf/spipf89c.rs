#[doc = "Register `SPIPF89C` reader"]
pub type R = crate::R<Spipf89cSpec>;
#[doc = "Register `SPIPF89C` writer"]
pub type W = crate::W<Spipf89cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf89c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf89c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf89cSpec;
impl crate::RegisterSpec for Spipf89cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf89c::R`](R) reader structure"]
impl crate::Readable for Spipf89cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf89c::W`](W) writer structure"]
impl crate::Writable for Spipf89cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF89C to value 0"]
impl crate::Resettable for Spipf89cSpec {}
