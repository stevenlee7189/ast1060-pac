#[doc = "Register `TIMERG03C` reader"]
pub type R = crate::R<Timerg03cSpec>;
#[doc = "Register `TIMERG03C` writer"]
pub type W = crate::W<Timerg03cSpec>;
#[doc = "Field `ClearTMC30` reader - Clear TMC30"]
pub type ClearTmc30R = crate::FieldReader<u32>;
#[doc = "Field `ClearTMC30` writer - Clear TMC30"]
pub type ClearTmc30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Clear TMC30"]
    #[inline(always)]
    pub fn clear_tmc30(&self) -> ClearTmc30R {
        ClearTmc30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear TMC30"]
    #[inline(always)]
    pub fn clear_tmc30(&mut self) -> ClearTmc30W<Timerg03cSpec> {
        ClearTmc30W::new(self, 0)
    }
}
#[doc = "TMC30 Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`timerg03c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timerg03c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Timerg03cSpec;
impl crate::RegisterSpec for Timerg03cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timerg03c::R`](R) reader structure"]
impl crate::Readable for Timerg03cSpec {}
#[doc = "`write(|w| ..)` method takes [`timerg03c::W`](W) writer structure"]
impl crate::Writable for Timerg03cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMERG03C to value 0"]
impl crate::Resettable for Timerg03cSpec {}
