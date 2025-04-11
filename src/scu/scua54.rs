#[doc = "Register `SCUA54` reader"]
pub type R = crate::R<Scua54Spec>;
#[doc = "Register `SCUA54` writer"]
pub type W = crate::W<Scua54Spec>;
#[doc = "Field `CM4FInstructionCacheInvalidAddr` reader - CM4F Instruction Cache Invalid Address"]
pub type Cm4finstructionCacheInvalidAddrR = crate::FieldReader<u16>;
#[doc = "Field `CM4FInstructionCacheInvalidAddr` writer - CM4F Instruction Cache Invalid Address"]
pub type Cm4finstructionCacheInvalidAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CM4FInstructionCacheInvalid` reader - CM4F Instruction Cache Invalid"]
pub type Cm4finstructionCacheInvalidR = crate::BitReader;
#[doc = "Field `CM4FInstructionCacheInvalid` writer - CM4F Instruction Cache Invalid"]
pub type Cm4finstructionCacheInvalidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM4FDataCacheInvalidAddr` reader - CM4F Data Cache Invalid Address"]
pub type Cm4fdataCacheInvalidAddrR = crate::FieldReader<u16>;
#[doc = "Field `CM4FDataCacheInvalidAddr` writer - CM4F Data Cache Invalid Address"]
pub type Cm4fdataCacheInvalidAddrW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CM4FDataCacheInvalid` reader - CM4F Data Cache Invalid"]
pub type Cm4fdataCacheInvalidR = crate::BitReader;
#[doc = "Field `CM4FDataCacheInvalid` writer - CM4F Data Cache Invalid"]
pub type Cm4fdataCacheInvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - CM4F Instruction Cache Invalid Address"]
    #[inline(always)]
    pub fn cm4finstruction_cache_invalid_addr(&self) -> Cm4finstructionCacheInvalidAddrR {
        Cm4finstructionCacheInvalidAddrR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - CM4F Instruction Cache Invalid"]
    #[inline(always)]
    pub fn cm4finstruction_cache_invalid(&self) -> Cm4finstructionCacheInvalidR {
        Cm4finstructionCacheInvalidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - CM4F Data Cache Invalid Address"]
    #[inline(always)]
    pub fn cm4fdata_cache_invalid_addr(&self) -> Cm4fdataCacheInvalidAddrR {
        Cm4fdataCacheInvalidAddrR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 31 - CM4F Data Cache Invalid"]
    #[inline(always)]
    pub fn cm4fdata_cache_invalid(&self) -> Cm4fdataCacheInvalidR {
        Cm4fdataCacheInvalidR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - CM4F Instruction Cache Invalid Address"]
    #[inline(always)]
    pub fn cm4finstruction_cache_invalid_addr(
        &mut self,
    ) -> Cm4finstructionCacheInvalidAddrW<Scua54Spec> {
        Cm4finstructionCacheInvalidAddrW::new(self, 0)
    }
    #[doc = "Bit 15 - CM4F Instruction Cache Invalid"]
    #[inline(always)]
    pub fn cm4finstruction_cache_invalid(&mut self) -> Cm4finstructionCacheInvalidW<Scua54Spec> {
        Cm4finstructionCacheInvalidW::new(self, 15)
    }
    #[doc = "Bits 16:26 - CM4F Data Cache Invalid Address"]
    #[inline(always)]
    pub fn cm4fdata_cache_invalid_addr(&mut self) -> Cm4fdataCacheInvalidAddrW<Scua54Spec> {
        Cm4fdataCacheInvalidAddrW::new(self, 16)
    }
    #[doc = "Bit 31 - CM4F Data Cache Invalid"]
    #[inline(always)]
    pub fn cm4fdata_cache_invalid(&mut self) -> Cm4fdataCacheInvalidW<Scua54Spec> {
        Cm4fdataCacheInvalidW::new(self, 31)
    }
}
#[doc = "CM4F Cache Invalidation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scua54::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scua54::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scua54Spec;
impl crate::RegisterSpec for Scua54Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scua54::R`](R) reader structure"]
impl crate::Readable for Scua54Spec {}
#[doc = "`write(|w| ..)` method takes [`scua54::W`](W) writer structure"]
impl crate::Writable for Scua54Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUA54 to value 0"]
impl crate::Resettable for Scua54Spec {}
