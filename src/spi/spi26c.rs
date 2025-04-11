#[doc = "Register `SPI26C` reader"]
pub type R = crate::R<Spi26cSpec>;
#[doc = "Register `SPI26C` writer"]
pub type W = crate::W<Spi26cSpec>;
#[doc = "Field `DMABufferReadwrPort` reader - DMA Buffer read/write port"]
pub type DmabufferReadwrPortR = crate::FieldReader<u32>;
#[doc = "Field `DMABufferReadwrPort` writer - DMA Buffer read/write port"]
pub type DmabufferReadwrPortW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Buffer read/write port"]
    #[inline(always)]
    pub fn dmabuffer_readwr_port(&self) -> DmabufferReadwrPortR {
        DmabufferReadwrPortR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Buffer read/write port"]
    #[inline(always)]
    pub fn dmabuffer_readwr_port(&mut self) -> DmabufferReadwrPortW<Spi26cSpec> {
        DmabufferReadwrPortW::new(self, 0)
    }
}
#[doc = "DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi26c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi26c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi26cSpec;
impl crate::RegisterSpec for Spi26cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi26c::R`](R) reader structure"]
impl crate::Readable for Spi26cSpec {}
#[doc = "`write(|w| ..)` method takes [`spi26c::W`](W) writer structure"]
impl crate::Writable for Spi26cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI26C to value 0"]
impl crate::Resettable for Spi26cSpec {}
