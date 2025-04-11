#[doc = "Register `SPI114` reader"]
pub type R = crate::R<Spi114Spec>;
#[doc = "Register `SPI114` writer"]
pub type W = crate::W<Spi114Spec>;
#[doc = "Field `FullyQualifiedCmd14` reader - Fully qualified Command"]
pub type FullyQualifiedCmd14R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd14` writer - Fully qualified Command"]
pub type FullyQualifiedCmd14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd5` reader - Fully qualified Command"]
pub type FullyQualifiedCmd5R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd5` writer - Fully qualified Command"]
pub type FullyQualifiedCmd5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry5` reader - Enable Entry"]
pub type EnblEntry5R = crate::BitReader;
#[doc = "Field `EnblEntry5` writer - Enable Entry"]
pub type EnblEntry5W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd14(&self) -> FullyQualifiedCmd14R {
        FullyQualifiedCmd14R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd5(&self) -> FullyQualifiedCmd5R {
        FullyQualifiedCmd5R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry5(&self) -> EnblEntry5R {
        EnblEntry5R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd14(&mut self) -> FullyQualifiedCmd14W<Spi114Spec> {
        FullyQualifiedCmd14W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd5(&mut self) -> FullyQualifiedCmd5W<Spi114Spec> {
        FullyQualifiedCmd5W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry5(&mut self) -> EnblEntry5W<Spi114Spec> {
        EnblEntry5W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi114::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi114::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi114Spec;
impl crate::RegisterSpec for Spi114Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi114::R`](R) reader structure"]
impl crate::Readable for Spi114Spec {}
#[doc = "`write(|w| ..)` method takes [`spi114::W`](W) writer structure"]
impl crate::Writable for Spi114Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI114 to value 0x8000_0606"]
impl crate::Resettable for Spi114Spec {
    const RESET_VALUE: u32 = 0x8000_0606;
}
