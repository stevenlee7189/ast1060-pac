#[doc = "Register `FMCFQCD[%s]` reader"]
pub type R = crate::R<FmcfqcdSpec>;
#[doc = "Register `FMCFQCD[%s]` writer"]
pub type W = crate::W<FmcfqcdSpec>;
#[doc = "Field `FullyQualifiedCmd17` reader - Fully qualified Command"]
pub type FullyQualifiedCmd17R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd17` writer - Fully qualified Command"]
pub type FullyQualifiedCmd17W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd8` reader - Fully qualified Command"]
pub type FullyQualifiedCmd8R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd8` writer - Fully qualified Command"]
pub type FullyQualifiedCmd8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry8` reader - Enable Entry"]
pub type EnblEntry8R = crate::BitReader;
#[doc = "Field `EnblEntry8` writer - Enable Entry"]
pub type EnblEntry8W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd17(&self) -> FullyQualifiedCmd17R {
        FullyQualifiedCmd17R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd8(&self) -> FullyQualifiedCmd8R {
        FullyQualifiedCmd8R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry8(&self) -> EnblEntry8R {
        EnblEntry8R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd17(&mut self) -> FullyQualifiedCmd17W<FmcfqcdSpec> {
        FullyQualifiedCmd17W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd8(&mut self) -> FullyQualifiedCmd8W<FmcfqcdSpec> {
        FullyQualifiedCmd8W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry8(&mut self) -> EnblEntry8W<FmcfqcdSpec> {
        EnblEntry8W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmcfqcd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmcfqcd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmcfqcdSpec;
impl crate::RegisterSpec for FmcfqcdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmcfqcd::R`](R) reader structure"]
impl crate::Readable for FmcfqcdSpec {}
#[doc = "`write(|w| ..)` method takes [`fmcfqcd::W`](W) writer structure"]
impl crate::Writable for FmcfqcdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMCFQCD[%s] to value 0"]
impl crate::Resettable for FmcfqcdSpec {}
