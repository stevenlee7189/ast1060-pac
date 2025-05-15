#[doc = "Register `SPIPFWA[%s]` reader"]
pub type R = crate::R<SpipfwaSpec>;
#[doc = "Register `SPIPFWA[%s]` writer"]
pub type W = crate::W<SpipfwaSpec>;
impl W {}
#[doc = "Write Address Table (\\hlink{SPIPF000\n\nYou can [`read`](crate::Reg::read) this register and get [`spipfwa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipfwa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpipfwaSpec;
impl crate::RegisterSpec for SpipfwaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipfwa::R`](R) reader structure"]
impl crate::Readable for SpipfwaSpec {}
#[doc = "`write(|w| ..)` method takes [`spipfwa::W`](W) writer structure"]
impl crate::Writable for SpipfwaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPFWA[%s] to value 0"]
impl crate::Resettable for SpipfwaSpec {}
