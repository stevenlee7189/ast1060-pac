#[doc = "Register `GPIO078` reader"]
pub type R = crate::R<Gpio078Spec>;
#[doc = "Register `GPIO078` writer"]
pub type W = crate::W<Gpio078Spec>;
#[doc = "Field `PortGPIOM70DataReg` reader - Port GPIOM\\[7:0\\] data register"]
pub type PortGpiom70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOM70DataReg` writer - Port GPIOM\\[7:0\\] data register"]
pub type PortGpiom70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPION70DataReg` reader - Port GPION\\[7:0\\] data register"]
pub type PortGpion70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPION70DataReg` writer - Port GPION\\[7:0\\] data register"]
pub type PortGpion70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOO70DataReg` reader - Port GPIOO\\[7:0\\] data register"]
pub type PortGpioo70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOO70DataReg` writer - Port GPIOO\\[7:0\\] data register"]
pub type PortGpioo70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortGPIOP70DataReg` reader - Port GPIOP\\[7:0\\] data register"]
pub type PortGpiop70dataRegR = crate::FieldReader;
#[doc = "Field `PortGPIOP70DataReg` writer - Port GPIOP\\[7:0\\] data register"]
pub type PortGpiop70dataRegW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiom70data_reg(&self) -> PortGpiom70dataRegR {
        PortGpiom70dataRegR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpion70data_reg(&self) -> PortGpion70dataRegR {
        PortGpion70dataRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioo70data_reg(&self) -> PortGpioo70dataRegR {
        PortGpioo70dataRegR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiop70data_reg(&self) -> PortGpiop70dataRegR {
        PortGpiop70dataRegR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port GPIOM\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiom70data_reg(&mut self) -> PortGpiom70dataRegW<Gpio078Spec> {
        PortGpiom70dataRegW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port GPION\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpion70data_reg(&mut self) -> PortGpion70dataRegW<Gpio078Spec> {
        PortGpion70dataRegW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port GPIOO\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpioo70data_reg(&mut self) -> PortGpioo70dataRegW<Gpio078Spec> {
        PortGpioo70dataRegW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port GPIOP\\[7:0\\] data register"]
    #[inline(always)]
    pub fn port_gpiop70data_reg(&mut self) -> PortGpiop70dataRegW<Gpio078Spec> {
        PortGpiop70dataRegW::new(self, 24)
    }
}
#[doc = "GPIO\\_M/N/O/P Data Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio078::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio078::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio078Spec;
impl crate::RegisterSpec for Gpio078Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio078::R`](R) reader structure"]
impl crate::Readable for Gpio078Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio078::W`](W) writer structure"]
impl crate::Writable for Gpio078Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO078 to value 0"]
impl crate::Resettable for Gpio078Spec {}
