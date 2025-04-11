#[doc = "Register `MI3C6C` reader"]
pub type R = crate::R<Mi3c6cSpec>;
#[doc = "Register `MI3C6C` writer"]
pub type W = crate::W<Mi3c6cSpec>;
#[doc = "Field `Debug32` reader - Indicates the Enable status of the controller."]
pub type Debug32R = crate::BitReader;
#[doc = "Field `Debug321` reader - Indicates the Enable status of the controller."]
pub type Debug321R = crate::BitReader;
#[doc = "Field `Debug33` reader - Indicates if Periodic Poll command is in progress."]
pub type Debug33R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Debug34` reader - Indicates Periodic Poll slot tick change pulse."]
pub type Debug34R = crate::BitReader;
#[doc = "Field `RSVD1` reader - reserved"]
pub type Rsvd1R = crate::FieldReader<u16>;
#[doc = "Field `Debug3935` reader - Indicates Slave Transfer State."]
pub type Debug3935R = crate::FieldReader;
#[doc = "Field `Debug4340` reader - Indicates Slave Command TID."]
pub type Debug4340R = crate::FieldReader;
#[doc = "Field `Scloe` reader - SCL output enable signal."]
pub type ScloeR = crate::BitReader;
#[doc = "Field `Scloe1` reader - SCL output enable signal."]
pub type Scloe1R = crate::BitReader;
#[doc = "Field `Sclout` reader - SCL output signal."]
pub type ScloutR = crate::BitReader;
#[doc = "Field `Sclout1` reader - SCL output signal."]
pub type Sclout1R = crate::BitReader;
#[doc = "Field `Sclpullupen` reader - SCL pullup enable signal."]
pub type SclpullupenR = crate::BitReader;
#[doc = "Field `Sclpullupen1` reader - SCL pullup enable signal."]
pub type Sclpullupen1R = crate::BitReader;
#[doc = "Field `Sclin` reader - SCL input signal."]
pub type SclinR = crate::BitReader;
#[doc = "Field `Sclin1` reader - SCL input signal."]
pub type Sclin1R = crate::BitReader;
#[doc = "Field `Sdaoe` reader - SDA output enable signal."]
pub type SdaoeR = crate::BitReader;
#[doc = "Field `Sdaoe1` reader - SDA output enable signal."]
pub type Sdaoe1R = crate::BitReader;
#[doc = "Field `Sdaout` reader - SDA output signal."]
pub type SdaoutR = crate::BitReader;
#[doc = "Field `Sdaout1` reader - SDA output signal."]
pub type Sdaout1R = crate::BitReader;
#[doc = "Field `Sdapullupen` reader - SDA pullup enable signal."]
pub type SdapullupenR = crate::BitReader;
#[doc = "Field `Sdapullupen1` reader - SDA pullup enable signal."]
pub type Sdapullupen1R = crate::BitReader;
#[doc = "Field `Sdain` reader - SDA input signal."]
pub type SdainR = crate::BitReader;
#[doc = "Field `Sdain1` reader - SDA input signal."]
pub type Sdain1R = crate::BitReader;
#[doc = "Field `RSVD` reader - reserved"]
pub type RsvdR = crate::FieldReader;
#[doc = "Field `RSVD2` reader - reserved"]
pub type Rsvd2R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Indicates the Enable status of the controller."]
    #[inline(always)]
    pub fn debug32(&self) -> Debug32R {
        Debug32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 0 - Indicates the Enable status of the controller."]
    #[inline(always)]
    pub fn debug321(&self) -> Debug321R {
        Debug321R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates if Periodic Poll command is in progress."]
    #[inline(always)]
    pub fn debug33(&self) -> Debug33R {
        Debug33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 1:2 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 2 - Indicates Periodic Poll slot tick change pulse."]
    #[inline(always)]
    pub fn debug34(&self) -> Debug34R {
        Debug34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:15 - reserved"]
    #[inline(always)]
    pub fn rsvd1(&self) -> Rsvd1R {
        Rsvd1R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 3:7 - Indicates Slave Transfer State."]
    #[inline(always)]
    pub fn debug3935(&self) -> Debug3935R {
        Debug3935R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates Slave Command TID."]
    #[inline(always)]
    pub fn debug4340(&self) -> Debug4340R {
        Debug4340R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - SCL output enable signal."]
    #[inline(always)]
    pub fn scloe(&self) -> ScloeR {
        ScloeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 16 - SCL output enable signal."]
    #[inline(always)]
    pub fn scloe1(&self) -> Scloe1R {
        Scloe1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCL output signal."]
    #[inline(always)]
    pub fn sclout(&self) -> ScloutR {
        ScloutR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 17 - SCL output signal."]
    #[inline(always)]
    pub fn sclout1(&self) -> Sclout1R {
        Sclout1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCL pullup enable signal."]
    #[inline(always)]
    pub fn sclpullupen(&self) -> SclpullupenR {
        SclpullupenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 18 - SCL pullup enable signal."]
    #[inline(always)]
    pub fn sclpullupen1(&self) -> Sclpullupen1R {
        Sclpullupen1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SCL input signal."]
    #[inline(always)]
    pub fn sclin(&self) -> SclinR {
        SclinR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 19 - SCL input signal."]
    #[inline(always)]
    pub fn sclin1(&self) -> Sclin1R {
        Sclin1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SDA output enable signal."]
    #[inline(always)]
    pub fn sdaoe(&self) -> SdaoeR {
        SdaoeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 20 - SDA output enable signal."]
    #[inline(always)]
    pub fn sdaoe1(&self) -> Sdaoe1R {
        Sdaoe1R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SDA output signal."]
    #[inline(always)]
    pub fn sdaout(&self) -> SdaoutR {
        SdaoutR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 21 - SDA output signal."]
    #[inline(always)]
    pub fn sdaout1(&self) -> Sdaout1R {
        Sdaout1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDA pullup enable signal."]
    #[inline(always)]
    pub fn sdapullupen(&self) -> SdapullupenR {
        SdapullupenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 22 - SDA pullup enable signal."]
    #[inline(always)]
    pub fn sdapullupen1(&self) -> Sdapullupen1R {
        Sdapullupen1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDA input signal."]
    #[inline(always)]
    pub fn sdain(&self) -> SdainR {
        SdainR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 23 - SDA input signal."]
    #[inline(always)]
    pub fn sdain1(&self) -> Sdain1R {
        Sdain1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - reserved"]
    #[inline(always)]
    pub fn rsvd2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "I3C6Dbg2 (Master)\n\nYou can [`read`](crate::Reg::read) this register and get [`mi3c6c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mi3c6c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mi3c6cSpec;
impl crate::RegisterSpec for Mi3c6cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi3c6c::R`](R) reader structure"]
impl crate::Readable for Mi3c6cSpec {}
#[doc = "`write(|w| ..)` method takes [`mi3c6c::W`](W) writer structure"]
impl crate::Writable for Mi3c6cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MI3C6C to value 0"]
impl crate::Resettable for Mi3c6cSpec {}
