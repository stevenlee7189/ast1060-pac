#[doc = "Register `FMC108` reader"]
pub type R = crate::R<Fmc108Spec>;
#[doc = "Register `FMC108` writer"]
pub type W = crate::W<Fmc108Spec>;
#[doc = "Field `FullyQualifiedCmd11` reader - Fully qualified Command"]
pub type FullyQualifiedCmd11R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd11` writer - Fully qualified Command"]
pub type FullyQualifiedCmd11W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd2` reader - Fully qualified Command"]
pub type FullyQualifiedCmd2R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd2` writer - Fully qualified Command"]
pub type FullyQualifiedCmd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry2` reader - Enable Entry"]
pub type EnblEntry2R = crate::BitReader;
#[doc = "Field `EnblEntry2` writer - Enable Entry"]
pub type EnblEntry2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd11(&self) -> FullyQualifiedCmd11R {
        FullyQualifiedCmd11R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd2(&self) -> FullyQualifiedCmd2R {
        FullyQualifiedCmd2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry2(&self) -> EnblEntry2R {
        EnblEntry2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd11(&mut self) -> FullyQualifiedCmd11W<Fmc108Spec> {
        FullyQualifiedCmd11W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd2(&mut self) -> FullyQualifiedCmd2W<Fmc108Spec> {
        FullyQualifiedCmd2W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry2(&mut self) -> EnblEntry2W<Fmc108Spec> {
        EnblEntry2W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc108::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc108::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc108Spec;
impl crate::RegisterSpec for Fmc108Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc108::R`](R) reader structure"]
impl crate::Readable for Fmc108Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc108::W`](W) writer structure"]
impl crate::Writable for Fmc108Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC108 to value 0x8000_009f"]
impl crate::Resettable for Fmc108Spec {
    const RESET_VALUE: u32 = 0x8000_009f;
}
