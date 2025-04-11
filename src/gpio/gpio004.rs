#[doc = "Register `GPIO004` reader"]
pub type R = crate::R<Gpio004Spec>;
#[doc = "Register `GPIO004` writer"]
pub type W = crate::W<Gpio004Spec>;
#[doc = "Field `PortGPIOA70DirionCtrl` reader - Port GPIOA\\[7:0\\] direction control"]
pub type PortGpioa70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOA70DirionCtrl` writer - Port GPIOA\\[7:0\\] direction control"]
pub type PortGpioa70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOB70DirionCtrl` reader - Port GPIOB\\[7:0\\] direction control"]
pub type PortGpiob70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOB70DirionCtrl` writer - Port GPIOB\\[7:0\\] direction control"]
pub type PortGpiob70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOC70DirionCtrl` reader - Port GPIOC\\[7:0\\] direction control"]
pub type PortGpioc70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOC70DirionCtrl` writer - Port GPIOC\\[7:0\\] direction control"]
pub type PortGpioc70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOD70DirionCtrl` reader - Port GPIOD\\[7:0\\] direction control"]
pub type PortGpiod70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOD70DirionCtrl` writer - Port GPIOD\\[7:0\\] direction control"]
pub type PortGpiod70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioa70dirion_ctrl(&self) -> PortGpioa70dirionCtrlR {
        PortGpioa70dirionCtrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiob70dirion_ctrl(&self) -> PortGpiob70dirionCtrlR {
        PortGpiob70dirionCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioc70dirion_ctrl(&self) -> PortGpioc70dirionCtrlR {
        PortGpioc70dirionCtrlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiod70dirion_ctrl(&self) -> PortGpiod70dirionCtrlR {
        PortGpiod70dirionCtrlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOA\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioa70dirion_ctrl(&mut self) -> PortGpioa70dirionCtrlW<Gpio004Spec> {
        PortGpioa70dirionCtrlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOB\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiob70dirion_ctrl(&mut self) -> PortGpiob70dirionCtrlW<Gpio004Spec> {
        PortGpiob70dirionCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOC\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioc70dirion_ctrl(&mut self) -> PortGpioc70dirionCtrlW<Gpio004Spec> {
        PortGpioc70dirionCtrlW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOD\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiod70dirion_ctrl(&mut self) -> PortGpiod70dirionCtrlW<Gpio004Spec> {
        PortGpiod70dirionCtrlW::new(self, 24)
    }
}
#[doc = "GPIO\\_A/B/C/D Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio004Spec;
impl crate::RegisterSpec for Gpio004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio004::R`](R) reader structure"]
impl crate::Readable for Gpio004Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio004::W`](W) writer structure"]
impl crate::Writable for Gpio004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO004 to value 0"]
impl crate::Resettable for Gpio004Spec {}
