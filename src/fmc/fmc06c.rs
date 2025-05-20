#[doc = "Register `FMC06C` reader"]
pub type R = crate::R<Fmc06cSpec>;
#[doc = "Register `FMC06C` writer"]
pub type W = crate::W<Fmc06cSpec>;
#[doc = "Field `RestartReg` reader - Restart register"]
pub type RestartRegR = crate::FieldReader<u16>;
#[doc = "Field `RestartReg` writer - Restart register"]
pub type RestartRegW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Restart register"]
    #[inline(always)]
    pub fn restart_reg(&self) -> RestartRegR {
        RestartRegR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Restart register"]
    #[inline(always)]
    pub fn restart_reg(&mut self) -> RestartRegW<Fmc06cSpec> {
        RestartRegW::new(self, 0)
    }
}
#[doc = "FMC\\_WDT2 Timer Restart Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc06cSpec;
impl crate::RegisterSpec for Fmc06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc06c::R`](R) reader structure"]
impl crate::Readable for Fmc06cSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc06c::W`](W) writer structure"]
impl crate::Writable for Fmc06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC06C to value 0"]
impl crate::Resettable for Fmc06cSpec {}
