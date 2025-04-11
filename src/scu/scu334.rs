#[doc = "Register `SCU334` reader"]
pub type R = crate::R<Scu334Spec>;
#[doc = "Register `SCU334` writer"]
pub type W = crate::W<Scu334Spec>;
#[doc = "Field `LowerLimit` reader - Lower Limit"]
pub type LowerLimitR = crate::FieldReader<u16>;
#[doc = "Field `LowerLimit` writer - Lower Limit"]
pub type LowerLimitW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `Reserved1` reader - Reserved (0)"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `UpperLimit` reader - Upper Limit"]
pub type UpperLimitR = crate::FieldReader<u16>;
#[doc = "Field `UpperLimit` writer - Upper Limit"]
pub type UpperLimitW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Lower Limit"]
    #[inline(always)]
    pub fn lower_limit(&self) -> LowerLimitR {
        LowerLimitR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 14:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Upper Limit"]
    #[inline(always)]
    pub fn upper_limit(&self) -> UpperLimitR {
        UpperLimitR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Lower Limit"]
    #[inline(always)]
    pub fn lower_limit(&mut self) -> LowerLimitW<Scu334Spec> {
        LowerLimitW::new(self, 0)
    }
    #[doc = "Bits 16:29 - Upper Limit"]
    #[inline(always)]
    pub fn upper_limit(&mut self) -> UpperLimitW<Scu334Spec> {
        UpperLimitW::new(self, 16)
    }
}
#[doc = "Frequency counter comparison range\n\nYou can [`read`](crate::Reg::read) this register and get [`scu334::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu334::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu334Spec;
impl crate::RegisterSpec for Scu334Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu334::R`](R) reader structure"]
impl crate::Readable for Scu334Spec {}
#[doc = "`write(|w| ..)` method takes [`scu334::W`](W) writer structure"]
impl crate::Writable for Scu334Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU334 to value 0"]
impl crate::Resettable for Scu334Spec {}
