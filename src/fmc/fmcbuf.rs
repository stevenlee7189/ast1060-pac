#[doc = "Register `FMCBUF[%s]` reader"]
pub type R = crate::R<FmcbufSpec>;
#[doc = "Register `FMCBUF[%s]` writer"]
pub type W = crate::W<FmcbufSpec>;
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
    pub fn dmabuffer_readwr_port(&mut self) -> DmabufferReadwrPortW<FmcbufSpec> {
        DmabufferReadwrPortW::new(self, 0)
    }
}
#[doc = "DMA Buffer Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmcbuf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcbuf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmcbufSpec;
impl crate::RegisterSpec for FmcbufSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcbuf::R`](R) reader structure"]
impl crate::Readable for FmcbufSpec {}
#[doc = "`write(|w| ..)` method takes [`fmcbuf::W`](W) writer structure"]
impl crate::Writable for FmcbufSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMCBUF[%s] to value 0"]
impl crate::Resettable for FmcbufSpec {}
