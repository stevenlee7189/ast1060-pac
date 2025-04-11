#[doc = "Register `GPIO534` reader"]
pub type R = crate::R<Gpio534Spec>;
#[doc = "Register `GPIO534` writer"]
pub type W = crate::W<Gpio534Spec>;
#[doc = "Field `PortSerialGPIOE70WDTRstToleranceEnbl` reader - Port Serial GPIOE\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioe70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOE70WDTRstToleranceEnbl` writer - Port Serial GPIOE\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioe70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOF70WDTRstToleranceEnbl` reader - Port Serial GPIOF\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiof70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOF70WDTRstToleranceEnbl` writer - Port Serial GPIOF\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiof70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOG70WDTRstToleranceEnbl` reader - Port Serial GPIOG\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiog70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOG70WDTRstToleranceEnbl` writer - Port Serial GPIOG\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpiog70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOH70WDTRstToleranceEnbl` reader - Port Serial GPIOH\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioh70wdtrstToleranceEnblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOH70WDTRstToleranceEnbl` writer - Port Serial GPIOH\\[7:0\\] WDT reset tolerance enable"]
pub type PortSerialGpioh70wdtrstToleranceEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioe70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpioe70wdtrstToleranceEnblR {
        PortSerialGpioe70wdtrstToleranceEnblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiof70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiof70wdtrstToleranceEnblR {
        PortSerialGpiof70wdtrstToleranceEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiog70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpiog70wdtrstToleranceEnblR {
        PortSerialGpiog70wdtrstToleranceEnblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioh70wdtrst_tolerance_enbl(
        &self,
    ) -> PortSerialGpioh70wdtrstToleranceEnblR {
        PortSerialGpioh70wdtrstToleranceEnblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioe70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpioe70wdtrstToleranceEnblW<Gpio534Spec> {
        PortSerialGpioe70wdtrstToleranceEnblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiof70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiof70wdtrstToleranceEnblW<Gpio534Spec> {
        PortSerialGpiof70wdtrstToleranceEnblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpiog70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpiog70wdtrstToleranceEnblW<Gpio534Spec> {
        PortSerialGpiog70wdtrstToleranceEnblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] WDT reset tolerance enable"]
    #[inline(always)]
    pub fn port_serial_gpioh70wdtrst_tolerance_enbl(
        &mut self,
    ) -> PortSerialGpioh70wdtrstToleranceEnblW<Gpio534Spec> {
        PortSerialGpioh70wdtrstToleranceEnblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H 1 Reset Tolerant Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio534::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio534::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio534Spec;
impl crate::RegisterSpec for Gpio534Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio534::R`](R) reader structure"]
impl crate::Readable for Gpio534Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio534::W`](W) writer structure"]
impl crate::Writable for Gpio534Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO534 to value 0"]
impl crate::Resettable for Gpio534Spec {}
