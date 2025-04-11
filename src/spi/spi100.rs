#[doc = "Register `SPI100` reader"]
pub type R = crate::R<Spi100Spec>;
#[doc = "Register `SPI100` writer"]
pub type W = crate::W<Spi100Spec>;
#[doc = "Field `FullyQualifiedCmd9` reader - Fully qualified Command"]
pub type FullyQualifiedCmd9R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd9` writer - Fully qualified Command"]
pub type FullyQualifiedCmd9W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd` reader - Fully qualified Command"]
pub type FullyQualifiedCmdR = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd` writer - Fully qualified Command"]
pub type FullyQualifiedCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EnblEntry` reader - Enable Entry"]
pub type EnblEntryR = crate::BitReader;
#[doc = "Field `EnblEntry` writer - Enable Entry"]
pub type EnblEntryW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd9(&self) -> FullyQualifiedCmd9R {
        FullyQualifiedCmd9R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd(&self) -> FullyQualifiedCmdR {
        FullyQualifiedCmdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry(&self) -> EnblEntryR {
        EnblEntryR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd9(&mut self) -> FullyQualifiedCmd9W<Spi100Spec> {
        FullyQualifiedCmd9W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd(&mut self) -> FullyQualifiedCmdW<Spi100Spec> {
        FullyQualifiedCmdW::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry(&mut self) -> EnblEntryW<Spi100Spec> {
        EnblEntryW::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`spi100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi100Spec;
impl crate::RegisterSpec for Spi100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi100::R`](R) reader structure"]
impl crate::Readable for Spi100Spec {}
#[doc = "`write(|w| ..)` method takes [`spi100::W`](W) writer structure"]
impl crate::Writable for Spi100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI100 to value 0x8000_1303"]
impl crate::Resettable for Spi100Spec {
    const RESET_VALUE: u32 = 0x8000_1303;
}
