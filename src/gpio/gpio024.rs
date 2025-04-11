#[doc = "Register `GPIO024` reader"]
pub type R = crate::R<Gpio024Spec>;
#[doc = "Register `GPIO024` writer"]
pub type W = crate::W<Gpio024Spec>;
#[doc = "Field `PortGPIOE70DirionCtrl` reader - Port GPIOE\\[7:0\\] direction control"]
pub type PortGpioe70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOE70DirionCtrl` writer - Port GPIOE\\[7:0\\] direction control"]
pub type PortGpioe70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOF70DirionCtrl` reader - Port GPIOF\\[7:0\\] direction control"]
pub type PortGpiof70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOF70DirionCtrl` writer - Port GPIOF\\[7:0\\] direction control"]
pub type PortGpiof70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOG70DirionCtrl` reader - Port GPIOG\\[7:0\\] direction control"]
pub type PortGpiog70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOG70DirionCtrl` writer - Port GPIOG\\[7:0\\] direction control"]
pub type PortGpiog70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOH70DirionCtrl` reader - Port GPIOH\\[7:0\\] direction control"]
pub type PortGpioh70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOH70DirionCtrl` writer - Port GPIOH\\[7:0\\] direction control"]
pub type PortGpioh70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioe70dirion_ctrl(&self) -> PortGpioe70dirionCtrlR {
        PortGpioe70dirionCtrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiof70dirion_ctrl(&self) -> PortGpiof70dirionCtrlR {
        PortGpiof70dirionCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiog70dirion_ctrl(&self) -> PortGpiog70dirionCtrlR {
        PortGpiog70dirionCtrlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioh70dirion_ctrl(&self) -> PortGpioh70dirionCtrlR {
        PortGpioh70dirionCtrlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOE\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioe70dirion_ctrl(&mut self) -> PortGpioe70dirionCtrlW<Gpio024Spec> {
        PortGpioe70dirionCtrlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOF\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiof70dirion_ctrl(&mut self) -> PortGpiof70dirionCtrlW<Gpio024Spec> {
        PortGpiof70dirionCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOG\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiog70dirion_ctrl(&mut self) -> PortGpiog70dirionCtrlW<Gpio024Spec> {
        PortGpiog70dirionCtrlW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOH\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioh70dirion_ctrl(&mut self) -> PortGpioh70dirionCtrlW<Gpio024Spec> {
        PortGpioh70dirionCtrlW::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio024::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio024::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio024Spec;
impl crate::RegisterSpec for Gpio024Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio024::R`](R) reader structure"]
impl crate::Readable for Gpio024Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio024::W`](W) writer structure"]
impl crate::Writable for Gpio024Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO024 to value 0"]
impl crate::Resettable for Gpio024Spec {}
