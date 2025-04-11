#[doc = "Register `GPIO118` reader"]
pub type R = crate::R<Gpio118Spec>;
#[doc = "Register `GPIO118` writer"]
pub type W = crate::W<Gpio118Spec>;
#[doc = "Field `PortGPIOQ70INTEnbl` reader - Port GPIOQ\\[7:0\\] interrupt enable"]
pub type PortGpioq70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOQ70INTEnbl` writer - Port GPIOQ\\[7:0\\] interrupt enable"]
pub type PortGpioq70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOR70INTEnbl` reader - Port GPIOR\\[7:0\\] interrupt enable"]
pub type PortGpior70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOR70INTEnbl` writer - Port GPIOR\\[7:0\\] interrupt enable"]
pub type PortGpior70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOS70INTEnbl` reader - Port GPIOS\\[7:0\\] interrupt enable"]
pub type PortGpios70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIOS70INTEnbl` writer - Port GPIOS\\[7:0\\] interrupt enable"]
pub type PortGpios70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIT70INTEnbl` reader - Port GPIT\\[7:0\\] interrupt enable"]
pub type PortGpit70intenblR = crate::FieldReader;
#[doc = "Field `PortGPIT70INTEnbl` writer - Port GPIT\\[7:0\\] interrupt enable"]
pub type PortGpit70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioq70intenbl(&self) -> PortGpioq70intenblR {
        PortGpioq70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpior70intenbl(&self) -> PortGpior70intenblR {
        PortGpior70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpios70intenbl(&self) -> PortGpios70intenblR {
        PortGpios70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpit70intenbl(&self) -> PortGpit70intenblR {
        PortGpit70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOQ\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpioq70intenbl(&mut self) -> PortGpioq70intenblW<Gpio118Spec> {
        PortGpioq70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPIOR\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpior70intenbl(&mut self) -> PortGpior70intenblW<Gpio118Spec> {
        PortGpior70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOS\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpios70intenbl(&mut self) -> PortGpios70intenblW<Gpio118Spec> {
        PortGpios70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIT\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_gpit70intenbl(&mut self) -> PortGpit70intenblW<Gpio118Spec> {
        PortGpit70intenblW::new(self, 24)
    }
}
#[doc = "GPIO\\_Q/R/S/T Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio118::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio118::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio118Spec;
impl crate::RegisterSpec for Gpio118Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio118::R`](R) reader structure"]
impl crate::Readable for Gpio118Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio118::W`](W) writer structure"]
impl crate::Writable for Gpio118Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO118 to value 0"]
impl crate::Resettable for Gpio118Spec {}
