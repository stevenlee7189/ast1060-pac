#[doc = "Register `I3CD008` reader"]
pub type R = crate::R<I3cd008Spec>;
#[doc = "Register `I3CD008` writer"]
pub type W = crate::W<I3cd008Spec>;
#[doc = "Field `RoleOfI3CCtrl` reader - Role of I3C controller"]
pub type RoleOfI3cctrlR = crate::FieldReader;
#[doc = "Field `HDRDDRXfers` reader - HDR-DDR transfers"]
pub type HdrddrxfersR = crate::BitReader;
#[doc = "Field `HDRTSXfers` reader - HDR-TS transfers"]
pub type HdrtsxfersR = crate::BitReader;
#[doc = "Field `CLOCKPERIOD` reader - Reflects the IC_CLK_PERIOD Configurable Parameter"]
pub type ClockperiodR = crate::FieldReader;
#[doc = "Field `HDRTXCLOCKPERIOD` reader - Reflects the IC_HDR_TX_CLK_PERIOD Configurable Parameter. n"]
pub type HdrtxclockperiodR = crate::FieldReader;
#[doc = "Field `DMAEN` reader - Specifies whether controller is configured to have DMA handshaking interface. n"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `SLVHJCAP` reader - Specifies slave's capability to initiate Hot-join request n"]
pub type SlvhjcapR = crate::BitReader;
#[doc = "Field `SLVIBICAP` reader - Specifies slave's capability to initiate slave interrupt requests. n"]
pub type SlvibicapR = crate::BitReader;
#[doc = "Field `RSVD3120` reader - These bits in Hardware Capabilities Register are reserved. It will always return 0. n"]
pub type Rsvd3120R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:2 - Role of I3C controller"]
    #[inline(always)]
    pub fn role_of_i3cctrl(&self) -> RoleOfI3cctrlR {
        RoleOfI3cctrlR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - HDR-DDR transfers"]
    #[inline(always)]
    pub fn hdrddrxfers(&self) -> HdrddrxfersR {
        HdrddrxfersR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HDR-TS transfers"]
    #[inline(always)]
    pub fn hdrtsxfers(&self) -> HdrtsxfersR {
        HdrtsxfersR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:10 - Reflects the IC_CLK_PERIOD Configurable Parameter"]
    #[inline(always)]
    pub fn clockperiod(&self) -> ClockperiodR {
        ClockperiodR::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bits 11:16 - Reflects the IC_HDR_TX_CLK_PERIOD Configurable Parameter. n"]
    #[inline(always)]
    pub fn hdrtxclockperiod(&self) -> HdrtxclockperiodR {
        HdrtxclockperiodR::new(((self.bits >> 11) & 0x3f) as u8)
    }
    #[doc = "Bit 17 - Specifies whether controller is configured to have DMA handshaking interface. n"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Specifies slave's capability to initiate Hot-join request n"]
    #[inline(always)]
    pub fn slvhjcap(&self) -> SlvhjcapR {
        SlvhjcapR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Specifies slave's capability to initiate slave interrupt requests. n"]
    #[inline(always)]
    pub fn slvibicap(&self) -> SlvibicapR {
        SlvibicapR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - These bits in Hardware Capabilities Register are reserved. It will always return 0. n"]
    #[inline(always)]
    pub fn rsvd3120(&self) -> Rsvd3120R {
        Rsvd3120R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "Hardware Capability register\n\nYou can [`read`](crate::Reg::read) this register and get [`i3cd008::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3cd008::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3cd008Spec;
impl crate::RegisterSpec for I3cd008Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3cd008::R`](R) reader structure"]
impl crate::Readable for I3cd008Spec {}
#[doc = "`write(|w| ..)` method takes [`i3cd008::W`](W) writer structure"]
impl crate::Writable for I3cd008Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CD008 to value 0"]
impl crate::Resettable for I3cd008Spec {}
