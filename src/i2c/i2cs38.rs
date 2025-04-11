#[doc = "Register `I2CS38` reader"]
pub type R = crate::R<I2cs38Spec>;
#[doc = "Register `I2CS38` writer"]
pub type W = crate::W<I2cs38Spec>;
#[doc = "Field `SDRAMDMABufferBaseAddr2` reader - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddr2R = crate::FieldReader<u32>;
#[doc = "Field `SDRAMDMABufferBaseAddr2` writer - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddr2W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `Reserved0x12` reader - Reserved (0x1)"]
pub type Reserved0x12R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr2(&self) -> SdramdmabufferBaseAddr2R {
        SdramdmabufferBaseAddr2R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0x1)"]
    #[inline(always)]
    pub fn reserved0x12(&self) -> Reserved0x12R {
        Reserved0x12R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr2(&mut self) -> SdramdmabufferBaseAddr2W<I2cs38Spec> {
        SdramdmabufferBaseAddr2W::new(self, 0)
    }
}
#[doc = "\newver{Slave~ DMA Mode Tx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs38::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs38::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs38Spec;
impl crate::RegisterSpec for I2cs38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs38::R`](R) reader structure"]
impl crate::Readable for I2cs38Spec {}
#[doc = "`write(|w| ..)` method takes [`i2cs38::W`](W) writer structure"]
impl crate::Writable for I2cs38Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS38 to value 0"]
impl crate::Resettable for I2cs38Spec {}
