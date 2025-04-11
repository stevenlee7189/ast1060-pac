#[doc = "Register `SPIPF3BC` reader"]
pub type R = crate::R<Spipf3bcSpec>;
#[doc = "Register `SPIPF3BC` writer"]
pub type W = crate::W<Spipf3bcSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf3bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf3bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf3bcSpec;
impl crate::RegisterSpec for Spipf3bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf3bc::R`](R) reader structure"]
impl crate::Readable for Spipf3bcSpec {}
#[doc = "`write(|w| ..)` method takes [`spipf3bc::W`](W) writer structure"]
impl crate::Writable for Spipf3bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF3BC to value 0"]
impl crate::Resettable for Spipf3bcSpec {}
