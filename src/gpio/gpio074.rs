#[doc = "Register `GPIO074` reader"]
pub type R = crate::R<Gpio074Spec>;
#[doc = "Register `GPIO074` writer"]
pub type W = crate::W<Gpio074Spec>;
#[doc = "Field `PortGPIOI70DirionCtrl` reader - Port GPIOI\\[7:0\\] direction control"]
pub type PortGpioi70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOI70DirionCtrl` writer - Port GPIOI\\[7:0\\] direction control"]
pub type PortGpioi70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOJ70DirionCtrl` reader - Port GPIOJ\\[7:0\\] direction control"]
pub type PortGpioj70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOJ70DirionCtrl` writer - Port GPIOJ\\[7:0\\] direction control"]
pub type PortGpioj70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOK70DirionCtrl` reader - Port GPIOK\\[7:0\\] direction control"]
pub type PortGpiok70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOK70DirionCtrl` writer - Port GPIOK\\[7:0\\] direction control"]
pub type PortGpiok70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOL70DirionCtrl` reader - Port GPIOL\\[7:0\\] direction control"]
pub type PortGpiol70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOL70DirionCtrl` writer - Port GPIOL\\[7:0\\] direction control"]
pub type PortGpiol70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioi70dirion_ctrl(&self) -> PortGpioi70dirionCtrlR {
        PortGpioi70dirionCtrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioj70dirion_ctrl(&self) -> PortGpioj70dirionCtrlR {
        PortGpioj70dirionCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiok70dirion_ctrl(&self) -> PortGpiok70dirionCtrlR {
        PortGpiok70dirionCtrlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiol70dirion_ctrl(&self) -> PortGpiol70dirionCtrlR {
        PortGpiol70dirionCtrlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOI\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioi70dirion_ctrl(&mut self) -> PortGpioi70dirionCtrlW<Gpio074Spec> {
        PortGpioi70dirionCtrlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOJ\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioj70dirion_ctrl(&mut self) -> PortGpioj70dirionCtrlW<Gpio074Spec> {
        PortGpioj70dirionCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOK\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiok70dirion_ctrl(&mut self) -> PortGpiok70dirionCtrlW<Gpio074Spec> {
        PortGpiok70dirionCtrlW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOL\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiol70dirion_ctrl(&mut self) -> PortGpiol70dirionCtrlW<Gpio074Spec> {
        PortGpiol70dirionCtrlW::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio074Spec;
impl crate::RegisterSpec for Gpio074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio074::R`](R) reader structure"]
impl crate::Readable for Gpio074Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio074::W`](W) writer structure"]
impl crate::Writable for Gpio074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO074 to value 0"]
impl crate::Resettable for Gpio074Spec {}
