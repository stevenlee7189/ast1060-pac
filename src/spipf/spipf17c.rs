#[doc = "Register `SPIPF17C` reader"]
pub type R = crate::R<Spipf17cSpec>;
#[doc = "Register `SPIPF17C` writer"]
pub type W = crate::W<Spipf17cSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf17c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf17c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf17cSpec;
impl crate::RegisterSpec for Spipf17cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf17c::R`](R) reader structure"]
impl crate::Readable for Spipf17cSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf17c::W`](W) writer structure"]
impl crate::Writable for Spipf17cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF17C to value 0"]
impl crate::Resettable for Spipf17cSpec {}
