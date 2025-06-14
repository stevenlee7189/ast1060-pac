#[doc = "Register `SPI074` reader"]
pub type R = crate::R<Spi074Spec>;
#[doc = "Register `SPI074` writer"]
pub type W = crate::W<Spi074Spec>;
#[doc = "Field `CmdForPageProgram3BMode` reader - Command for Page Program 3B mode"]
pub type CmdForPageProgram3bmodeR = crate::FieldReader;
#[doc = "Field `CmdForPageProgram3BMode` writer - Command for Page Program 3B mode"]
pub type CmdForPageProgram3bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForPageProgram4BMode` reader - Command for Page Program 4B mode"]
pub type CmdForPageProgram4bmodeR = crate::FieldReader;
#[doc = "Field `CmdForPageProgram4BMode` writer - Command for Page Program 4B mode"]
pub type CmdForPageProgram4bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForSector4KErase3BMode` reader - Command for Sector (4K) Erase 3B mode"]
pub type CmdForSector4kerase3bmodeR = crate::FieldReader;
#[doc = "Field `CmdForSector4KErase3BMode` writer - Command for Sector (4K) Erase 3B mode"]
pub type CmdForSector4kerase3bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForSector4KErase4BMode` reader - Command for Sector (4K) Erase 4B mode"]
pub type CmdForSector4kerase4bmodeR = crate::FieldReader;
#[doc = "Field `CmdForSector4KErase4BMode` writer - Command for Sector (4K) Erase 4B mode"]
pub type CmdForSector4kerase4bmodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Command for Page Program 3B mode"]
    #[inline(always)]
    pub fn cmd_for_page_program3bmode(&self) -> CmdForPageProgram3bmodeR {
        CmdForPageProgram3bmodeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command for Page Program 4B mode"]
    #[inline(always)]
    pub fn cmd_for_page_program4bmode(&self) -> CmdForPageProgram4bmodeR {
        CmdForPageProgram4bmodeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command for Sector (4K) Erase 3B mode"]
    #[inline(always)]
    pub fn cmd_for_sector4kerase3bmode(&self) -> CmdForSector4kerase3bmodeR {
        CmdForSector4kerase3bmodeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Command for Sector (4K) Erase 4B mode"]
    #[inline(always)]
    pub fn cmd_for_sector4kerase4bmode(&self) -> CmdForSector4kerase4bmodeR {
        CmdForSector4kerase4bmodeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command for Page Program 3B mode"]
    #[inline(always)]
    pub fn cmd_for_page_program3bmode(&mut self) -> CmdForPageProgram3bmodeW<Spi074Spec> {
        CmdForPageProgram3bmodeW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command for Page Program 4B mode"]
    #[inline(always)]
    pub fn cmd_for_page_program4bmode(&mut self) -> CmdForPageProgram4bmodeW<Spi074Spec> {
        CmdForPageProgram4bmodeW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command for Sector (4K) Erase 3B mode"]
    #[inline(always)]
    pub fn cmd_for_sector4kerase3bmode(&mut self) -> CmdForSector4kerase3bmodeW<Spi074Spec> {
        CmdForSector4kerase3bmodeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Command for Sector (4K) Erase 4B mode"]
    #[inline(always)]
    pub fn cmd_for_sector4kerase4bmode(&mut self) -> CmdForSector4kerase4bmodeW<Spi074Spec> {
        CmdForSector4kerase4bmodeW::new(self, 24)
    }
}
#[doc = "Host Direct Access Commands \\#2 (SPI1 only)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi074Spec;
impl crate::RegisterSpec for Spi074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi074::R`](R) reader structure"]
impl crate::Readable for Spi074Spec {}
#[doc = "`write(|w| ..)` method takes [`spi074::W`](W) writer structure"]
impl crate::Writable for Spi074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI074 to value 0x2120_1202"]
impl crate::Resettable for Spi074Spec {
    const RESET_VALUE: u32 = 0x2120_1202;
}
