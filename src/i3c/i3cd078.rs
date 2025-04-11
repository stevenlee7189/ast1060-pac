#[doc = "Register `I3CD078` reader"]
pub type R = crate::R<I3cd078Spec>;
#[doc = "Register `I3CD078` writer"]
pub type W = crate::W<I3cd078Spec>;
#[doc = "Field `BCR` reader - BCR"]
pub type BcrR = crate::FieldReader;
#[doc = "Field `DCR` reader - DCR"]
pub type DcrR = crate::FieldReader;
#[doc = "Field `HDRCAP` reader - HDR_CAP"]
pub type HdrcapR = crate::FieldReader;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - BCR"]
    #[inline(always)]
    pub fn bcr(&self) -> BcrR {
        BcrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DCR"]
    #[inline(always)]
    pub fn dcr(&self) -> DcrR {
        DcrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - HDR_CAP"]
    #[inline(always)]
    pub fn hdrcap(&self) -> HdrcapR {
        HdrcapR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "I3C Slave Characteristic Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd078Spec;
impl crate::RegisterSpec for I3cd078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd078::R`](R) reader structure"]
impl crate::Readable for I3cd078Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd078::W`](W) writer structure"]
impl crate::Writable for I3cd078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD078 to value 0"]
impl crate::Resettable for I3cd078Spec {}
