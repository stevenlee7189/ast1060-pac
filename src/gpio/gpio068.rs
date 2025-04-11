#[doc = "Register `GPIO068` reader"]
pub type R = crate::R<Gpio068Spec>;
#[doc = "Register `GPIO068` writer"]
pub type W = crate::W<Gpio068Spec>;
#[doc = "Field `PortGPIOE70CmdSource0` reader - Port GPIOE\\[7:0\\] Command Source 0"]
pub type PortGpioe70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOE70CmdSource0` writer - Port GPIOE\\[7:0\\] Command Source 0"]
pub type PortGpioe70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `PortGPIOF70CmdSource0` reader - Port GPIOF\\[7:0\\] Command Source 0"]
pub type PortGpiof70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOF70CmdSource0` writer - Port GPIOF\\[7:0\\] Command Source 0"]
pub type PortGpiof70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `PortGPIOG70CmdSource0` reader - Port GPIOG\\[7:0\\] Command Source 0"]
pub type PortGpiog70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOG70CmdSource0` writer - Port GPIOG\\[7:0\\] Command Source 0"]
pub type PortGpiog70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `PortGPIOH70CmdSource0` reader - Port GPIOH\\[7:0\\] Command Source 0"]
pub type PortGpioh70cmdSource0R = crate::BitReader;
#[doc = "Field `PortGPIOH70CmdSource0` writer - Port GPIOH\\[7:0\\] Command Source 0"]
pub type PortGpioh70cmdSource0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port GPIOE\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioe70cmd_source0(&self) -> PortGpioe70cmdSource0R {
        PortGpioe70cmdSource0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Port GPIOF\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiof70cmd_source0(&self) -> PortGpiof70cmdSource0R {
        PortGpiof70cmdSource0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Port GPIOG\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiog70cmd_source0(&self) -> PortGpiog70cmdSource0R {
        PortGpiog70cmdSource0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Port GPIOH\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioh70cmd_source0(&self) -> PortGpioh70cmdSource0R {
        PortGpioh70cmdSource0R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port GPIOE\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioe70cmd_source0(&mut self) -> PortGpioe70cmdSource0W<Gpio068Spec> {
        PortGpioe70cmdSource0W::new(self, 0)
    }
    #[doc = "Bit 8 - Port GPIOF\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiof70cmd_source0(&mut self) -> PortGpiof70cmdSource0W<Gpio068Spec> {
        PortGpiof70cmdSource0W::new(self, 8)
    }
    #[doc = "Bit 16 - Port GPIOG\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpiog70cmd_source0(&mut self) -> PortGpiog70cmdSource0W<Gpio068Spec> {
        PortGpiog70cmdSource0W::new(self, 16)
    }
    #[doc = "Bit 24 - Port GPIOH\\[7:0\\] Command Source 0"]
    #[inline(always)]
    pub fn port_gpioh70cmd_source0(&mut self) -> PortGpioh70cmdSource0W<Gpio068Spec> {
        PortGpioh70cmdSource0W::new(self, 24)
    }
}
#[doc = "GPIO\\_E/F/G/H Command Source 0\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio068Spec;
impl crate::RegisterSpec for Gpio068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio068::R`](R) reader structure"]
impl crate::Readable for Gpio068Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio068::W`](W) writer structure"]
impl crate::Writable for Gpio068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO068 to value 0"]
impl crate::Resettable for Gpio068Spec {}
