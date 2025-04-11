#[doc = "Register `GPIO550` reader"]
pub type R = crate::R<Gpio550Spec>;
#[doc = "Register `GPIO550` writer"]
pub type W = crate::W<Gpio550Spec>;
#[doc = "Field `PortSerialGPIOI70WDTRstToleranceEnbl` reader - Port Serial GPIOI\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioi70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOI70WDTRstToleranceEnbl` writer - Port Serial GPIOI\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioi70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOJ70WDTRstToleranceEnbl` reader - Port Serial GPIOJ\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioj70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOJ70WDTRstToleranceEnbl` writer - Port Serial GPIOJ\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioj70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOK70WDTRstToleranceEnbl` reader - Port Serial GPIOK\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiok70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOK70WDTRstToleranceEnbl` writer - Port Serial GPIOK\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiok70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOL70WDTRstToleranceEnbl` reader - Port Serial GPIOL\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiol70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOL70WDTRstToleranceEnbl` writer - Port Serial GPIOL\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiol70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioi70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpioi70wdtrstToleranceEnblR {
        PortSerialGpioi70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioj70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpioj70wdtrstToleranceEnblR {
        PortSerialGpioj70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiok70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiok70wdtrstToleranceEnblR {
        PortSerialGpiok70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiol70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiol70wdtrstToleranceEnblR {
        PortSerialGpiol70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOI\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioi70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpioi70wdtrstToleranceEnblW<Gpio550Spec> {
        PortSerialGpioi70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOJ\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioj70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpioj70wdtrstToleranceEnblW<Gpio550Spec> {
        PortSerialGpioj70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOK\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiok70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiok70wdtrstToleranceEnblW<Gpio550Spec> {
        PortSerialGpiok70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOL\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiol70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiol70wdtrstToleranceEnblW<Gpio550Spec> {
        PortSerialGpiol70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_I/J/K/L 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio550::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio550::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio550Spec;
impl crate::RegisterSpec for Gpio550Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio550::R`](R) reader structure"]
impl crate::Readable for Gpio550Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio550::W`](W) writer structure"]
impl crate::Writable for Gpio550Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO550 to value 0"]
impl crate::Resettable for Gpio550Spec {}
