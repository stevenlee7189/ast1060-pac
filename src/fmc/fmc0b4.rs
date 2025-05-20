#[doc = "Register `FMC0B4` reader"]
pub type R = crate::R<Fmc0b4Spec>;
#[doc = "Register `FMC0B4` writer"]
pub type W = crate::W<Fmc0b4Spec>;
#[doc = "Field `SegmentsLowerBoundAddrBit27121` reader - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27121R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsLowerBoundAddrBit27121` writer - Segments Lower bound address bit\\[27:12\\]"]
pub type SegmentsLowerBoundAddrBit27121W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27121` reader - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27121R = crate::FieldReader<u16>;
#[doc = "Field `SegmentsUpperBoundAddrBit27121` writer - Segments Upper bound address bit\\[27:12\\]"]
pub type SegmentsUpperBoundAddrBit27121W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27121(&self) -> SegmentsLowerBoundAddrBit27121R {
        SegmentsLowerBoundAddrBit27121R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27121(&self) -> SegmentsUpperBoundAddrBit27121R {
        SegmentsUpperBoundAddrBit27121R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Segments Lower bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_lower_bound_addr_bit27121(
        &mut self,
    ) -> SegmentsLowerBoundAddrBit27121W<Fmc0b4Spec> {
        SegmentsLowerBoundAddrBit27121W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Segments Upper bound address bit\\[27:12\\]"]
    #[inline(always)]
    pub fn segments_upper_bound_addr_bit27121(
        &mut self,
    ) -> SegmentsUpperBoundAddrBit27121W<Fmc0b4Spec> {
        SegmentsUpperBoundAddrBit27121W::new(self, 16)
    }
}
#[doc = "Write Address Filter Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc0b4Spec;
impl crate::RegisterSpec for Fmc0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc0b4::R`](R) reader structure"]
impl crate::Readable for Fmc0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc0b4::W`](W) writer structure"]
impl crate::Writable for Fmc0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC0B4 to value 0"]
impl crate::Resettable for Fmc0b4Spec {}
