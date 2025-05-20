#[doc = "Register `FMC110` reader"]
pub type R = crate::R<Fmc110Spec>;
#[doc = "Register `FMC110` writer"]
pub type W = crate::W<Fmc110Spec>;
#[doc = "Field `FullyQualifiedCmd13` reader - Fully qualified Command"]
pub type FullyQualifiedCmd13R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd13` writer - Fully qualified Command"]
pub type FullyQualifiedCmd13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FullyQualifiedCmd4` reader - Fully qualified Command"]
pub type FullyQualifiedCmd4R = crate::FieldReader;
#[doc = "Field `FullyQualifiedCmd4` writer - Fully qualified Command"]
pub type FullyQualifiedCmd4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::FieldReader<u16>;
#[doc = "Field `EnblEntry4` reader - Enable Entry"]
pub type EnblEntry4R = crate::BitReader;
#[doc = "Field `EnblEntry4` writer - Enable Entry"]
pub type EnblEntry4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd13(&self) -> FullyQualifiedCmd13R {
        FullyQualifiedCmd13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd4(&self) -> FullyQualifiedCmd4R {
        FullyQualifiedCmd4R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:30 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry4(&self) -> EnblEntry4R {
        EnblEntry4R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd13(&mut self) -> FullyQualifiedCmd13W<Fmc110Spec> {
        FullyQualifiedCmd13W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Fully qualified Command"]
    #[inline(always)]
    pub fn fully_qualified_cmd4(&mut self) -> FullyQualifiedCmd4W<Fmc110Spec> {
        FullyQualifiedCmd4W::new(self, 8)
    }
    #[doc = "Bit 31 - Enable Entry"]
    #[inline(always)]
    pub fn enbl_entry4(&mut self) -> EnblEntry4W<Fmc110Spec> {
        EnblEntry4W::new(self, 31)
    }
}
#[doc = "Fully Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc110Spec;
impl crate::RegisterSpec for Fmc110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc110::R`](R) reader structure"]
impl crate::Readable for Fmc110Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc110::W`](W) writer structure"]
impl crate::Writable for Fmc110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC110 to value 0x8000_0001"]
impl crate::Resettable for Fmc110Spec {
    const RESET_VALUE: u32 = 0x8000_0001;
}
