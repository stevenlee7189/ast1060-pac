#[doc = "Register `I3C050` reader"]
pub type R = crate::R<I3c050Spec>;
#[doc = "Register `I3C050` writer"]
pub type W = crate::W<I3c050Spec>;
#[doc = "Field `DGSDAFMAX` reader - DG_SDA_FMAX"]
pub type DgsdafmaxR = crate::FieldReader;
#[doc = "Field `DGSDAFMAX` writer - DG_SDA_FMAX"]
pub type DgsdafmaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DGSDARMAX` reader - DG_SDA_RMAX"]
pub type DgsdarmaxR = crate::FieldReader;
#[doc = "Field `DGSDARMAX` writer - DG_SDA_RMAX"]
pub type DgsdarmaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DGSCLFMAX` reader - DG_SCL_FMAX"]
pub type DgsclfmaxR = crate::FieldReader;
#[doc = "Field `DGSCLFMAX` writer - DG_SCL_FMAX"]
pub type DgsclfmaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DGSCLRMAX` reader - DG_SCL_RMAX"]
pub type DgsclrmaxR = crate::FieldReader;
#[doc = "Field `DGSCLRMAX` writer - DG_SCL_RMAX"]
pub type DgsclrmaxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CDRDLYMAX` reader - CDR_DLYMAX"]
pub type CdrdlymaxR = crate::FieldReader;
#[doc = "Field `CDRDLYMAX` writer - CDR_DLYMAX"]
pub type CdrdlymaxW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CDRENDG` reader - CDR_EN_DG"]
pub type CdrendgR = crate::BitReader;
#[doc = "Field `CDRENDG` writer - CDR_EN_DG"]
pub type CdrendgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDRENMASK` reader - CDR_EN_MASK"]
pub type CdrenmaskR = crate::BitReader;
#[doc = "Field `CDRENMASK` writer - CDR_EN_MASK"]
pub type CdrenmaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - DG_SDA_FMAX"]
    #[inline(always)]
    pub fn dgsdafmax(&self) -> DgsdafmaxR {
        DgsdafmaxR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DG_SDA_RMAX"]
    #[inline(always)]
    pub fn dgsdarmax(&self) -> DgsdarmaxR {
        DgsdarmaxR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DG_SCL_FMAX"]
    #[inline(always)]
    pub fn dgsclfmax(&self) -> DgsclfmaxR {
        DgsclfmaxR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DG_SCL_RMAX"]
    #[inline(always)]
    pub fn dgsclrmax(&self) -> DgsclrmaxR {
        DgsclrmaxR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - CDR_DLYMAX"]
    #[inline(always)]
    pub fn cdrdlymax(&self) -> CdrdlymaxR {
        CdrdlymaxR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - CDR_EN_DG"]
    #[inline(always)]
    pub fn cdrendg(&self) -> CdrendgR {
        CdrendgR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - CDR_EN_MASK"]
    #[inline(always)]
    pub fn cdrenmask(&self) -> CdrenmaskR {
        CdrenmaskR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DG_SDA_FMAX"]
    #[inline(always)]
    pub fn dgsdafmax(&mut self) -> DgsdafmaxW<I3c050Spec> {
        DgsdafmaxW::new(self, 0)
    }
    #[doc = "Bits 4:7 - DG_SDA_RMAX"]
    #[inline(always)]
    pub fn dgsdarmax(&mut self) -> DgsdarmaxW<I3c050Spec> {
        DgsdarmaxW::new(self, 4)
    }
    #[doc = "Bits 8:11 - DG_SCL_FMAX"]
    #[inline(always)]
    pub fn dgsclfmax(&mut self) -> DgsclfmaxW<I3c050Spec> {
        DgsclfmaxW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DG_SCL_RMAX"]
    #[inline(always)]
    pub fn dgsclrmax(&mut self) -> DgsclrmaxW<I3c050Spec> {
        DgsclrmaxW::new(self, 12)
    }
    #[doc = "Bits 16:23 - CDR_DLYMAX"]
    #[inline(always)]
    pub fn cdrdlymax(&mut self) -> CdrdlymaxW<I3c050Spec> {
        CdrdlymaxW::new(self, 16)
    }
    #[doc = "Bit 24 - CDR_EN_DG"]
    #[inline(always)]
    pub fn cdrendg(&mut self) -> CdrendgW<I3c050Spec> {
        CdrendgW::new(self, 24)
    }
    #[doc = "Bit 25 - CDR_EN_MASK"]
    #[inline(always)]
    pub fn cdrenmask(&mut self) -> CdrenmaskW<I3c050Spec> {
        CdrenmaskW::new(self, 25)
    }
}
#[doc = "I3C5Reg0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3c050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3c050Spec;
impl crate::RegisterSpec for I3c050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3c050::R`](R) reader structure"]
impl crate::Readable for I3c050Spec {}
#[doc = "`write(|w| ..)` method takes [`i3c050::W`](W) writer structure"]
impl crate::Writable for I3c050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3C050 to value 0"]
impl crate::Resettable for I3c050Spec {}
