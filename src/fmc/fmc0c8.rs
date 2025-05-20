#[doc = "Register `FMC0C8` reader"]
pub type R = crate::R<Fmc0c8Spec>;
#[doc = "Register `FMC0C8` writer"]
pub type W = crate::W<Fmc0c8Spec>;
#[doc = "Field `SegmentsLowerBoundAddrBit27126` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27126R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit27126` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27126W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27126` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27126R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27126` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27126W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27126(&self) -> SegmentsLowerBoundAddrBit27126R {
        SegmentsLowerBoundAddrBit27126R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27126(&self) -> SegmentsUpperBoundAddrBit27126R {
        SegmentsUpperBoundAddrBit27126R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27126(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit27126W<Fmc0c8Spec> {
        SegmentsLowerBoundAddrBit27126W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27126(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit27126W<Fmc0c8Spec> {
        SegmentsUpperBoundAddrBit27126W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#7\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0c8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0c8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc0c8Spec;
impl crate::RegisterSpec for Fmc0c8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc0c8::R`](R) reader structure"]
impl crate::Readable for Fmc0c8Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc0c8::W`](W) writer structure"]
impl crate::Writable for Fmc0c8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC0C8 to value 0"]
impl crate::Resettable for Fmc0c8Spec {}
