#[doc = "Register `GPIO044` reader"]
pub type R = crate::R<Gpio044Spec>;
#[doc = "Register `GPIO044` writer"]
pub type W = crate::W<Gpio044Spec>;
#[doc = "Field `PortGPIOA70DebounceSettingReg2` reader - Port GPIOA\\[7:0\\] debounce setting register #2"]
pub type PortGpioa70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOA70DebounceSettingReg2` writer - Port GPIOA\\[7:0\\] debounce setting register #2"]
pub type PortGpioa70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70DebounceSettingReg2` reader - Port GPIOB\\[7:0\\] debounce setting register #2"]
pub type PortGpiob70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOB70DebounceSettingReg2` writer - Port GPIOB\\[7:0\\] debounce setting register #2"]
pub type PortGpiob70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70DebounceSettingReg2` reader - Port GPIOC\\[7:0\\] debounce setting register #2"]
pub type PortGpioc70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOC70DebounceSettingReg2` writer - Port GPIOC\\[7:0\\] debounce setting register #2"]
pub type PortGpioc70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70DebounceSettingReg2` reader - Port GPIOD\\[7:0\\] debounce setting register #2"]
pub type PortGpiod70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOD70DebounceSettingReg2` writer - Port GPIOD\\[7:0\\] debounce setting register #2"]
pub type PortGpiod70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioa70debounce_setting_reg2(&self) -> PortGpioa70debounceSettingReg2R {
        PortGpioa70debounceSettingReg2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiob70debounce_setting_reg2(&self) -> PortGpiob70debounceSettingReg2R {
        PortGpiob70debounceSettingReg2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioc70debounce_setting_reg2(&self) -> PortGpioc70debounceSettingReg2R {
        PortGpioc70debounceSettingReg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiod70debounce_setting_reg2(&self) -> PortGpiod70debounceSettingReg2R {
        PortGpiod70debounceSettingReg2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioa70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioa70debounceSettingReg2W<Gpio044Spec> {
        PortGpioa70debounceSettingReg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiob70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiob70debounceSettingReg2W<Gpio044Spec> {
        PortGpiob70debounceSettingReg2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioc70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioc70debounceSettingReg2W<Gpio044Spec> {
        PortGpioc70debounceSettingReg2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiod70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiod70debounceSettingReg2W<Gpio044Spec> {
        PortGpiod70debounceSettingReg2W::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio044Spec;
impl crate::RegisterSpec for Gpio044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio044::R`](R) reader structure"]
impl crate::Readable for Gpio044Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio044::W`](W) writer structure"]
impl crate::Writable for Gpio044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO044 to value 0"]
impl crate::Resettable for Gpio044Spec {}
