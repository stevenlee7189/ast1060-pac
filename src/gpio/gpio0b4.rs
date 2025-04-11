#[doc = "Register `GPIO0B4` reader"]
pub type R = crate::R<Gpio0b4Spec>;
#[doc = "Register `GPIO0B4` writer"]
pub type W = crate::W<Gpio0b4Spec>;
#[doc = "Field `PortGPIOI70DebounceSettingReg2` reader - Port GPIOI\\[7:0\\] debounce setting register #2"]
pub type PortGpioi70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOI70DebounceSettingReg2` writer - Port GPIOI\\[7:0\\] debounce setting register #2"]
pub type PortGpioi70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70DebounceSettingReg2` reader - Port GPIOJ\\[7:0\\] debounce setting register #2"]
pub type PortGpioj70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOJ70DebounceSettingReg2` writer - Port GPIOJ\\[7:0\\] debounce setting register #2"]
pub type PortGpioj70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70DebounceSettingReg2` reader - Port GPIOK\\[7:0\\] debounce setting register #2"]
pub type PortGpiok70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOK70DebounceSettingReg2` writer - Port GPIOK\\[7:0\\] debounce setting register #2"]
pub type PortGpiok70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70DebounceSettingReg2` reader - Port GPIOL\\[7:0\\] debounce setting register #2"]
pub type PortGpiol70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOL70DebounceSettingReg2` writer - Port GPIOL\\[7:0\\] debounce setting register #2"]
pub type PortGpiol70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioi70debounce_setting_reg2(&self) -> PortGpioi70debounceSettingReg2R {
        PortGpioi70debounceSettingReg2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioj70debounce_setting_reg2(&self) -> PortGpioj70debounceSettingReg2R {
        PortGpioj70debounceSettingReg2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiok70debounce_setting_reg2(&self) -> PortGpiok70debounceSettingReg2R {
        PortGpiok70debounceSettingReg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiol70debounce_setting_reg2(&self) -> PortGpiol70debounceSettingReg2R {
        PortGpiol70debounceSettingReg2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioi70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioi70debounceSettingReg2W<Gpio0b4Spec> {
        PortGpioi70debounceSettingReg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioj70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioj70debounceSettingReg2W<Gpio0b4Spec> {
        PortGpioj70debounceSettingReg2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiok70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiok70debounceSettingReg2W<Gpio0b4Spec> {
        PortGpiok70debounceSettingReg2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiol70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiol70debounceSettingReg2W<Gpio0b4Spec> {
        PortGpiol70debounceSettingReg2W::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio0b4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio0b4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio0b4Spec;
impl crate::RegisterSpec for Gpio0b4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio0b4::R`](R) reader structure"]
impl crate::Readable for Gpio0b4Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio0b4::W`](W) writer structure"]
impl crate::Writable for Gpio0b4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO0B4 to value 0"]
impl crate::Resettable for Gpio0b4Spec {}
