#[doc = "Register `SPI0A0` reader"]
pub type R = crate::R<Spi0a0Spec>;
#[doc = "Register `SPI0A0` writer"]
pub type W = crate::W<Spi0a0Spec>;
#[doc = "Enable read/write command filter for CE0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblReadwrCmdFilterForCe0 {
    #[doc = "0: disable filter, all commands are allowed."]
    DisableFilterAllCommandsAreAllowed = 0,
    #[doc = "1: enable filter, only commands defined in \\hlink{SPIR100 $\\sim$ \\hlink{SPIR17C are allowed."]
    EnableFilterOnlyCommandsDefinedInHlinkSpir100SimHlinkSpir17cAreAllowed = 1,
}
impl From<EnblReadwrCmdFilterForCe0> for bool {
    #[inline(always)]
    fn from(variant: EnblReadwrCmdFilterForCe0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblReadwrCmdFilterForCE0` reader - Enable read/write command filter for CE0"]
pub type EnblReadwrCmdFilterForCe0R = crate::BitReader<EnblReadwrCmdFilterForCe0>;
impl EnblReadwrCmdFilterForCe0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblReadwrCmdFilterForCe0 {
        match self . bits { false => EnblReadwrCmdFilterForCe0 :: DisableFilterAllCommandsAreAllowed , true => EnblReadwrCmdFilterForCe0 :: EnableFilterOnlyCommandsDefinedInHlinkSpir100SimHlinkSpir17cAreAllowed , }
    }
    #[doc = "disable filter, all commands are allowed."]
    #[inline(always)]
    pub fn is_disable_filter_all_commands_are_allowed(&self) -> bool {
        *self == EnblReadwrCmdFilterForCe0::DisableFilterAllCommandsAreAllowed
    }
    #[doc = "enable filter, only commands defined in \\hlink{SPIR100 $\\sim$ \\hlink{SPIR17C are allowed."]
    #[inline(always)]
    pub fn is_enable_filter_only_commands_defined_in_hlink_spir100_sim_hlink_spir17c_are_allowed(
        &self,
    ) -> bool {
        * self == EnblReadwrCmdFilterForCe0 :: EnableFilterOnlyCommandsDefinedInHlinkSpir100SimHlinkSpir17cAreAllowed
    }
}
#[doc = "Field `EnblReadwrCmdFilterForCE0` writer - Enable read/write command filter for CE0"]
pub type EnblReadwrCmdFilterForCe0W<'a, REG> = crate::BitWriter<'a, REG, EnblReadwrCmdFilterForCe0>;
impl<'a, REG> EnblReadwrCmdFilterForCe0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable filter, all commands are allowed."]
    #[inline(always)]
    pub fn disable_filter_all_commands_are_allowed(self) -> &'a mut crate::W<REG> {
        self.variant(EnblReadwrCmdFilterForCe0::DisableFilterAllCommandsAreAllowed)
    }
    #[doc = "enable filter, only commands defined in \\hlink{SPIR100 $\\sim$ \\hlink{SPIR17C are allowed."]
    #[inline(always)]
    pub fn enable_filter_only_commands_defined_in_hlink_spir100_sim_hlink_spir17c_are_allowed(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblReadwrCmdFilterForCe0 :: EnableFilterOnlyCommandsDefinedInHlinkSpir100SimHlinkSpir17cAreAllowed)
    }
}
#[doc = "Field `EnblReadwrCmdFilterForCE1` reader - Enable read/write command filter for CE1"]
pub type EnblReadwrCmdFilterForCe1R = crate::BitReader;
#[doc = "Field `EnblReadwrCmdFilterForCE1` writer - Enable read/write command filter for CE1"]
pub type EnblReadwrCmdFilterForCe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Enbl0x03And0x13InFullyQualifiedCmdList` reader - Enable 0x03 and 0x13 in Fully Qualified Command list"]
pub type Enbl0x03and0x13inFullyQualifiedCmdListR = crate::BitReader;
#[doc = "Field `Enbl0x03And0x13InFullyQualifiedCmdList` writer - Enable 0x03 and 0x13 in Fully Qualified Command list"]
pub type Enbl0x03and0x13inFullyQualifiedCmdListW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable read/write command filter for CE0"]
    #[inline(always)]
    pub fn enbl_readwr_cmd_filter_for_ce0(&self) -> EnblReadwrCmdFilterForCe0R {
        EnblReadwrCmdFilterForCe0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable read/write command filter for CE1"]
    #[inline(always)]
    pub fn enbl_readwr_cmd_filter_for_ce1(&self) -> EnblReadwrCmdFilterForCe1R {
        EnblReadwrCmdFilterForCe1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable 0x03 and 0x13 in Fully Qualified Command list"]
    #[inline(always)]
    pub fn enbl0x03and0x13in_fully_qualified_cmd_list(
        &self,
    ) -> Enbl0x03and0x13inFullyQualifiedCmdListR {
        Enbl0x03and0x13inFullyQualifiedCmdListR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable read/write command filter for CE0"]
    #[inline(always)]
    pub fn enbl_readwr_cmd_filter_for_ce0(&mut self) -> EnblReadwrCmdFilterForCe0W<Spi0a0Spec> {
        EnblReadwrCmdFilterForCe0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable read/write command filter for CE1"]
    #[inline(always)]
    pub fn enbl_readwr_cmd_filter_for_ce1(&mut self) -> EnblReadwrCmdFilterForCe1W<Spi0a0Spec> {
        EnblReadwrCmdFilterForCe1W::new(self, 1)
    }
    #[doc = "Bit 31 - Enable 0x03 and 0x13 in Fully Qualified Command list"]
    #[inline(always)]
    pub fn enbl0x03and0x13in_fully_qualified_cmd_list(
        &mut self,
    ) -> Enbl0x03and0x13inFullyQualifiedCmdListW<Spi0a0Spec> {
        Enbl0x03and0x13inFullyQualifiedCmdListW::new(self, 31)
    }
}
#[doc = "Command Filter Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi0a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi0a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi0a0Spec;
impl crate::RegisterSpec for Spi0a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi0a0::R`](R) reader structure"]
impl crate::Readable for Spi0a0Spec {}
#[doc = "`write(|w| ..)` method takes [`spi0a0::W`](W) writer structure"]
impl crate::Writable for Spi0a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI0A0 to value 0x8000_0000"]
impl crate::Resettable for Spi0a0Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
