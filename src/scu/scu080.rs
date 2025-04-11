#[doc = "Register `SCU080` reader"]
pub type R = crate::R<Scu080Spec>;
#[doc = "Register `SCU080` writer"]
pub type W = crate::W<Scu080Spec>;
#[doc = "Field `StopMCLKForSRAMCtrl` reader - Stop MCLK (For SRAM Controller)"]
pub type StopMclkforSramctrlR = crate::BitReader;
#[doc = "Field `StopMCLKForSRAMCtrl` writer - Stop MCLK (For SRAM Controller)"]
pub type StopMclkforSramctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved, must keep at value \"111111\""]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved3` writer - Reserved, must keep at value \"111111\""]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved, must keep at value \"11111\""]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved, must keep at value \"11111\""]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `StopYCLKForHACE` reader - Stop YCLK (For HACE)"]
pub type StopYclkforHaceR = crate::BitReader;
#[doc = "Field `StopYCLKForHACE` writer - Stop YCLK (For HACE)"]
pub type StopYclkforHaceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop MCLK (For SRAM Controller)"]
    #[inline(always)]
    pub fn stop_mclkfor_sramctrl(&self) -> StopMclkforSramctrlR {
        StopMclkforSramctrlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - Reserved, must keep at value \"111111\""]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Reserved, must keep at value \"11111\""]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Stop YCLK (For HACE)"]
    #[inline(always)]
    pub fn stop_yclkfor_hace(&self) -> StopYclkforHaceR {
        StopYclkforHaceR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop MCLK (For SRAM Controller)"]
    #[inline(always)]
    pub fn stop_mclkfor_sramctrl(&mut self) -> StopMclkforSramctrlW<Scu080Spec> {
        StopMclkforSramctrlW::new(self, 0)
    }
    #[doc = "Bits 1:6 - Reserved, must keep at value \"111111\""]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scu080Spec> {
        Reserved3W::new(self, 1)
    }
    #[doc = "Bit 7 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu080Spec> {
        Reserved2W::new(self, 7)
    }
    #[doc = "Bits 8:12 - Reserved, must keep at value \"11111\""]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu080Spec> {
        Reserved1W::new(self, 8)
    }
    #[doc = "Bit 13 - Stop YCLK (For HACE)"]
    #[inline(always)]
    pub fn stop_yclkfor_hace(&mut self) -> StopYclkforHaceW<Scu080Spec> {
        StopYclkforHaceW::new(self, 13)
    }
}
#[doc = "Clock Stop Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu080Spec;
impl crate::RegisterSpec for Scu080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu080::R`](R) reader structure"]
impl crate::Readable for Scu080Spec {}
#[doc = "`write(|w| ..)` method takes [`scu080::W`](W) writer structure"]
impl crate::Writable for Scu080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU080 to value 0xffff_7f8a"]
impl crate::Resettable for Scu080Spec {
    const RESET_VALUE: u32 = 0xffff_7f8a;
}
