#[doc = "Register `FMC0CC` reader"]
pub type R = crate::R<Fmc0ccSpec>;
#[doc = "Register `FMC0CC` writer"]
pub type W = crate::W<Fmc0ccSpec>;
#[doc = "Field `SegmentsLowerBoundAddrBit27127` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27127R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit27127` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27127W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27127` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27127R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27127` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27127W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27127(&self) -> SegmentsLowerBoundAddrBit27127R {
        SegmentsLowerBoundAddrBit27127R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27127(&self) -> SegmentsUpperBoundAddrBit27127R {
        SegmentsUpperBoundAddrBit27127R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27127(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit27127W<Fmc0ccSpec> {
        SegmentsLowerBoundAddrBit27127W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27127(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit27127W<Fmc0ccSpec> {
        SegmentsUpperBoundAddrBit27127W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc0ccSpec;
impl crate::RegisterSpec for Fmc0ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc0cc::R`](R) reader structure"]
impl crate::Readable for Fmc0ccSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc0cc::W`](W) writer structure"]
impl crate::Writable for Fmc0ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC0CC to value 0"]
impl crate::Resettable for Fmc0ccSpec {}
