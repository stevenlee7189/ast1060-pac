#[doc = "Register `I2CG10` reader"]
pub type R = crate::R<I2cg10Spec>;
#[doc = "Register `I2CG10` writer"]
pub type W = crate::W<I2cg10Spec>;
#[doc = "Field `BaseClk1DivisorBasedivider1` reader - Base clock 1 divisor, base_divider_1"]
pub type BaseClk1divisorBasedivider1R = crate::FieldReader;
#[doc = "Field `BaseClk1DivisorBasedivider1` writer - Base clock 1 divisor, base_divider_1"]
pub type BaseClk1divisorBasedivider1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BaseClk2DivisorBasedivider2` reader - Base clock 2 divisor, base_divider_2"]
pub type BaseClk2divisorBasedivider2R = crate::FieldReader;
#[doc = "Field `BaseClk2DivisorBasedivider2` writer - Base clock 2 divisor, base_divider_2"]
pub type BaseClk2divisorBasedivider2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BaseClk3DivisorBasedivider3` reader - Base clock 3 divisor, base_divider_3"]
pub type BaseClk3divisorBasedivider3R = crate::FieldReader;
#[doc = "Field `BaseClk3DivisorBasedivider3` writer - Base clock 3 divisor, base_divider_3"]
pub type BaseClk3divisorBasedivider3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BaseClk4DivisorBasedivider4` reader - Base clock 4 divisor, base_divider_4"]
pub type BaseClk4divisorBasedivider4R = crate::FieldReader;
#[doc = "Field `BaseClk4DivisorBasedivider4` writer - Base clock 4 divisor, base_divider_4"]
pub type BaseClk4divisorBasedivider4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Base clock 1 divisor, base_divider_1"]
    #[inline(always)]
    pub fn base_clk1divisor_basedivider1(&self) -> BaseClk1divisorBasedivider1R {
        BaseClk1divisorBasedivider1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Base clock 2 divisor, base_divider_2"]
    #[inline(always)]
    pub fn base_clk2divisor_basedivider2(&self) -> BaseClk2divisorBasedivider2R {
        BaseClk2divisorBasedivider2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Base clock 3 divisor, base_divider_3"]
    #[inline(always)]
    pub fn base_clk3divisor_basedivider3(&self) -> BaseClk3divisorBasedivider3R {
        BaseClk3divisorBasedivider3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Base clock 4 divisor, base_divider_4"]
    #[inline(always)]
    pub fn base_clk4divisor_basedivider4(&self) -> BaseClk4divisorBasedivider4R {
        BaseClk4divisorBasedivider4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Base clock 1 divisor, base_divider_1"]
    #[inline(always)]
    pub fn base_clk1divisor_basedivider1(&mut self) -> BaseClk1divisorBasedivider1W<I2cg10Spec> {
        BaseClk1divisorBasedivider1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Base clock 2 divisor, base_divider_2"]
    #[inline(always)]
    pub fn base_clk2divisor_basedivider2(&mut self) -> BaseClk2divisorBasedivider2W<I2cg10Spec> {
        BaseClk2divisorBasedivider2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Base clock 3 divisor, base_divider_3"]
    #[inline(always)]
    pub fn base_clk3divisor_basedivider3(&mut self) -> BaseClk3divisorBasedivider3W<I2cg10Spec> {
        BaseClk3divisorBasedivider3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Base clock 4 divisor, base_divider_4"]
    #[inline(always)]
    pub fn base_clk4divisor_basedivider4(&mut self) -> BaseClk4divisorBasedivider4W<I2cg10Spec> {
        BaseClk4divisorBasedivider4W::new(self, 24)
    }
}
#[doc = "New Clock Divider Control Register (I2CG0C\\[1\\] = 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cg10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cg10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cg10Spec;
impl crate::RegisterSpec for I2cg10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cg10::R`](R) reader structure"]
impl crate::Readable for I2cg10Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cg10::W`](W) writer structure"]
impl crate::Writable for I2cg10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CG10 to value 0"]
impl crate::Resettable for I2cg10Spec {}
