#[doc = "Register `FMC0C0` reader"]
pub type R = crate::R<Fmc0c0Spec>;
#[doc = "Register `FMC0C0` writer"]
pub type W = crate::W<Fmc0c0Spec>;
#[doc = "Field `SegmentsLowerBoundAddrBit27124` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27124R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit27124` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27124W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27124` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27124R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27124` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27124W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27124(&self) -> SegmentsLowerBoundAddrBit27124R {
        SegmentsLowerBoundAddrBit27124R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27124(&self) -> SegmentsUpperBoundAddrBit27124R {
        SegmentsUpperBoundAddrBit27124R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27124(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit27124W<Fmc0c0Spec> {
        SegmentsLowerBoundAddrBit27124W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27124(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit27124W<Fmc0c0Spec> {
        SegmentsUpperBoundAddrBit27124W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#5\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc0c0Spec;
impl crate::RegisterSpec for Fmc0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc0c0::R`](R) reader structure"]
impl crate::Readable for Fmc0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc0c0::W`](W) writer structure"]
impl crate::Writable for Fmc0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC0C0 to value 0"]
impl crate::Resettable for Fmc0c0Spec {}
