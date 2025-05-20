#[doc = "Register `FMC104` reader"]
pub type R = crate::R<Fmc104Spec>;
#[doc = "Register `FMC104` writer"]
pub type W = crate::W<Fmc104Spec>;
#[doc = "Field `FullyQualifiedCmd10` reader - Fully qualified Command"]
pub type FullyQualifiedCmd10R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd10` writer - Fully qualified Command"]
pub type FullyQualifiedCmd10W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd1` reader - Fully qualified Command"]
pub type FullyQualifiedCmd1R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd1` writer - Fully qualified Command"]
pub type FullyQualifiedCmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry1` reader - Enable Entry"]
pub type EnblEntry1R = crate::BitReader;
#[doc = "Field `EnblEntry1` writer - Enable Entry"]
pub type EnblEntry1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd10(&self) -> FullyQualifiedCmd10R {
        FullyQualifiedCmd10R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd1(&self) -> FullyQualifiedCmd1R {
        FullyQualifiedCmd1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry1(&self) -> EnblEntry1R {
        EnblEntry1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd10(&mut self) -> FullyQualifiedCmd10W<Fmc104Spec> {
        FullyQualifiedCmd10W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd1(&mut self) -> FullyQualifiedCmd1W<Fmc104Spec> {
        FullyQualifiedCmd1W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry1(&mut self) -> EnblEntry1W<Fmc104Spec> {
        EnblEntry1W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc104::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc104::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc104Spec;
impl crate::RegisterSpec for Fmc104Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc104::R`](R) reader structure"]
impl crate::Readable for Fmc104Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc104::W`](W) writer structure"]
impl crate::Writable for Fmc104Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC104 to value 0x8000_000b"]
impl crate::Resettable for Fmc104Spec {
    const RESET_VALUE: u32 = 0x8000_000b;
}
