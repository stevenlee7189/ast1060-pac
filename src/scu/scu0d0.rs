#[doc = "Register `SCU0D0` reader"]
pub type R = crate::R<Scu0d0Spec>;
#[doc = "Register `SCU0D0` writer"]
pub type W = crate::W<Scu0d0Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `EnblTheReferenceClkDividerDiv13ForRTC` reader - Enable the reference clock divider (div13) for RTC"]
pub type EnblTheReferenceClkDividerDiv13forRtcR = crate::BitReader;
#[doc = "Field `EnblTheReferenceClkDividerDiv13ForRTC` writer - Enable the reference clock divider (div13) for RTC"]
pub type EnblTheReferenceClkDividerDiv13forRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JTAGRoutingSel` reader - JTAG routing selection"]
pub type JtagroutingSelR = crate::FieldReader;
#[doc = "Field `JTAGRoutingSel` writer - JTAG routing selection"]
pub type JtagroutingSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:12 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - Enable the reference clock divider (div13) for RTC"]
    #[inline(always)]
    pub fn enbl_the_reference_clk_divider_div13for_rtc(
        &self,
    ) -> EnblTheReferenceClkDividerDiv13forRtcR {
        EnblTheReferenceClkDividerDiv13forRtcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - JTAG routing selection"]
    #[inline(always)]
    pub fn jtagrouting_sel(&self) -> JtagroutingSelR {
        JtagroutingSelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu0d0Spec> {
        Reserved1W::new(self, 0)
    }
    #[doc = "Bit 13 - Enable the reference clock divider (div13) for RTC"]
    #[inline(always)]
    pub fn enbl_the_reference_clk_divider_div13for_rtc(
        &mut self,
    ) -> EnblTheReferenceClkDividerDiv13forRtcW<Scu0d0Spec> {
        EnblTheReferenceClkDividerDiv13forRtcW::new(self, 13)
    }
    #[doc = "Bits 14:15 - JTAG routing selection"]
    #[inline(always)]
    pub fn jtagrouting_sel(&mut self) -> JtagroutingSelW<Scu0d0Spec> {
        JtagroutingSelW::new(self, 14)
    }
}
#[doc = "Misc. 3 Control Register\\label{SCUREG:MISC3\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0d0Spec;
impl crate::RegisterSpec for Scu0d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0d0::R`](R) reader structure"]
impl crate::Readable for Scu0d0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0d0::W`](W) writer structure"]
impl crate::Writable for Scu0d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0D0 to value 0x10"]
impl crate::Resettable for Scu0d0Spec {
    const RESET_VALUE: u32 = 0x10;
}
