#[doc = "Register `GPIO594` reader"]
pub type R = crate::R<Gpio594Spec>;
#[doc = "Register `GPIO594` writer"]
pub type W = crate::W<Gpio594Spec>;
#[doc = "Field `PortSerialGPIOM70INTEnbl` reader - Port Serial GPIOM\\[7:0\\] interrupt enable"]
pub type PortSerialGpiom70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOM70INTEnbl` writer - Port Serial GPIOM\\[7:0\\] interrupt enable"]
pub type PortSerialGpiom70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPION70INTEnbl` reader - Port Serial GPION\\[7:0\\] interrupt enable"]
pub type PortSerialGpion70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPION70INTEnbl` writer - Port Serial GPION\\[7:0\\] interrupt enable"]
pub type PortSerialGpion70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOO70INTEnbl` reader - Port Serial GPIOO\\[7:0\\] interrupt enable"]
pub type PortSerialGpioo70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOO70INTEnbl` writer - Port Serial GPIOO\\[7:0\\] interrupt enable"]
pub type PortSerialGpioo70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PortSerialGPIOP70INTEnbl` reader - Port Serial GPIOP\\[7:0\\] interrupt enable"]
pub type PortSerialGpiop70intenblR = crate::FieldReader;
#[doc = "Field `PortSerialGPIOP70INTEnbl` writer - Port Serial GPIOP\\[7:0\\] interrupt enable"]
pub type PortSerialGpiop70intenblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiom70intenbl(&self) -> PortSerialGpiom70intenblR {
        PortSerialGpiom70intenblR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpion70intenbl(&self) -> PortSerialGpion70intenblR {
        PortSerialGpion70intenblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioo70intenbl(&self) -> PortSerialGpioo70intenblR {
        PortSerialGpioo70intenblR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiop70intenbl(&self) -> PortSerialGpiop70intenblR {
        PortSerialGpiop70intenblR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Port Serial GPIOM\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiom70intenbl(&mut self) -> PortSerialGpiom70intenblW<Gpio594Spec> {
        PortSerialGpiom70intenblW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Port Serial GPION\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpion70intenbl(&mut self) -> PortSerialGpion70intenblW<Gpio594Spec> {
        PortSerialGpion70intenblW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Port Serial GPIOO\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpioo70intenbl(&mut self) -> PortSerialGpioo70intenblW<Gpio594Spec> {
        PortSerialGpioo70intenblW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Port Serial GPIOP\\[7:0\\] interrupt enable"]
    #[inline(always)]
    pub fn port_serial_gpiop70intenbl(&mut self) -> PortSerialGpiop70intenblW<Gpio594Spec> {
        PortSerialGpiop70intenblW::new(self, 24)
    }
}
#[doc = "Serial GPIO\\_M/N/O/P 1 Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio594::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio594::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio594Spec;
impl crate::RegisterSpec for Gpio594Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio594::R`](R) reader structure"]
impl crate::Readable for Gpio594Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio594::W`](W) writer structure"]
impl crate::Writable for Gpio594Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO594 to value 0"]
impl crate::Resettable for Gpio594Spec {}
