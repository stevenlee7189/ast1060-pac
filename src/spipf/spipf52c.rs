#[doc = "Register `SPIPF52C` reader"]
pub type R = crate::R<Spipf52cSpec>;
#[doc = "Register `SPIPF52C` writer"]
pub type W = crate::W<Spipf52cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf52c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf52c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf52cSpec;
impl crate::RegisterSpec for Spipf52cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf52c::R`](R) reader structure"]
impl crate::Readable for Spipf52cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf52c::W`](W) writer structure"]
impl crate::Writable for Spipf52cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF52C to value 0"]
impl crate::Resettable for Spipf52cSpec {}
