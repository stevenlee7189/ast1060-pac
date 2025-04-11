#[doc = "Register `SPI030` reader"]
pub type R = crate::R<Spi030Spec>;
#[doc = "Register `SPI030` writer"]
pub type W = crate::W<Spi030Spec>;
#[doc = "Field `StartAddrA3116` reader - Start address A\\[31:16\\]"]
pub type StartAddrA3116R = crate::FieldReader<u16>;
#[doc = "Field `StartAddrA3116` writer - Start address A\\[31:16\\]"]
pub type StartAddrA3116W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EndAddrA3116` reader - End address A\\[31:16\\]"]
pub type EndAddrA3116R = crate::FieldReader<u16>;
#[doc = "Field `EndAddrA3116` writer - End address A\\[31:16\\]"]
pub type EndAddrA3116W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Start address A\\[31:16\\]"]
    #[inline(always)]
    pub fn start_addr_a3116(&self) -> StartAddrA3116R {
        StartAddrA3116R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - End address A\\[31:16\\]"]
    #[inline(always)]
    pub fn end_addr_a3116(&self) -> EndAddrA3116R {
        EndAddrA3116R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Start address A\\[31:16\\]"]
    #[inline(always)]
    pub fn start_addr_a3116(&mut self) -> StartAddrA3116W<Spi030Spec> {
        StartAddrA3116W::new(self, 0)
    }
    #[doc = "Bits 16:31 - End address A\\[31:16\\]"]
    #[inline(always)]
    pub fn end_addr_a3116(&mut self) -> EndAddrA3116W<Spi030Spec> {
        EndAddrA3116W::new(self, 16)
    }
}
#[doc = "CE0 Address Decoding Range Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi030Spec;
impl crate::RegisterSpec for Spi030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi030::R`](R) reader structure"]
impl crate::Readable for Spi030Spec {}
#[doc = "`write(|w| ..)` method takes [`spi030::W`](W) writer structure"]
impl crate::Writable for Spi030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI030 to value 0x07f0_0000"]
impl crate::Resettable for Spi030Spec {
    const RESET_VALUE: u32 = 0x07f0_0000;
}
