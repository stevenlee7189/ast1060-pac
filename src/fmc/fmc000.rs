#[doc = "Register `FMC000` reader"]
pub type R = crate::R<Fmc000Spec>;
#[doc = "Register `FMC000` writer"]
pub type W = crate::W<Fmc000Spec>;
#[doc = "CE0 flash type selection\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ce0flashTypeSel {
    #[doc = "2: Select SPI flash type"]
    SelectSpiFlashType = 2,
}
impl From<Ce0flashTypeSel> for u8 {
    #[inline(always)]
    fn from(variant: Ce0flashTypeSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ce0flashTypeSel {
    type Ux = u8;
}
impl crate::IsEnum for Ce0flashTypeSel {}
#[doc = "Field `CE0FlashTypeSel` reader - CE0 flash type selection"]
pub type Ce0flashTypeSelR = crate::FieldReader<Ce0flashTypeSel>;
impl Ce0flashTypeSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ce0flashTypeSel> {
        match self.bits {
            2 => Some(Ce0flashTypeSel::SelectSpiFlashType),
            _ => None,
        }
    }
    #[doc = "Select SPI flash type"]
    #[inline(always)]
    pub fn is_select_spi_flash_type(&self) -> bool {
        *self == Ce0flashTypeSel::SelectSpiFlashType
    }
}
#[doc = "Field `CE1FlashTypeSel` reader - CE1 flash type selection"]
pub type Ce1flashTypeSelR = crate::FieldReader;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader<u16>;
#[doc = "Enable CE0 default write type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblCe0defaultWrType {
    #[doc = "0: CEx is default at write disable mode"]
    CexIsDefaultAtWriteDisableMode = 0,
    #[doc = "1: CEx is default at write enable mode"]
    CexIsDefaultAtWriteEnableMode = 1,
}
impl From<EnblCe0defaultWrType> for bool {
    #[inline(always)]
    fn from(variant: EnblCe0defaultWrType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblCE0DefaultWrType` reader - Enable CE0 default write type"]
pub type EnblCe0defaultWrTypeR = crate::BitReader<EnblCe0defaultWrType>;
impl EnblCe0defaultWrTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblCe0defaultWrType {
        match self.bits {
            false => EnblCe0defaultWrType::CexIsDefaultAtWriteDisableMode,
            true => EnblCe0defaultWrType::CexIsDefaultAtWriteEnableMode,
        }
    }
    #[doc = "CEx is default at write disable mode"]
    #[inline(always)]
    pub fn is_cex_is_default_at_write_disable_mode(&self) -> bool {
        *self == EnblCe0defaultWrType::CexIsDefaultAtWriteDisableMode
    }
    #[doc = "CEx is default at write enable mode"]
    #[inline(always)]
    pub fn is_cex_is_default_at_write_enable_mode(&self) -> bool {
        *self == EnblCe0defaultWrType::CexIsDefaultAtWriteEnableMode
    }
}
#[doc = "Field `EnblCE0DefaultWrType` writer - Enable CE0 default write type"]
pub type EnblCe0defaultWrTypeW<'a, REG> = crate::BitWriter<'a, REG, EnblCe0defaultWrType>;
impl<'a, REG> EnblCe0defaultWrTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CEx is default at write disable mode"]
    #[inline(always)]
    pub fn cex_is_default_at_write_disable_mode(self) -> &'a mut crate::W<REG> {
        self.variant(EnblCe0defaultWrType::CexIsDefaultAtWriteDisableMode)
    }
    #[doc = "CEx is default at write enable mode"]
    #[inline(always)]
    pub fn cex_is_default_at_write_enable_mode(self) -> &'a mut crate::W<REG> {
        self.variant(EnblCe0defaultWrType::CexIsDefaultAtWriteEnableMode)
    }
}
#[doc = "Field `EnblCE1DefaultWrType` reader - Enable CE1 default write type"]
pub type EnblCe1defaultWrTypeR = crate::BitReader;
#[doc = "Field `EnblCE1DefaultWrType` writer - Enable CE1 default write type"]
pub type EnblCe1defaultWrTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - CE0 flash type selection"]
    #[inline(always)]
    pub fn ce0flash_type_sel(&self) -> Ce0flashTypeSelR {
        Ce0flashTypeSelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - CE1 flash type selection"]
    #[inline(always)]
    pub fn ce1flash_type_sel(&self) -> Ce1flashTypeSelR {
        Ce1flashTypeSelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:15 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Enable CE0 default write type"]
    #[inline(always)]
    pub fn enbl_ce0default_wr_type(&self) -> EnblCe0defaultWrTypeR {
        EnblCe0defaultWrTypeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable CE1 default write type"]
    #[inline(always)]
    pub fn enbl_ce1default_wr_type(&self) -> EnblCe1defaultWrTypeR {
        EnblCe1defaultWrTypeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 18) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 16 - Enable CE0 default write type"]
    #[inline(always)]
    pub fn enbl_ce0default_wr_type(&mut self) -> EnblCe0defaultWrTypeW<Fmc000Spec> {
        EnblCe0defaultWrTypeW::new(self, 16)
    }
    #[doc = "Bit 17 - Enable CE1 default write type"]
    #[inline(always)]
    pub fn enbl_ce1default_wr_type(&mut self) -> EnblCe1defaultWrTypeW<Fmc000Spec> {
        EnblCe1defaultWrTypeW::new(self, 17)
    }
}
#[doc = "CE Type Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc000::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc000::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc000Spec;
impl crate::RegisterSpec for Fmc000Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc000::R`](R) reader structure"]
impl crate::Readable for Fmc000Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc000::W`](W) writer structure"]
impl crate::Writable for Fmc000Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC000 to value 0x2a"]
impl crate::Resettable for Fmc000Spec {
    const RESET_VALUE: u32 = 0x2a;
}
