#[doc = "Register `SPIRBUF[%s]` reader"]
pub type R = crate::R<SpirbufSpec>;
#[doc = "Register `SPIRBUF[%s]` writer"]
pub type W = crate::W<SpirbufSpec>;
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
    pub fn dmabuffer_readwr_port(&mut self) -> DmabufferReadwrPortW<SpirbufSpec> {
        DmabufferReadwrPortW::new(self, 0)
    }
}
#[doc = "DMA Buffer R/W Port\n\nYou can [`read`](crate::Reg::read) this register and get [`spirbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spirbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SpirbufSpec;
impl crate::RegisterSpec for SpirbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spirbuf::R`](R) reader structure"]
impl crate::Readable for SpirbufSpec {}
#[doc = "`write(|w| ..)` method takes [`spirbuf::W`](W) writer structure"]
impl crate::Writable for SpirbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIRBUF[%s] to value 0"]
impl crate::Resettable for SpirbufSpec {}
