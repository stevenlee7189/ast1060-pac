#[doc = "Register `SPI260` reader"]
pub type R = crate::R<Spi260Spec>;
#[doc = "Register `SPI260` writer"]
pub type W = crate::W<Spi260Spec>;
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
    pub fn dmabuffer_readwr_port(&mut self) -> DmabufferReadwrPortW<Spi260Spec> {
        DmabufferReadwrPortW::new(self, 0)
    }
}
#[doc = "DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spi260::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi260::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi260Spec;
impl crate::RegisterSpec for Spi260Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi260::R`](R) reader structure"]
impl crate::Readable for Spi260Spec {}
#[doc = "`write(|w| ..)` method takes [`spi260::W`](W) writer structure"]
impl crate::Writable for Spi260Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI260 to value 0"]
impl crate::Resettable for Spi260Spec {}
