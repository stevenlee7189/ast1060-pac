#[doc = "Register `GPIO51C` reader"]
pub type R = crate::R<Gpio51cSpec>;
#[doc = "Register `GPIO51C` writer"]
pub type W = crate::W<Gpio51cSpec>;
#[doc = "Field `PortSerialGPIOE70DataReg` reader - Port Serial GPIOE\\[7:0\\] data register"]
pub type PortSerialGpioe70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOE70DataReg` writer - Port Serial GPIOE\\[7:0\\] data register"]
pub type PortSerialGpioe70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOF70DataReg` reader - Port Serial GPIOF\\[7:0\\] data register"]
pub type PortSerialGpiof70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOF70DataReg` writer - Port Serial GPIOF\\[7:0\\] data register"]
pub type PortSerialGpiof70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOG70DataReg` reader - Port Serial GPIOG\\[7:0\\] data register"]
pub type PortSerialGpiog70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOG70DataReg` writer - Port Serial GPIOG\\[7:0\\] data register"]
pub type PortSerialGpiog70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOH70DataReg` reader - Port Serial GPIOH\\[7:0\\] data register"]
pub type PortSerialGpioh70dataRegR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOH70DataReg` writer - Port Serial GPIOH\\[7:0\\] data register"]
pub type PortSerialGpioh70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioe70data_reg(&self) -> PortSerialGpioe70dataRegR {
        PortSerialGpioe70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiof70data_reg(&self) -> PortSerialGpiof70dataRegR {
        PortSerialGpiof70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiog70data_reg(&self) -> PortSerialGpiog70dataRegR {
        PortSerialGpiog70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioh70data_reg(&self) -> PortSerialGpioh70dataRegR {
        PortSerialGpioh70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOE\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioe70data_reg(&mut self) -> PortSerialGpioe70dataRegW<Gpio51cSpec> {
        PortSerialGpioe70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPIOF\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiof70data_reg(&mut self) -> PortSerialGpiof70dataRegW<Gpio51cSpec> {
        PortSerialGpiof70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOG\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpiog70data_reg(&mut self) -> PortSerialGpiog70dataRegW<Gpio51cSpec> {
        PortSerialGpiog70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOH\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_serial_gpioh70data_reg(&mut self) -> PortSerialGpioh70dataRegW<Gpio51cSpec> {
        PortSerialGpioh70dataRegW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_E/F/G/H 1 Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio51c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio51c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio51cSpec;
impl crate::RegisterSpec for Gpio51cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio51c::R`](R) reader structure"]
impl crate::Readable for Gpio51cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio51c::W`](W) writer structure"]
impl crate::Writable for Gpio51cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO51C to value 0"]
impl crate::Resettable for Gpio51cSpec {}
