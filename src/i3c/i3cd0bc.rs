#[doc = "Register `I3CD0BC` reader"]
pub type R = crate::R<I3cd0bcSpec>;
#[doc = "Register `I3CD0BC` writer"]
pub type W = crate::W<I3cd0bcSpec>;
#[doc = "Field `I2CFMLCNT` reader - I2C_FM_LCNT"]
pub type I2cfmlcntR = crate::FieldReader<u16>;
#[doc = "Field `I2CFMLCNT` writer - I2C_FM_LCNT"]
pub type I2cfmlcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `I2CFMHCNT` reader - I2C_FM_HCNT"]
pub type I2cfmhcntR = crate::FieldReader<u16>;
#[doc = "Field `I2CFMHCNT` writer - I2C_FM_HCNT"]
pub type I2cfmhcntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - I2C_FM_LCNT"]
    #[inline(always)]
    pub fn i2cfmlcnt(&self) -> I2cfmlcntR {
        I2cfmlcntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - I2C_FM_HCNT"]
    #[inline(always)]
    pub fn i2cfmhcnt(&self) -> I2cfmhcntR {
        I2cfmhcntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - I2C_FM_LCNT"]
    #[inline(always)]
    pub fn i2cfmlcnt(&mut self) -> I2cfmlcntW<I3cd0bcSpec> {
        I2cfmlcntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - I2C_FM_HCNT"]
    #[inline(always)]
    pub fn i2cfmhcnt(&mut self) -> I2cfmhcntW<I3cd0bcSpec> {
        I2cfmhcntW::new(self, 16)
    }
}
#[doc = "SCL I2C Fast Mode Timing Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd0bcSpec;
impl crate::RegisterSpec for I3cd0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd0bc::R`](R) reader structure"]
impl crate::Readable for I3cd0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`i3cd0bc::W`](W) writer structure"]
impl crate::Writable for I3cd0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD0BC to value 0"]
impl crate::Resettable for I3cd0bcSpec {}
