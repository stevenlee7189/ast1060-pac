#[doc = "Register `SCUF1C` reader"]
pub type R = crate::R<Scuf1cSpec>;
#[doc = "Register `SCUF1C` writer"]
pub type W = crate::W<Scuf1cSpec>;
#[doc = "Field `EnblWrProtOfSCU310` reader - Enable Write Protection of hlinkSCU310"]
pub type EnblWrProtOfScu310R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU310` writer - Enable Write Protection of hlinkSCU310"]
pub type EnblWrProtOfScu310W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU314` reader - Enable Write Protection of hlinkSCU314"]
pub type EnblWrProtOfScu314R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU314` writer - Enable Write Protection of hlinkSCU314"]
pub type EnblWrProtOfScu314W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - Reserved(0)"]
pub type Reserved4R = crate::FieldReader;
#[doc = "Field `EnblWrProtOfSCU330` reader - Enable Write Protection of hlinkSCU330"]
pub type EnblWrProtOfScu330R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU330` writer - Enable Write Protection of hlinkSCU330"]
pub type EnblWrProtOfScu330W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU334` reader - Enable Write Protection of hlinkSCU334"]
pub type EnblWrProtOfScu334R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU334` writer - Enable Write Protection of hlinkSCU334"]
pub type EnblWrProtOfScu334W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU338` reader - Enable Write Protection of hlinkSCU338"]
pub type EnblWrProtOfScu338R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU338` writer - Enable Write Protection of hlinkSCU338"]
pub type EnblWrProtOfScu338W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCU33C` reader - Enable Write Protection of hlinkSCU33C"]
pub type EnblWrProtOfScu33cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCU33C` writer - Enable Write Protection of hlinkSCU33C"]
pub type EnblWrProtOfScu33cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `Reserved3` writer - Reserved"]
pub type Reserved3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU310"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu310(&self) -> EnblWrProtOfScu310R {
        EnblWrProtOfScu310R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu314(&self) -> EnblWrProtOfScu314R {
        EnblWrProtOfScu314R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu330(&self) -> EnblWrProtOfScu330R {
        EnblWrProtOfScu330R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu334(&self) -> EnblWrProtOfScu334R {
        EnblWrProtOfScu334R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Write Protection of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu338(&self) -> EnblWrProtOfScu338R {
        EnblWrProtOfScu338R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Write Protection of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu33c(&self) -> EnblWrProtOfScu33cR {
        EnblWrProtOfScu33cR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Write Protection of hlinkSCU310"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu310(&mut self) -> EnblWrProtOfScu310W<Scuf1cSpec> {
        EnblWrProtOfScu310W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Write Protection of hlinkSCU314"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu314(&mut self) -> EnblWrProtOfScu314W<Scuf1cSpec> {
        EnblWrProtOfScu314W::new(self, 1)
    }
    #[doc = "Bit 4 - Enable Write Protection of hlinkSCU330"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu330(&mut self) -> EnblWrProtOfScu330W<Scuf1cSpec> {
        EnblWrProtOfScu330W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Write Protection of hlinkSCU334"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu334(&mut self) -> EnblWrProtOfScu334W<Scuf1cSpec> {
        EnblWrProtOfScu334W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Write Protection of hlinkSCU338"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu338(&mut self) -> EnblWrProtOfScu338W<Scuf1cSpec> {
        EnblWrProtOfScu338W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Write Protection of hlinkSCU33C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scu33c(&mut self) -> EnblWrProtOfScu33cW<Scuf1cSpec> {
        EnblWrProtOfScu33cW::new(self, 7)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&mut self) -> Reserved3W<Scuf1cSpec> {
        Reserved3W::new(self, 8)
    }
    #[doc = "Bits 10:13 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scuf1cSpec> {
        Reserved1W::new(self, 10)
    }
}
#[doc = "Write Protect Register \\#8\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf1cSpec;
impl crate::RegisterSpec for Scuf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf1c::R`](R) reader structure"]
impl crate::Readable for Scuf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf1c::W`](W) writer structure"]
impl crate::Writable for Scuf1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF1C to value 0"]
impl crate::Resettable for Scuf1cSpec {}
