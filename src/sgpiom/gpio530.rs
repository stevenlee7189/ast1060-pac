#[doc = "Register `GPIO530` reader"]
pub type R = crate::R<Gpio530Spec>;
#[doc = "Register `GPIO530` writer"]
pub type W = crate::W<Gpio530Spec>;
#[doc = "Field `PortSerialGPIOE70INTStsReg` reader - Port Serial GPIOE\\[7:0\\] interrupt status register"]
pub type PortSerialGpioe70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOE70INTStsReg` writer - Port Serial GPIOE\\[7:0\\] interrupt status register"]
pub type PortSerialGpioe70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOF70INTStsReg` reader - Port Serial GPIOF\\[7:0\\] interrupt status register"]
pub type PortSerialGpiof70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOF70INTStsReg` writer - Port Serial GPIOF\\[7:0\\] interrupt status register"]
pub type PortSerialGpiof70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOG70INTStsReg` reader - Port Serial GPIOG\\[7:0\\] interrupt status register"]
pub type PortSerialGpiog70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOG70INTStsReg` writer - Port Serial GPIOG\\[7:0\\] interrupt status register"]
pub type PortSerialGpiog70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOH70INTStsReg` reader - Port Serial GPIOH\\[7:0\\] interrupt status register"]
pub type PortSerialGpioh70intstsRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOH70INTStsReg` writer - Port Serial GPIOH\\[7:0\\] interrupt status register"]
pub type PortSerialGpioh70intstsRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsts_reg(&self) -> PortSerialGpioe70intstsRegR {
        PortSerialGpioe70intstsRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsts_reg(&self) -> PortSerialGpiof70intstsRegR {
        PortSerialGpiof70intstsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsts_reg(&self) -> PortSerialGpiog70intstsRegR {
        PortSerialGpiog70intstsRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsts_reg(&self) -> PortSerialGpioh70intstsRegR {
        PortSerialGpioh70intstsRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioe70intsts_reg(&mut self) -> PortSerialGpioe70intstsRegW<Gpio530Spec> {
        PortSerialGpioe70intstsRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiof70intsts_reg(&mut self) -> PortSerialGpiof70intstsRegW<Gpio530Spec> {
        PortSerialGpiof70intstsRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpiog70intsts_reg(&mut self) -> PortSerialGpiog70intstsRegW<Gpio530Spec> {
        PortSerialGpiog70intstsRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] interrupt status register"]
    #[inline(always)]
    pub fn port_serial_gpioh70intsts_reg(&mut self) -> PortSerialGpioh70intstsRegW<Gpio530Spec> {
        PortSerialGpioh70intstsRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H 1 Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio530::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio530::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio530Spec;
impl crate::RegisterSpec for Gpio530Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio530::R`](R) reader structure"]
impl crate::Readable for Gpio530Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio530::W`](W) writer structure"]
impl crate::Writable for Gpio530Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO530 to value 0"]
impl crate::Resettable for Gpio530Spec {}
