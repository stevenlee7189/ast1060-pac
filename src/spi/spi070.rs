#[doc = "Register `SPI070` reader"]
pub type R = crate::R<Spi070Spec>;
#[doc = "Register `SPI070` writer"]
pub type W = crate::W<Spi070Spec>;
#[doc = "Field `CmdForReadStatus` reader - Command for Read Status"]
pub type CmdForReadStatusR = crate::FieldReader;
#[doc = "Field `CmdForReadStatus` writer - Command for Read Status"]
pub type CmdForReadStatusW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CmdForWrEnbl` reader - Command for Write Enable"]
pub type CmdForWrEnblR = crate::FieldReader;
#[doc = "Field `CmdForWrEnbl` writer - Command for Write Enable"]
pub type CmdForWrEnblW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SPIFlashStsRegRegdebug` reader - SPI flash status register regdebug"]
pub type SpiflashStsRegRegdebugR = crate::FieldReader;
#[doc = "Host access SPI1 CE selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostAccessSpi1cesel {
    #[doc = "0: CE0"]
    Ce0 = 0,
    #[doc = "1: CE1"]
    Ce1 = 1,
}
impl From<HostAccessSpi1cesel> for bool {
    #[inline(always)]
    fn from(variant: HostAccessSpi1cesel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HostAccessSPI1CESel` reader - Host access SPI1 CE selection"]
pub type HostAccessSpi1ceselR = crate::BitReader<HostAccessSpi1cesel>;
impl HostAccessSpi1ceselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostAccessSpi1cesel {
        match self.bits {
            false => HostAccessSpi1cesel::Ce0,
            true => HostAccessSpi1cesel::Ce1,
        }
    }
    #[doc = "CE0"]
    #[inline(always)]
    pub fn is_ce0(&self) -> bool {
        *self == HostAccessSpi1cesel::Ce0
    }
    #[doc = "CE1"]
    #[inline(always)]
    pub fn is_ce1(&self) -> bool {
        *self == HostAccessSpi1cesel::Ce1
    }
}
#[doc = "Field `HostAccessSPI1CESel` writer - Host access SPI1 CE selection"]
pub type HostAccessSpi1ceselW<'a, REG> = crate::BitWriter<'a, REG, HostAccessSpi1cesel>;
impl<'a, REG> HostAccessSpi1ceselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CE0"]
    #[inline(always)]
    pub fn ce0(self) -> &'a mut crate::W<REG> {
        self.variant(HostAccessSpi1cesel::Ce0)
    }
    #[doc = "CE1"]
    #[inline(always)]
    pub fn ce1(self) -> &'a mut crate::W<REG> {
        self.variant(HostAccessSpi1cesel::Ce1)
    }
}
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Direct access command execution step regdebug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DirectAccessCmdExecutionStepRegdebug {
    #[doc = "0: Idle"]
    Idle = 0,
}
impl From<DirectAccessCmdExecutionStepRegdebug> for u8 {
    #[inline(always)]
    fn from(variant: DirectAccessCmdExecutionStepRegdebug) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DirectAccessCmdExecutionStepRegdebug {
    type Ux = u8;
}
impl crate::IsEnum for DirectAccessCmdExecutionStepRegdebug {}
#[doc = "Field `DirectAccessCmdExecutionStepRegdebug` reader - Direct access command execution step regdebug"]
pub type DirectAccessCmdExecutionStepRegdebugR =
    crate::FieldReader<DirectAccessCmdExecutionStepRegdebug>;
impl DirectAccessCmdExecutionStepRegdebugR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DirectAccessCmdExecutionStepRegdebug> {
        match self.bits {
            0 => Some(DirectAccessCmdExecutionStepRegdebug::Idle),
            _ => None,
        }
    }
    #[doc = "Idle"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DirectAccessCmdExecutionStepRegdebug::Idle
    }
}
#[doc = "Enable command encoding for direct access\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnblCmdEncodingForDirAccess {
    #[doc = "0: Disable, host should manage the controller to issue the required commands."]
    DisableHostShouldManageTheControllerToIssueTheRequiredCommands = 0,
    #[doc = "1: Enable, support encoded page program and erase commands. Refer Section \\ref{SECB:SPIDMA."]
    EnableSupportEncodedPageProgramAndEraseCommandsReferSectionRefSecbspidma = 1,
}
impl From<EnblCmdEncodingForDirAccess> for bool {
    #[inline(always)]
    fn from(variant: EnblCmdEncodingForDirAccess) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EnblCmdEncodingForDirAccess` reader - Enable command encoding for direct access"]
pub type EnblCmdEncodingForDirAccessR = crate::BitReader<EnblCmdEncodingForDirAccess>;
impl EnblCmdEncodingForDirAccessR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnblCmdEncodingForDirAccess {
        match self . bits { false => EnblCmdEncodingForDirAccess :: DisableHostShouldManageTheControllerToIssueTheRequiredCommands , true => EnblCmdEncodingForDirAccess :: EnableSupportEncodedPageProgramAndEraseCommandsReferSectionRefSecbspidma , }
    }
    #[doc = "Disable, host should manage the controller to issue the required commands."]
    #[inline(always)]
    pub fn is_disable_host_should_manage_the_controller_to_issue_the_required_commands(
        &self,
    ) -> bool {
        * self == EnblCmdEncodingForDirAccess :: DisableHostShouldManageTheControllerToIssueTheRequiredCommands
    }
    #[doc = "Enable, support encoded page program and erase commands. Refer Section \\ref{SECB:SPIDMA."]
    #[inline(always)]
    pub fn is_enable_support_encoded_page_program_and_erase_commands_refer_section_ref_secbspidma(
        &self,
    ) -> bool {
        * self == EnblCmdEncodingForDirAccess :: EnableSupportEncodedPageProgramAndEraseCommandsReferSectionRefSecbspidma
    }
}
#[doc = "Field `EnblCmdEncodingForDirAccess` writer - Enable command encoding for direct access"]
pub type EnblCmdEncodingForDirAccessW<'a, REG> =
    crate::BitWriter<'a, REG, EnblCmdEncodingForDirAccess>;
impl<'a, REG> EnblCmdEncodingForDirAccessW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable, host should manage the controller to issue the required commands."]
    #[inline(always)]
    pub fn disable_host_should_manage_the_controller_to_issue_the_required_commands(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblCmdEncodingForDirAccess :: DisableHostShouldManageTheControllerToIssueTheRequiredCommands)
    }
    #[doc = "Enable, support encoded page program and erase commands. Refer Section \\ref{SECB:SPIDMA."]
    #[inline(always)]
    pub fn enable_support_encoded_page_program_and_erase_commands_refer_section_ref_secbspidma(
        self,
    ) -> &'a mut crate::W<REG> {
        self . variant (EnblCmdEncodingForDirAccess :: EnableSupportEncodedPageProgramAndEraseCommandsReferSectionRefSecbspidma)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command for Read Status"]
    #[inline(always)]
    pub fn cmd_for_read_status(&self) -> CmdForReadStatusR {
        CmdForReadStatusR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command for Write Enable"]
    #[inline(always)]
    pub fn cmd_for_wr_enbl(&self) -> CmdForWrEnblR {
        CmdForWrEnblR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SPI flash status register regdebug"]
    #[inline(always)]
    pub fn spiflash_sts_reg_regdebug(&self) -> SpiflashStsRegRegdebugR {
        SpiflashStsRegRegdebugR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Host access SPI1 CE selection"]
    #[inline(always)]
    pub fn host_access_spi1cesel(&self) -> HostAccessSpi1ceselR {
        HostAccessSpi1ceselR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - Direct access command execution step regdebug"]
    #[inline(always)]
    pub fn direct_access_cmd_execution_step_regdebug(
        &self,
    ) -> DirectAccessCmdExecutionStepRegdebugR {
        DirectAccessCmdExecutionStepRegdebugR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - Enable command encoding for direct access"]
    #[inline(always)]
    pub fn enbl_cmd_encoding_for_dir_access(&self) -> EnblCmdEncodingForDirAccessR {
        EnblCmdEncodingForDirAccessR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command for Read Status"]
    #[inline(always)]
    pub fn cmd_for_read_status(&mut self) -> CmdForReadStatusW<Spi070Spec> {
        CmdForReadStatusW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command for Write Enable"]
    #[inline(always)]
    pub fn cmd_for_wr_enbl(&mut self) -> CmdForWrEnblW<Spi070Spec> {
        CmdForWrEnblW::new(self, 8)
    }
    #[doc = "Bit 24 - Host access SPI1 CE selection"]
    #[inline(always)]
    pub fn host_access_spi1cesel(&mut self) -> HostAccessSpi1ceselW<Spi070Spec> {
        HostAccessSpi1ceselW::new(self, 24)
    }
    #[doc = "Bit 31 - Enable command encoding for direct access"]
    #[inline(always)]
    pub fn enbl_cmd_encoding_for_dir_access(&mut self) -> EnblCmdEncodingForDirAccessW<Spi070Spec> {
        EnblCmdEncodingForDirAccessW::new(self, 31)
    }
}
#[doc = "Host Direct Access Commands \\#1 (SPI1 only)\n\nYou can [`read`](crate::Reg::read) this register and get [`spi070::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi070::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi070Spec;
impl crate::RegisterSpec for Spi070Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi070::R`](R) reader structure"]
impl crate::Readable for Spi070Spec {}
#[doc = "`write(|w| ..)` method takes [`spi070::W`](W) writer structure"]
impl crate::Writable for Spi070Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI070 to value 0x8000_0605"]
impl crate::Resettable for Spi070Spec {
    const RESET_VALUE: u32 = 0x8000_0605;
}
