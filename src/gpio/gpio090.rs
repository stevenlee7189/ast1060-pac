#[doc = "Register `GPIO090` reader"]
pub type R = crate::R<Gpio090Spec>;
#[doc = "Register `GPIO090` writer"]
pub type W = crate::W<Gpio090Spec>;
#[doc = "Field `PortGPIOI70CmdSource0` reader - Port GPIOI\\[7:0\\] Command Source 0"]
pub type PortGpioi70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOI70CmdSource0` writer - Port GPIOI\\[7:0\\] Command Source 0"]
pub type PortGpioi70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOJ70CmdSource0` reader - Port GPIOJ\\[7:0\\] Command Source 0"]
pub type PortGpioj70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOJ70CmdSource0` writer - Port GPIOJ\\[7:0\\] Command Source 0"]
pub type PortGpioj70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOK70CmdSource0` reader - Port GPIOK\\[7:0\\] Command Source 0"]
pub type PortGpiok70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOK70CmdSource0` writer - Port GPIOK\\[7:0\\] Command Source 0"]
pub type PortGpiok70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOL70CmdSource0` reader - Port GPIOL\\[7:0\\] Command Source 0"]
pub type PortGpiol70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOL70CmdSource0` writer - Port GPIOL\\[7:0\\] Command Source 0"]
pub type PortGpiol70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOI\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioi70cmd_source0(&self) -> PortGpioi70cmdSource0R {
        PortGpioi70cmdSource0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOJ\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioj70cmd_source0(&self) -> PortGpioj70cmdSource0R {
        PortGpioj70cmdSource0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOK\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiok70cmd_source0(&self) -> PortGpiok70cmdSource0R {
        PortGpiok70cmdSource0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOL\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiol70cmd_source0(&self) -> PortGpiol70cmdSource0R {
        PortGpiol70cmdSource0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOI\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioi70cmd_source0(&mut self) -> PortGpioi70cmdSource0W<Gpio090Spec> {
        PortGpioi70cmdSource0W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOJ\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioj70cmd_source0(&mut self) -> PortGpioj70cmdSource0W<Gpio090Spec> {
        PortGpioj70cmdSource0W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOK\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiok70cmd_source0(&mut self) -> PortGpiok70cmdSource0W<Gpio090Spec> {
        PortGpiok70cmdSource0W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOL\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiol70cmd_source0(&mut self) -> PortGpiol70cmdSource0W<Gpio090Spec> {
        PortGpiol70cmdSource0W::new(self, 24)
    }
}
#[doc = "GPIO\\_I/J/K/L Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio090::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio090::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio090Spec;
impl crate::RegisterSpec for Gpio090Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio090::R`](R) reader structure"]
impl crate::Readable for Gpio090Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio090::W`](W) writer structure"]
impl crate::Writable for Gpio090Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO090 to value 0"]
impl crate::Resettable for Gpio090Spec {}
