#[doc = "Register `GPIO104` reader"]
pub type R = crate::R<Gpio104Spec>;
#[doc = "Register `GPIO104` writer"]
pub type W = crate::W<Gpio104Spec>;
#[doc = "Field `PortGPIOM70DebounceSettingReg2` reader - Port GPIOM\\[7:0\\] debounce setting register #2"]
pub type PortGpiom70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOM70DebounceSettingReg2` writer - Port GPIOM\\[7:0\\] debounce setting register #2"]
pub type PortGpiom70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70DebounceSettingReg2` reader - Port GPION\\[7:0\\] debounce setting register #2"]
pub type PortGpion70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPION70DebounceSettingReg2` writer - Port GPION\\[7:0\\] debounce setting register #2"]
pub type PortGpion70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70DebounceSettingReg2` reader - Port GPIOO\\[7:0\\] debounce setting register #2"]
pub type PortGpioo70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOO70DebounceSettingReg2` writer - Port GPIOO\\[7:0\\] debounce setting register #2"]
pub type PortGpioo70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70DebounceSettingReg2` reader - Port GPIOP\\[7:0\\] debounce setting register #2"]
pub type PortGpiop70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOP70DebounceSettingReg2` writer - Port GPIOP\\[7:0\\] debounce setting register #2"]
pub type PortGpiop70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiom70debounce_setting_reg2(&self) -> PortGpiom70debounceSettingReg2R {
        PortGpiom70debounceSettingReg2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpion70debounce_setting_reg2(&self) -> PortGpion70debounceSettingReg2R {
        PortGpion70debounceSettingReg2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioo70debounce_setting_reg2(&self) -> PortGpioo70debounceSettingReg2R {
        PortGpioo70debounceSettingReg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiop70debounce_setting_reg2(&self) -> PortGpiop70debounceSettingReg2R {
        PortGpiop70debounceSettingReg2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiom70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiom70debounceSettingReg2W<Gpio104Spec> {
        PortGpiom70debounceSettingReg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpion70debounce_setting_reg2(
        &mut self,
    ) -> PortGpion70debounceSettingReg2W<Gpio104Spec> {
        PortGpion70debounceSettingReg2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioo70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioo70debounceSettingReg2W<Gpio104Spec> {
        PortGpioo70debounceSettingReg2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiop70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiop70debounceSettingReg2W<Gpio104Spec> {
        PortGpiop70debounceSettingReg2W::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio104Spec;
impl crate::RegisterSpec for Gpio104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio104::R`](R) reader structure"]
impl crate::Readable for Gpio104Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio104::W`](W) writer structure"]
impl crate::Writable for Gpio104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO104 to value 0"]
impl crate::Resettable for Gpio104Spec {}
