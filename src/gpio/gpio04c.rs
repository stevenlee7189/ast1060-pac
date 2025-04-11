#[doc = "Register `GPIO04C` reader"]
pub type R = crate::R<Gpio04cSpec>;
#[doc = "Register `GPIO04C` writer"]
pub type W = crate::W<Gpio04cSpec>;
#[doc = "Field `PortGPIOE70DebounceSettingReg2` reader - Port GPIOE\\[7:0\\] debounce setting register #2"]
pub type PortGpioe70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOE70DebounceSettingReg2` writer - Port GPIOE\\[7:0\\] debounce setting register #2"]
pub type PortGpioe70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70DebounceSettingReg2` reader - Port GPIOF\\[7:0\\] debounce setting register #2"]
pub type PortGpiof70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOF70DebounceSettingReg2` writer - Port GPIOF\\[7:0\\] debounce setting register #2"]
pub type PortGpiof70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70DebounceSettingReg2` reader - Port GPIOG\\[7:0\\] debounce setting register #2"]
pub type PortGpiog70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOG70DebounceSettingReg2` writer - Port GPIOG\\[7:0\\] debounce setting register #2"]
pub type PortGpiog70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70DebounceSettingReg2` reader - Port GPIOH\\[7:0\\] debounce setting register #2"]
pub type PortGpioh70debounceSettingReg2R = crate::FieldReader;
#[doc = "Field `PortGPIOH70DebounceSettingReg2` writer - Port GPIOH\\[7:0\\] debounce setting register #2"]
pub type PortGpioh70debounceSettingReg2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioe70debounce_setting_reg2(&self) -> PortGpioe70debounceSettingReg2R {
        PortGpioe70debounceSettingReg2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiof70debounce_setting_reg2(&self) -> PortGpiof70debounceSettingReg2R {
        PortGpiof70debounceSettingReg2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiog70debounce_setting_reg2(&self) -> PortGpiog70debounceSettingReg2R {
        PortGpiog70debounceSettingReg2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioh70debounce_setting_reg2(&self) -> PortGpioh70debounceSettingReg2R {
        PortGpioh70debounceSettingReg2R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioe70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioe70debounceSettingReg2W<Gpio04cSpec> {
        PortGpioe70debounceSettingReg2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiof70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiof70debounceSettingReg2W<Gpio04cSpec> {
        PortGpiof70debounceSettingReg2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpiog70debounce_setting_reg2(
        &mut self,
    ) -> PortGpiog70debounceSettingReg2W<Gpio04cSpec> {
        PortGpiog70debounceSettingReg2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] debounce setting register #2"]
    #[inline(always)]
    pub fn port_gpioh70debounce_setting_reg2(
        &mut self,
    ) -> PortGpioh70debounceSettingReg2W<Gpio04cSpec> {
        PortGpioh70debounceSettingReg2W::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Debounce Setting Register \\#2\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio04cSpec;
impl crate::RegisterSpec for Gpio04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio04c::R`](R) reader structure"]
impl crate::Readable for Gpio04cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio04c::W`](W) writer structure"]
impl crate::Writable for Gpio04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO04C to value 0"]
impl crate::Resettable for Gpio04cSpec {}
