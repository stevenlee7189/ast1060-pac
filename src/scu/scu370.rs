#[doc = "Register `SCU370` reader"]
pub type R = crate::R<Scu370Spec>;
#[doc = "Register `SCU370` writer"]
pub type W = crate::W<Scu370Spec>;
#[doc = "Field `EnblDutyCalculatingRingClk` reader - Enable duty calculating ring clock"]
pub type EnblDutyCalculatingRingClkR = crate::BitReader;
#[doc = "Field `EnblDutyCalculatingRingClk` writer - Enable duty calculating ring clock"]
pub type EnblDutyCalculatingRingClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RingClkSelect` reader - Ring clock select"]
pub type RingClkSelectR = crate::FieldReader;
#[doc = "Field `RingClkSelect` writer - Ring clock select"]
pub type RingClkSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `StartCalculateDuty` reader - Start calculate duty"]
pub type StartCalculateDutyR = crate::BitReader;
#[doc = "Field `StartCalculateDuty` writer - Start calculate duty"]
pub type StartCalculateDutyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `RGMII1TXCKDutyCountDoneSts` reader - RGMII1TXCK Duty count done status"]
pub type Rgmii1txckdutyCountDoneStsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable duty calculating ring clock"]
    #[inline(always)]
    pub fn enbl_duty_calculating_ring_clk(&self) -> EnblDutyCalculatingRingClkR {
        EnblDutyCalculatingRingClkR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Ring clock select"]
    #[inline(always)]
    pub fn ring_clk_select(&self) -> RingClkSelectR {
        RingClkSelectR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Start calculate duty"]
    #[inline(always)]
    pub fn start_calculate_duty(&self) -> StartCalculateDutyR {
        StartCalculateDutyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bit 16 - RGMII1TXCK Duty count done status"]
    #[inline(always)]
    pub fn rgmii1txckduty_count_done_sts(&self) -> Rgmii1txckdutyCountDoneStsR {
        Rgmii1txckdutyCountDoneStsR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable duty calculating ring clock"]
    #[inline(always)]
    pub fn enbl_duty_calculating_ring_clk(&mut self) -> EnblDutyCalculatingRingClkW<Scu370Spec> {
        EnblDutyCalculatingRingClkW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Ring clock select"]
    #[inline(always)]
    pub fn ring_clk_select(&mut self) -> RingClkSelectW<Scu370Spec> {
        RingClkSelectW::new(self, 1)
    }
    #[doc = "Bit 3 - Start calculate duty"]
    #[inline(always)]
    pub fn start_calculate_duty(&mut self) -> StartCalculateDutyW<Scu370Spec> {
        StartCalculateDutyW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu370Spec> {
        Reserved2W::new(self, 4)
    }
}
#[doc = "Clock Duty Measurement Control\n\nYou can [`read`](crate::Reg::read) this register and get [`scu370::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu370::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu370Spec;
impl crate::RegisterSpec for Scu370Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu370::R`](R) reader structure"]
impl crate::Readable for Scu370Spec {}
#[doc = "`write(|w| ..)` method takes [`scu370::W`](W) writer structure"]
impl crate::Writable for Scu370Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU370 to value 0"]
impl crate::Resettable for Scu370Spec {}
