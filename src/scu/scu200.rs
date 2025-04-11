#[doc = "Register `SCU200` reader"]
pub type R = crate::R<Scu200Spec>;
#[doc = "Register `SCU200` writer"]
pub type W = crate::W<Scu200Spec>;
#[doc = "Field `HPLLNumeratorM` reader - H-PLL Numerator (M)"]
pub type HpllnumeratorMR = crate::FieldReader<u16>;
#[doc = "Field `HPLLNumeratorM` writer - H-PLL Numerator (M)"]
pub type HpllnumeratorMW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `HPLLDenumeratorN` reader - H-PLL Denumerator (N)"]
pub type HplldenumeratorNR = crate::FieldReader;
#[doc = "Field `HPLLDenumeratorN` writer - H-PLL Denumerator (N)"]
pub type HplldenumeratorNW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HPLLPostDividerP` reader - H-PLL Post Divider (P)"]
pub type HpllpostDividerPR = crate::FieldReader;
#[doc = "Field `HPLLPostDividerP` writer - H-PLL Post Divider (P)"]
pub type HpllpostDividerPW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TurnOffHPLL` reader - Turn off H-PLL"]
pub type TurnOffHpllR = crate::BitReader;
#[doc = "Field `TurnOffHPLL` writer - Turn off H-PLL"]
pub type TurnOffHpllW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHPLLBypassMode` reader - Enable H-PLL bypass mode"]
pub type EnblHpllbypassModeR = crate::BitReader;
#[doc = "Field `EnblHPLLBypassMode` writer - Enable H-PLL bypass mode"]
pub type EnblHpllbypassModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblHPLLRst` reader - Enable H-PLL reset"]
pub type EnblHpllrstR = crate::BitReader;
#[doc = "Field `EnblHPLLRst` writer - Enable H-PLL reset"]
pub type EnblHpllrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPLLParameters` reader - H-PLL parameters"]
pub type HpllparametersR = crate::FieldReader;
#[doc = "Field `HPLLParameters` writer - H-PLL parameters"]
pub type HpllparametersW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:12 - H-PLL Numerator (M)"]
    #[inline(always)]
    pub fn hpllnumerator_m(&self) -> HpllnumeratorMR {
        HpllnumeratorMR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 13:18 - H-PLL Denumerator (N)"]
    #[inline(always)]
    pub fn hplldenumerator_n(&self) -> HplldenumeratorNR {
        HplldenumeratorNR::new(((self.bits >> 13) & 0x3f) as u8)
    }
    #[doc = "Bits 19:22 - H-PLL Post Divider (P)"]
    #[inline(always)]
    pub fn hpllpost_divider_p(&self) -> HpllpostDividerPR {
        HpllpostDividerPR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 23 - Turn off H-PLL"]
    #[inline(always)]
    pub fn turn_off_hpll(&self) -> TurnOffHpllR {
        TurnOffHpllR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable H-PLL bypass mode"]
    #[inline(always)]
    pub fn enbl_hpllbypass_mode(&self) -> EnblHpllbypassModeR {
        EnblHpllbypassModeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable H-PLL reset"]
    #[inline(always)]
    pub fn enbl_hpllrst(&self) -> EnblHpllrstR {
        EnblHpllrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28 - H-PLL parameters"]
    #[inline(always)]
    pub fn hpllparameters(&self) -> HpllparametersR {
        HpllparametersR::new(((self.bits >> 26) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:12 - H-PLL Numerator (M)"]
    #[inline(always)]
    pub fn hpllnumerator_m(&mut self) -> HpllnumeratorMW<Scu200Spec> {
        HpllnumeratorMW::new(self, 0)
    }
    #[doc = "Bits 13:18 - H-PLL Denumerator (N)"]
    #[inline(always)]
    pub fn hplldenumerator_n(&mut self) -> HplldenumeratorNW<Scu200Spec> {
        HplldenumeratorNW::new(self, 13)
    }
    #[doc = "Bits 19:22 - H-PLL Post Divider (P)"]
    #[inline(always)]
    pub fn hpllpost_divider_p(&mut self) -> HpllpostDividerPW<Scu200Spec> {
        HpllpostDividerPW::new(self, 19)
    }
    #[doc = "Bit 23 - Turn off H-PLL"]
    #[inline(always)]
    pub fn turn_off_hpll(&mut self) -> TurnOffHpllW<Scu200Spec> {
        TurnOffHpllW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable H-PLL bypass mode"]
    #[inline(always)]
    pub fn enbl_hpllbypass_mode(&mut self) -> EnblHpllbypassModeW<Scu200Spec> {
        EnblHpllbypassModeW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable H-PLL reset"]
    #[inline(always)]
    pub fn enbl_hpllrst(&mut self) -> EnblHpllrstW<Scu200Spec> {
        EnblHpllrstW::new(self, 25)
    }
    #[doc = "Bits 26:28 - H-PLL parameters"]
    #[inline(always)]
    pub fn hpllparameters(&mut self) -> HpllparametersW<Scu200Spec> {
        HpllparametersW::new(self, 26)
    }
}
#[doc = "H-PLL Parameter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu200Spec;
impl crate::RegisterSpec for Scu200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu200::R`](R) reader structure"]
impl crate::Readable for Scu200Spec {}
#[doc = "`write(|w| ..)` method takes [`scu200::W`](W) writer structure"]
impl crate::Writable for Scu200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU200 to value 0x1000_4077"]
impl crate::Resettable for Scu200Spec {
    const RESET_VALUE: u32 = 0x1000_4077;
}
