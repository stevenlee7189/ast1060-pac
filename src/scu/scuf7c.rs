#[doc = "Register `SCUF7C` reader"]
pub type R = crate::R<Scuf7cSpec>;
#[doc = "Register `SCUF7C` writer"]
pub type W = crate::W<Scuf7cSpec>;
#[doc = "Field `Reserved2` reader - Reserved(0)"]
pub type Reserved2R = crate::FieldReader<u16>;
#[doc = "Field `EnblWrProtOfSCUF90` reader - Enable Write Protection of hlinkSCUF90"]
pub type EnblWrProtOfScuf90R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUF90` writer - Enable Write Protection of hlinkSCUF90"]
pub type EnblWrProtOfScuf90W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved(0)"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUF98` reader - Enable Write Protection of hlinkSCUF98"]
pub type EnblWrProtOfScuf98R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUF98` writer - Enable Write Protection of hlinkSCUF98"]
pub type EnblWrProtOfScuf98W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUF9C` reader - Enable Write Protection of hlinkSCUF9C"]
pub type EnblWrProtOfScuf9cR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUF9C` writer - Enable Write Protection of hlinkSCUF9C"]
pub type EnblWrProtOfScuf9cW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFB0` reader - Enable Write Protection of hlinkSCUFB0"]
pub type EnblWrProtOfScufb0R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFB0` writer - Enable Write Protection of hlinkSCUFB0"]
pub type EnblWrProtOfScufb0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFB4` reader - Enable Write Protection of hlinkSCUFB4"]
pub type EnblWrProtOfScufb4R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFB4` writer - Enable Write Protection of hlinkSCUFB4"]
pub type EnblWrProtOfScufb4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFB8` reader - Enable Write Protection of hlinkSCUFB8"]
pub type EnblWrProtOfScufb8R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFB8` writer - Enable Write Protection of hlinkSCUFB8"]
pub type EnblWrProtOfScufb8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFBC` reader - Enable Write Protection of hlinkSCUFBC"]
pub type EnblWrProtOfScufbcR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFBC` writer - Enable Write Protection of hlinkSCUFBC"]
pub type EnblWrProtOfScufbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFD0` reader - Enable Write Protection of hlinkSCUFD0"]
pub type EnblWrProtOfScufd0R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFD0` writer - Enable Write Protection of hlinkSCUFD0"]
pub type EnblWrProtOfScufd0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFD4` reader - Enable Write Protection of hlinkSCUFD4"]
pub type EnblWrProtOfScufd4R = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFD4` writer - Enable Write Protection of hlinkSCUFD4"]
pub type EnblWrProtOfScufd4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWrProtOfSCUFFC` reader - Enable Write Protection of hlinkSCUFFC"]
pub type EnblWrProtOfScuffcR = crate::BitReader;
#[doc = "Field `EnblWrProtOfSCUFFC` writer - Enable Write Protection of hlinkSCUFFC"]
pub type EnblWrProtOfScuffcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF90"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf90(&self) -> EnblWrProtOfScuf90R {
        EnblWrProtOfScuf90R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved(0)"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF98"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf98(&self) -> EnblWrProtOfScuf98R {
        EnblWrProtOfScuf98R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable Write Protection of hlinkSCUF9C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf9c(&self) -> EnblWrProtOfScuf9cR {
        EnblWrProtOfScuf9cR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Write Protection of hlinkSCUFB0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufb0(&self) -> EnblWrProtOfScufb0R {
        EnblWrProtOfScufb0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable Write Protection of hlinkSCUFB4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufb4(&self) -> EnblWrProtOfScufb4R {
        EnblWrProtOfScufb4R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable Write Protection of hlinkSCUFB8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufb8(&self) -> EnblWrProtOfScufb8R {
        EnblWrProtOfScufb8R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable Write Protection of hlinkSCUFBC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufbc(&self) -> EnblWrProtOfScufbcR {
        EnblWrProtOfScufbcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Write Protection of hlinkSCUFD0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufd0(&self) -> EnblWrProtOfScufd0R {
        EnblWrProtOfScufd0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Write Protection of hlinkSCUFD4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufd4(&self) -> EnblWrProtOfScufd4R {
        EnblWrProtOfScufd4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFFC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuffc(&self) -> EnblWrProtOfScuffcR {
        EnblWrProtOfScuffcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Enable Write Protection of hlinkSCUF90"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf90(&mut self) -> EnblWrProtOfScuf90W<Scuf7cSpec> {
        EnblWrProtOfScuf90W::new(self, 16)
    }
    #[doc = "Bit 18 - Enable Write Protection of hlinkSCUF98"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf98(&mut self) -> EnblWrProtOfScuf98W<Scuf7cSpec> {
        EnblWrProtOfScuf98W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable Write Protection of hlinkSCUF9C"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuf9c(&mut self) -> EnblWrProtOfScuf9cW<Scuf7cSpec> {
        EnblWrProtOfScuf9cW::new(self, 19)
    }
    #[doc = "Bit 20 - Enable Write Protection of hlinkSCUFB0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufb0(&mut self) -> EnblWrProtOfScufb0W<Scuf7cSpec> {
        EnblWrProtOfScufb0W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable Write Protection of hlinkSCUFB4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufb4(&mut self) -> EnblWrProtOfScufb4W<Scuf7cSpec> {
        EnblWrProtOfScufb4W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable Write Protection of hlinkSCUFB8"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufb8(&mut self) -> EnblWrProtOfScufb8W<Scuf7cSpec> {
        EnblWrProtOfScufb8W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable Write Protection of hlinkSCUFBC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufbc(&mut self) -> EnblWrProtOfScufbcW<Scuf7cSpec> {
        EnblWrProtOfScufbcW::new(self, 23)
    }
    #[doc = "Bit 24 - Enable Write Protection of hlinkSCUFD0"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufd0(&mut self) -> EnblWrProtOfScufd0W<Scuf7cSpec> {
        EnblWrProtOfScufd0W::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Write Protection of hlinkSCUFD4"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scufd4(&mut self) -> EnblWrProtOfScufd4W<Scuf7cSpec> {
        EnblWrProtOfScufd4W::new(self, 25)
    }
    #[doc = "Bit 31 - Enable Write Protection of hlinkSCUFFC"]
    #[inline(always)]
    pub fn enbl_wr_prot_of_scuffc(&mut self) -> EnblWrProtOfScuffcW<Scuf7cSpec> {
        EnblWrProtOfScuffcW::new(self, 31)
    }
}
#[doc = "Write Protect Register \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`scuf7c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuf7c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuf7cSpec;
impl crate::RegisterSpec for Scuf7cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuf7c::R`](R) reader structure"]
impl crate::Readable for Scuf7cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuf7c::W`](W) writer structure"]
impl crate::Writable for Scuf7cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUF7C to value 0"]
impl crate::Resettable for Scuf7cSpec {}
