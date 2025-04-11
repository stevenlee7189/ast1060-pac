#[doc = "Register `I3CD060` reader"]
pub type R = crate::R<I3cd060Spec>;
#[doc = "Register `I3CD060` writer"]
pub type W = crate::W<I3cd060Spec>;
#[doc = "Field `PDEVCHARTABLESTARTADDR` reader - P_DEV_CHAR_TABLE_START_ADDR"]
pub type PdevchartablestartaddrR = crate::FieldReader<u16>;
#[doc = "Field `DEVCHARTABLEDEPTH` reader - DEV_CHAR_TABLE_DEPTH"]
pub type DevchartabledepthR = crate::FieldReader;
#[doc = "Field `PRESENTDEVCHARTABLEINDX` reader - PRESENT_DEV_CHAR_TABLE_INDX"]
pub type PresentdevchartableindxR = crate::FieldReader;
#[doc = "Field `PRESENTDEVCHARTABLEINDX` writer - PRESENT_DEV_CHAR_TABLE_INDX"]
pub type PresentdevchartableindxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RSVD3124` reader - RSVD_31_24"]
pub type Rsvd3124R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - P_DEV_CHAR_TABLE_START_ADDR"]
    #[inline(always)]
    pub fn pdevchartablestartaddr(&self) -> PdevchartablestartaddrR {
        PdevchartablestartaddrR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:18 - DEV_CHAR_TABLE_DEPTH"]
    #[inline(always)]
    pub fn devchartabledepth(&self) -> DevchartabledepthR {
        DevchartabledepthR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bits 19:21 - PRESENT_DEV_CHAR_TABLE_INDX"]
    #[inline(always)]
    pub fn presentdevchartableindx(&self) -> PresentdevchartableindxR {
        PresentdevchartableindxR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:31 - RSVD_31_24"]
    #[inline(always)]
    pub fn rsvd3124(&self) -> Rsvd3124R {
        Rsvd3124R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 19:21 - PRESENT_DEV_CHAR_TABLE_INDX"]
    #[inline(always)]
    pub fn presentdevchartableindx(&mut self) -> PresentdevchartableindxW<I3cd060Spec> {
        PresentdevchartableindxW::new(self, 19)
    }
}
#[doc = "Pointer for Device Characteristics Table\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd060Spec;
impl crate::RegisterSpec for I3cd060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd060::R`](R) reader structure"]
impl crate::Readable for I3cd060Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd060::W`](W) writer structure"]
impl crate::Writable for I3cd060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD060 to value 0"]
impl crate::Resettable for I3cd060Spec {}
