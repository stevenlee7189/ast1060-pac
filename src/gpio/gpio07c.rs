#[doc = "Register `GPIO07C` reader"]
pub type R = crate::R<Gpio07cSpec>;
#[doc = "Register `GPIO07C` writer"]
pub type W = crate::W<Gpio07cSpec>;
#[doc = "Field `PortGPIOM70DirionCtrl` reader - Port GPIOM\\[7:0\\] direction control"]
pub type PortGpiom70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOM70DirionCtrl` writer - Port GPIOM\\[7:0\\] direction control"]
pub type PortGpiom70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70DirionCtrl` reader - Port GPION\\[7:0\\] direction control"]
pub type PortGpion70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPION70DirionCtrl` writer - Port GPION\\[7:0\\] direction control"]
pub type PortGpion70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70DirionCtrl` reader - Port GPIOO\\[7:0\\] direction control"]
pub type PortGpioo70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOO70DirionCtrl` writer - Port GPIOO\\[7:0\\] direction control"]
pub type PortGpioo70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70DirionCtrl` reader - Port GPIOP\\[7:0\\] direction control"]
pub type PortGpiop70dirionCtrlR = crate::FieldReader;
#[doc = "Field `PortGPIOP70DirionCtrl` writer - Port GPIOP\\[7:0\\] direction control"]
pub type PortGpiop70dirionCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiom70dirion_ctrl(&self) -> PortGpiom70dirionCtrlR {
        PortGpiom70dirionCtrlR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpion70dirion_ctrl(&self) -> PortGpion70dirionCtrlR {
        PortGpion70dirionCtrlR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioo70dirion_ctrl(&self) -> PortGpioo70dirionCtrlR {
        PortGpioo70dirionCtrlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiop70dirion_ctrl(&self) -> PortGpiop70dirionCtrlR {
        PortGpiop70dirionCtrlR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiom70dirion_ctrl(&mut self) -> PortGpiom70dirionCtrlW<Gpio07cSpec> {
        PortGpiom70dirionCtrlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpion70dirion_ctrl(&mut self) -> PortGpion70dirionCtrlW<Gpio07cSpec> {
        PortGpion70dirionCtrlW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpioo70dirion_ctrl(&mut self) -> PortGpioo70dirionCtrlW<Gpio07cSpec> {
        PortGpioo70dirionCtrlW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] direction control"]
    #[inline(always)]
    pub fn port_gpiop70dirion_ctrl(&mut self) -> PortGpiop70dirionCtrlW<Gpio07cSpec> {
        PortGpiop70dirionCtrlW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Direction Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio07cSpec;
impl crate::RegisterSpec for Gpio07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio07c::R`](R) reader structure"]
impl crate::Readable for Gpio07cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio07c::W`](W) writer structure"]
impl crate::Writable for Gpio07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO07C to value 0"]
impl crate::Resettable for Gpio07cSpec {}
