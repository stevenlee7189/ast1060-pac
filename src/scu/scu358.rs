#[doc = "Register `SCU358` reader"]
pub type R = crate::R<Scu358Spec>;
#[doc = "Register `SCU358` writer"]
pub type W = crate::W<Scu358Spec>;
#[doc = "Field `MACRGMIITXCLK100MClkOutputDelay` reader - MAC RGMII_TXCLK 100M clock output delay"]
pub type Macrgmiitxclk100mclkOutputDelayR = crate::FieldReader;
#[doc = "Field `MACRGMIITXCLK100MClkOutputDelay` writer - MAC RGMII_TXCLK 100M clock output delay"]
pub type Macrgmiitxclk100mclkOutputDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MACRGMIIRXCLK100MClkInputDelay` reader - MAC RGMII_RXCLK 100M clock input delay"]
pub type Macrgmiirxclk100mclkInputDelayR = crate::FieldReader;
#[doc = "Field `MACRGMIIRXCLK100MClkInputDelay` writer - MAC RGMII_RXCLK 100M clock input delay"]
pub type Macrgmiirxclk100mclkInputDelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MACRXCLK100MClkTreeInverePhase` reader - MAC RXCLK 100M clock tree invere phase"]
pub type Macrxclk100mclkTreeInverePhaseR = crate::BitReader;
#[doc = "Field `MACRXCLK100MClkTreeInverePhase` writer - MAC RXCLK 100M clock tree invere phase"]
pub type Macrxclk100mclkTreeInverePhaseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - MAC RGMII_TXCLK 100M clock output delay"]
    #[inline(always)]
    pub fn macrgmiitxclk100mclk_output_delay(&self) -> Macrgmiitxclk100mclkOutputDelayR {
        Macrgmiitxclk100mclkOutputDelayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - MAC RGMII_RXCLK 100M clock input delay"]
    #[inline(always)]
    pub fn macrgmiirxclk100mclk_input_delay(&self) -> Macrgmiirxclk100mclkInputDelayR {
        Macrgmiirxclk100mclkInputDelayR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - MAC RXCLK 100M clock tree invere phase"]
    #[inline(always)]
    pub fn macrxclk100mclk_tree_invere_phase(&self) -> Macrxclk100mclkTreeInverePhaseR {
        Macrxclk100mclkTreeInverePhaseR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - MAC RGMII_TXCLK 100M clock output delay"]
    #[inline(always)]
    pub fn macrgmiitxclk100mclk_output_delay(
        &mut self,
    ) -> Macrgmiitxclk100mclkOutputDelayW<Scu358Spec> {
        Macrgmiitxclk100mclkOutputDelayW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu358Spec> {
        Reserved3W::new(self, 6)
    }
    #[doc = "Bits 12:17 - MAC RGMII_RXCLK 100M clock input delay"]
    #[inline(always)]
    pub fn macrgmiirxclk100mclk_input_delay(
        &mut self,
    ) -> Macrgmiirxclk100mclkInputDelayW<Scu358Spec> {
        Macrgmiirxclk100mclkInputDelayW::new(self, 12)
    }
    #[doc = "Bits 18:23 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu358Spec> {
        Reserved2W::new(self, 18)
    }
    #[doc = "Bit 24 - MAC RXCLK 100M clock tree invere phase"]
    #[inline(always)]
    pub fn macrxclk100mclk_tree_invere_phase(
        &mut self,
    ) -> Macrxclk100mclkTreeInverePhaseW<Scu358Spec> {
        Macrxclk100mclkTreeInverePhaseW::new(self, 24)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu358Spec> {
        Reserved1W::new(self, 25)
    }
}
#[doc = "MAC Interface Clock Delay 100M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu358::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu358::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu358Spec;
impl crate::RegisterSpec for Scu358Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu358::R`](R) reader structure"]
impl crate::Readable for Scu358Spec {}
#[doc = "`write(|w| ..)` method takes [`scu358::W`](W) writer structure"]
impl crate::Writable for Scu358Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU358 to value 0"]
impl crate::Resettable for Scu358Spec {}
