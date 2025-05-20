#[doc = "Register `FMC050` reader"]
pub type R = crate::R<Fmc050Spec>;
#[doc = "Register `FMC050` writer"]
pub type W = crate::W<Fmc050Spec>;
#[doc = "Field `EnblGenerateSoftRstCmd` reader - Enable generate soft-reset command"]
pub type EnblGenerateSoftRstCmdR = crate::BitReader;
#[doc = "Field `EnblGenerateSoftRstCmd` writer - Enable generate soft-reset command"]
pub type EnblGenerateSoftRstCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblWaitSPIWIPIdle` reader - Enable wait SPI WIP idle"]
pub type EnblWaitSpiwipidleR = crate::BitReader;
#[doc = "Field `EnblWaitSPIWIPIdle` writer - Enable wait SPI WIP idle"]
pub type EnblWaitSpiwipidleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QPIModeForRstCmd` reader - QPI mode for reset command"]
pub type QpimodeForRstCmdR = crate::BitReader;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `LatestReadSPIStsReg` reader - Latest read SPI status register"]
pub type LatestReadSpistsRegR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Enable generate soft-reset command"]
    #[inline(always)]
    pub fn enbl_generate_soft_rst_cmd(&self) -> EnblGenerateSoftRstCmdR {
        EnblGenerateSoftRstCmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable wait SPI WIP idle"]
    #[inline(always)]
    pub fn enbl_wait_spiwipidle(&self) -> EnblWaitSpiwipidleR {
        EnblWaitSpiwipidleR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - QPI mode for reset command"]
    #[inline(always)]
    pub fn qpimode_for_rst_cmd(&self) -> QpimodeForRstCmdR {
        QpimodeForRstCmdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:7 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - Latest read SPI status register"]
    #[inline(always)]
    pub fn latest_read_spists_reg(&self) -> LatestReadSpistsRegR {
        LatestReadSpistsRegR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Enable generate soft-reset command"]
    #[inline(always)]
    pub fn enbl_generate_soft_rst_cmd(&mut self) -> EnblGenerateSoftRstCmdW<Fmc050Spec> {
        EnblGenerateSoftRstCmdW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable wait SPI WIP idle"]
    #[inline(always)]
    pub fn enbl_wait_spiwipidle(&mut self) -> EnblWaitSpiwipidleW<Fmc050Spec> {
        EnblWaitSpiwipidleW::new(self, 1)
    }
}
#[doc = "Auto Soft-Reset Command Control\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc050::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc050::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc050Spec;
impl crate::RegisterSpec for Fmc050Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc050::R`](R) reader structure"]
impl crate::Readable for Fmc050Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc050::W`](W) writer structure"]
impl crate::Writable for Fmc050Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC050 to value 0"]
impl crate::Resettable for Fmc050Spec {}
