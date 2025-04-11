#[doc = "Register `I3CD0C0` reader"]
pub type R = crate::R<I3cd0c0Spec>;
#[doc = "Register `I3CD0C0` writer"]
pub type W = crate::W<I3cd0c0Spec>;
#[doc = "Field `I2CFMPLCNT` reader - I2C_FMP_LCNT"]
pub type I2cfmplcntR = crate::FieldReader<u16>;
#[doc = "Field `I2CFMPLCNT` writer - I2C_FMP_LCNT"]
pub type I2cfmplcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `I2CFMPHCNT` reader - I2C_FMP_HCNT"]
pub type I2cfmphcntR = crate::FieldReader;
#[doc = "Field `I2CFMPHCNT` writer - I2C_FMP_HCNT"]
pub type I2cfmphcntW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSVD3124` reader - These bits in SCL I2C FM plus timing register are reserved."]
pub type Rsvd3124R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - I2C_FMP_LCNT"]
    #[inline(always)]
    pub fn i2cfmplcnt(&self) -> I2cfmplcntR {
        I2cfmplcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - I2C_FMP_HCNT"]
    #[inline(always)]
    pub fn i2cfmphcnt(&self) -> I2cfmphcntR {
        I2cfmphcntR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - These bits in SCL I2C FM plus timing register are reserved."]
    #[inline(always)]
    pub fn rsvd3124(&self) -> Rsvd3124R {
        Rsvd3124R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - I2C_FMP_LCNT"]
    #[inline(always)]
    pub fn i2cfmplcnt(&mut self) -> I2cfmplcntW<I3cd0c0Spec> {
        I2cfmplcntW::new(self, 0)
    }
    #[doc = "Bits 16:23 - I2C_FMP_HCNT"]
    #[inline(always)]
    pub fn i2cfmphcnt(&mut self) -> I2cfmphcntW<I3cd0c0Spec> {
        I2cfmphcntW::new(self, 16)
    }
}
#[doc = "SCL I2C Fast Mode Plus Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0c0Spec;
impl crate::RegisterSpec for I3cd0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0c0::R`](R) reader structure"]
impl crate::Readable for I3cd0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0c0::W`](W) writer structure"]
impl crate::Writable for I3cd0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0C0 to value 0"]
impl crate::Resettable for I3cd0c0Spec {}
