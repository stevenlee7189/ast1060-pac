#[doc = "Register `I2CS4C` reader"]
pub type R = crate::R<I2cs4cSpec>;
#[doc = "Register `I2CS4C` writer"]
pub type W = crate::W<I2cs4cSpec>;
#[doc = "Field `DMATxActualLenByte` reader - DMA Tx actual length (Byte)"]
pub type DmatxActualLenByteR = crate::FieldReader<u16>;
#[doc = "Field `DMATxActualLenByte` writer - DMA Tx actual length (Byte)"]
pub type DmatxActualLenByteW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `DMARxActualLenByte` reader - DMA Rx actual length (Byte)"]
pub type DmarxActualLenByteR = crate::FieldReader<u16>;
#[doc = "Field `DMARxActualLenByte` writer - DMA Rx actual length (Byte)"]
pub type DmarxActualLenByteW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - DMA Tx actual length (Byte)"]
    #[inline(always)]
    pub fn dmatx_actual_len_byte(&self) -> DmatxActualLenByteR {
        DmatxActualLenByteR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - DMA Rx actual length (Byte)"]
    #[inline(always)]
    pub fn dmarx_actual_len_byte(&self) -> DmarxActualLenByteR {
        DmarxActualLenByteR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - DMA Tx actual length (Byte)"]
    #[inline(always)]
    pub fn dmatx_actual_len_byte(&mut self) -> DmatxActualLenByteW<I2cs4cSpec> {
        DmatxActualLenByteW::new(self, 0)
    }
    #[doc = "Bits 16:28 - DMA Rx actual length (Byte)"]
    #[inline(always)]
    pub fn dmarx_actual_len_byte(&mut self) -> DmarxActualLenByteW<I2cs4cSpec> {
        DmarxActualLenByteW::new(self, 16)
    }
}
#[doc = "Slave DMA Length Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2cs4c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2cs4c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cs4cSpec;
impl crate::RegisterSpec for I2cs4cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2cs4c::R`](R) reader structure"]
impl crate::Readable for I2cs4cSpec {}
#[doc = "`write(|w| ..)` method takes [`i2cs4c::W`](W) writer structure"]
impl crate::Writable for I2cs4cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2CS4C to value 0"]
impl crate::Resettable for I2cs4cSpec {}
