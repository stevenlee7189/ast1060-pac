#[doc = "Register `SCUA1C` reader"]
pub type R = crate::R<Scua1cSpec>;
#[doc = "Register `SCUA1C` writer"]
pub type W = crate::W<Scua1cSpec>;
#[doc = "Field `TheUpperBoundSRAMBaseAddrForCM4FData` reader - The upper bound SRAM base address for CM4F data"]
pub type TheUpperBoundSrambaseAddrForCm4fdataR = crate::FieldReader<u32>;
#[doc = "Field `TheUpperBoundSRAMBaseAddrForCM4FData` writer - The upper bound SRAM base address for CM4F data"]
pub type TheUpperBoundSrambaseAddrForCm4fdataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The upper bound SRAM base address for CM4F data"]
    #[inline(always)]
    pub fn the_upper_bound_srambase_addr_for_cm4fdata(
        &self,
    ) -> TheUpperBoundSrambaseAddrForCm4fdataR {
        TheUpperBoundSrambaseAddrForCm4fdataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The upper bound SRAM base address for CM4F data"]
    #[inline(always)]
    pub fn the_upper_bound_srambase_addr_for_cm4fdata(
        &mut self,
    ) -> TheUpperBoundSrambaseAddrForCm4fdataW<Scua1cSpec> {
        TheUpperBoundSrambaseAddrForCm4fdataW::new(self, 0)
    }
}
#[doc = "CM4F Data Memory Address Limit Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scua1cSpec;
impl crate::RegisterSpec for Scua1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scua1c::R`](R) reader structure"]
impl crate::Readable for Scua1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scua1c::W`](W) writer structure"]
impl crate::Writable for Scua1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUA1C to value 0x2000_0000"]
impl crate::Resettable for Scua1cSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
