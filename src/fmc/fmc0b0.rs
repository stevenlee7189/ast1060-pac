#[doc = "Register `FMC0B0` reader"]
pub type R = crate::R<Fmc0b0Spec>;
#[doc = "Register `FMC0B0` writer"]
pub type W = crate::W<Fmc0b0Spec>;
#[doc = "Field `SegmentsLowerBoundAddrBit2712` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit2712R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit2712` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit2712W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit2712` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit2712R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit2712` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit2712W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit2712(&self) -> SegmentsLowerBoundAddrBit2712R {
        SegmentsLowerBoundAddrBit2712R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit2712(&self) -> SegmentsUpperBoundAddrBit2712R {
        SegmentsUpperBoundAddrBit2712R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit2712(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit2712W<Fmc0b0Spec> {
        SegmentsLowerBoundAddrBit2712W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit2712(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit2712W<Fmc0b0Spec> {
        SegmentsUpperBoundAddrBit2712W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0b0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0b0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc0b0Spec;
impl crate::RegisterSpec for Fmc0b0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc0b0::R`](R) reader structure"]
impl crate::Readable for Fmc0b0Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc0b0::W`](W) writer structure"]
impl crate::Writable for Fmc0b0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC0B0 to value 0"]
impl crate::Resettable for Fmc0b0Spec {}
