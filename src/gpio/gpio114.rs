#[doc = "Register `GPIO114` reader"]
pub type R = crate::R<Gpio114Spec>;
#[doc = "Register `GPIO114` writer"]
pub type W = crate::W<Gpio114Spec>;
#[doc = "Field `PortGPIOQ70CmdSource1` reader - Port GPIOQ\\[7:0\\] Command Source 1"]
pub type PortGpioq70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOQ70CmdSource1` writer - Port GPIOQ\\[7:0\\] Command Source 1"]
pub type PortGpioq70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOR70CmdSource1` reader - Port GPIOR\\[7:0\\] Command Source 1"]
pub type PortGpior70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOR70CmdSource1` writer - Port GPIOR\\[7:0\\] Command Source 1"]
pub type PortGpior70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOS70CmdSource1` reader - Port GPIOS\\[7:0\\] Command Source 1"]
pub type PortGpios70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIOS70CmdSource1` writer - Port GPIOS\\[7:0\\] Command Source 1"]
pub type PortGpios70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIT70CmdSource1` reader - Port GPIT\\[7:0\\] Command Source 1"]
pub type PortGpit70cmdSource1R = crate::BitReader;
#[doc = "Field `PortGPIT70CmdSource1` writer - Port GPIT\\[7:0\\] Command Source 1"]
pub type PortGpit70cmdSource1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOQ\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioq70cmd_source1(&self) -> PortGpioq70cmdSource1R {
        PortGpioq70cmdSource1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOR\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpior70cmd_source1(&self) -> PortGpior70cmdSource1R {
        PortGpior70cmdSource1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOS\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpios70cmd_source1(&self) -> PortGpios70cmdSource1R {
        PortGpios70cmdSource1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIT\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpit70cmd_source1(&self) -> PortGpit70cmdSource1R {
        PortGpit70cmdSource1R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOQ\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpioq70cmd_source1(&mut self) -> PortGpioq70cmdSource1W<Gpio114Spec> {
        PortGpioq70cmdSource1W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOR\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpior70cmd_source1(&mut self) -> PortGpior70cmdSource1W<Gpio114Spec> {
        PortGpior70cmdSource1W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOS\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpios70cmd_source1(&mut self) -> PortGpios70cmdSource1W<Gpio114Spec> {
        PortGpios70cmdSource1W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIT\\[7:0\\] Command Source 1"]
    #[inline(always)]
    pub fn port_gpit70cmd_source1(&mut self) -> PortGpit70cmdSource1W<Gpio114Spec> {
        PortGpit70cmdSource1W::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Command Source 1\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio114Spec;
impl crate::RegisterSpec for Gpio114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio114::R`](R) reader structure"]
impl crate::Readable for Gpio114Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio114::W`](W) writer structure"]
impl crate::Writable for Gpio114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO114 to value 0"]
impl crate::Resettable for Gpio114Spec {}
