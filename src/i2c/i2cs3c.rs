#[doc = "Register `I2CS3C` reader"]
pub type R = crate::R<I2cs3cSpec>;
#[doc = "Register `I2CS3C` writer"]
pub type W = crate::W<I2cs3cSpec>;
#[doc = "Field `SDRAMDMABufferBaseAddr3` reader - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddr3R = crate::FieldReader<u32>;
#[doc = "Field `SDRAMDMABufferBaseAddr3` writer - SDRAM DMA Buffer Base Address"]
pub type SdramdmabufferBaseAddr3W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `Reserved0x13` reader - Reserved (0x1)"]
pub type Reserved0x13R = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr3(&self) -> SdramdmabufferBaseAddr3R {
        SdramdmabufferBaseAddr3R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0x1)"]
    #[inline(always)]
    pub fn reserved0x13(&self) -> Reserved0x13R {
        Reserved0x13R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - SDRAM DMA Buffer Base Address"]
    #[inline(always)]
    pub fn sdramdmabuffer_base_addr3(&mut self) -> SdramdmabufferBaseAddr3W<I2cs3cSpec> {
        SdramdmabufferBaseAddr3W::new(self, 0)
    }
}
#[doc = "\newver{Slave~ DMA Mode Rx Buffer Base Address\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs3c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs3c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs3cSpec;
impl crate::RegisterSpec for I2cs3cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs3c::R`](R) reader structure"]
impl crate::Readable for I2cs3cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cs3c::W`](W) writer structure"]
impl crate::Writable for I2cs3cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS3C to value 0"]
impl crate::Resettable for I2cs3cSpec {}
