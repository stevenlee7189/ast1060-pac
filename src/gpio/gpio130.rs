#[doc = "Register `GPIO130` reader"]
pub type R = crate::R<Gpio130Spec>;
#[doc = "Register `GPIO130` writer"]
pub type W = crate::W<Gpio130Spec>;
#[doc = "Field `PortGPIOQ70DebounceSettingReg1` reader - Port GPIOQ\\[7:0\\] debounce setting register #1"]
pub type PortGpioq70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOQ70DebounceSettingReg1` writer - Port GPIOQ\\[7:0\\] debounce setting register #1"]
pub type PortGpioq70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70DebounceSettingReg1` reader - Port GPIOR\\[7:0\\] debounce setting register #1"]
pub type PortGpior70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOR70DebounceSettingReg1` writer - Port GPIOR\\[7:0\\] debounce setting register #1"]
pub type PortGpior70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70DebounceSettingReg1` reader - Port GPIOS\\[7:0\\] debounce setting register #1"]
pub type PortGpios70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIOS70DebounceSettingReg1` writer - Port GPIOS\\[7:0\\] debounce setting register #1"]
pub type PortGpios70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70DebounceSettingReg1` reader - Port GPIT\\[7:0\\] debounce setting register #1"]
pub type PortGpit70debounceSettingReg1R = crate::FieldReader;
#[doc = "Field `PortGPIT70DebounceSettingReg1` writer - Port GPIT\\[7:0\\] debounce setting register #1"]
pub type PortGpit70debounceSettingReg1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioq70debounce_setting_reg1(&self) -> PortGpioq70debounceSettingReg1R {
        PortGpioq70debounceSettingReg1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpior70debounce_setting_reg1(&self) -> PortGpior70debounceSettingReg1R {
        PortGpior70debounceSettingReg1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpios70debounce_setting_reg1(&self) -> PortGpios70debounceSettingReg1R {
        PortGpios70debounceSettingReg1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpit70debounce_setting_reg1(&self) -> PortGpit70debounceSettingReg1R {
        PortGpit70debounceSettingReg1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpioq70debounce_setting_reg1(
        &mut self,
    ) -> PortGpioq70debounceSettingReg1W<Gpio130Spec> {
        PortGpioq70debounceSettingReg1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpior70debounce_setting_reg1(
        &mut self,
    ) -> PortGpior70debounceSettingReg1W<Gpio130Spec> {
        PortGpior70debounceSettingReg1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpios70debounce_setting_reg1(
        &mut self,
    ) -> PortGpios70debounceSettingReg1W<Gpio130Spec> {
        PortGpios70debounceSettingReg1W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] debounce setting register #1"]
    #[inline(always)]
    pub fn port_gpit70debounce_setting_reg1(
        &mut self,
    ) -> PortGpit70debounceSettingReg1W<Gpio130Spec> {
        PortGpit70debounceSettingReg1W::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Debounce Setting Register \\#1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio130::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio130::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio130Spec;
impl crate::RegisterSpec for Gpio130Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio130::R`](R) reader structure"]
impl crate::Readable for Gpio130Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio130::W`](W) writer structure"]
impl crate::Writable for Gpio130Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO130 to value 0"]
impl crate::Resettable for Gpio130Spec {}
