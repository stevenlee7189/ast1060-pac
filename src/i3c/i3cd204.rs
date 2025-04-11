#[doc = "Register `I3CD204` reader"]
pub type R = crate::R<I3cd204Spec>;
#[doc = "Register `I3CD204` writer"]
pub type W = crate::W<I3cd204Spec>;
#[doc = "Field `MSBPROVISIONALID` reader - MSB_PROVISIONAL_ID"]
pub type MsbprovisionalidR = crate::FieldReader<u16>;
#[doc = "Field `DYNAMICADDR` reader - DYNAMIC_ADDR"]
pub type DynamicaddrR = crate::FieldReader;
#[doc = "Field `DCRTYPE` reader - DCR_TYPE"]
pub type DcrtypeR = crate::FieldReader;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader<u16>;
#[doc = "Field `BCRTYPE` reader - BCR_TYPE"]
pub type BcrtypeR = crate::FieldReader;
#[doc = "Field `STATICADDR` reader - STATIC_ADDR"]
pub type StaticaddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - MSB_PROVISIONAL_ID"]
    #[inline(always)]
    pub fn msbprovisionalid(&self) -> MsbprovisionalidR {
        MsbprovisionalidR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 0:7 - DYNAMIC_ADDR"]
    #[inline(always)]
    pub fn dynamicaddr(&self) -> DynamicaddrR {
        DynamicaddrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DCR_TYPE"]
    #[inline(always)]
    pub fn dcrtype(&self) -> DcrtypeR {
        DcrtypeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - BCR_TYPE"]
    #[inline(always)]
    pub fn bcrtype(&self) -> BcrtypeR {
        BcrtypeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - STATIC_ADDR"]
    #[inline(always)]
    pub fn staticaddr(&self) -> StaticaddrR {
        StaticaddrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "Device Characteristic Table Location-2 of Device1 (Master Mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd204::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd204::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd204Spec;
impl crate::RegisterSpec for I3cd204Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd204::R`](R) reader structure"]
impl crate::Readable for I3cd204Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd204::W`](W) writer structure"]
impl crate::Writable for I3cd204Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD204 to value 0"]
impl crate::Resettable for I3cd204Spec {}
