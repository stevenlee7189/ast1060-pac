#[doc = "Register `FMC150` reader"]
pub type R = crate::R<Fmc150Spec>;
#[doc = "Register `FMC150` writer"]
pub type W = crate::W<Fmc150Spec>;
#[doc = "Field `Cmd1` reader - Command #1"]
pub type Cmd1R = crate::FieldReader;
#[doc = "Field `Cmd1` writer - Command #1"]
pub type Cmd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd2` reader - Command #2"]
pub type Cmd2R = crate::FieldReader;
#[doc = "Field `Cmd2` writer - Command #2"]
pub type Cmd2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd3` reader - Command #3"]
pub type Cmd3R = crate::FieldReader;
#[doc = "Field `Cmd3` writer - Command #3"]
pub type Cmd3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd` reader - 3B/4B Command"]
pub type _3b4bcmdR = crate::BitReader<_3b4bcmd>;
impl _3b4bcmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd {
        match self.bits {
            false => _3b4bcmd::Command321AreFor3bMode,
            true => _3b4bcmd::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd` writer - 3B/4B Command"]
pub type _3b4bcmdW<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd>;
impl<'a, REG> _3b4bcmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd::Command321AreFor4bMode)
    }
}
#[doc = "Command #1 setting\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting {}
#[doc = "Field `Cmd1Setting` reader - Command #1 setting"]
pub type Cmd1settingR = crate::FieldReader<Cmd1setting>;
impl Cmd1settingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting {
        match self.bits {
            0 => Cmd1setting::Disable,
            1 => Cmd1setting::ForRead,
            2 => Cmd1setting::ForWrite,
            3 => Cmd1setting::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting` writer - Command #1 setting"]
pub type Cmd1settingW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting, crate::Safe>;
impl<'a, REG> Cmd1settingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting {}
#[doc = "Field `Cmd2Setting` reader - Command #2 setting"]
pub type Cmd2settingR = crate::FieldReader<Cmd2setting>;
impl Cmd2settingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting {
        match self.bits {
            0 => Cmd2setting::Disable,
            1 => Cmd2setting::ForRead,
            2 => Cmd2setting::ForWrite,
            3 => Cmd2setting::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting` writer - Command #2 setting"]
pub type Cmd2settingW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting, crate::Safe>;
impl<'a, REG> Cmd2settingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting {}
#[doc = "Field `Cmd3Setting` reader - Command #3 setting"]
pub type Cmd3settingR = crate::FieldReader<Cmd3setting>;
impl Cmd3settingR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting {
        match self.bits {
            0 => Cmd3setting::Disable,
            1 => Cmd3setting::ForRead,
            2 => Cmd3setting::ForWrite,
            3 => Cmd3setting::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting` writer - Command #3 setting"]
pub type Cmd3settingW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting, crate::Safe>;
impl<'a, REG> Cmd3settingW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd1(&self) -> Cmd1R {
        Cmd1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd2(&self) -> Cmd2R {
        Cmd2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd3(&self) -> Cmd3R {
        Cmd3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd(&self) -> _3b4bcmdR {
        _3b4bcmdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting(&self) -> Cmd1settingR {
        Cmd1settingR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting(&self) -> Cmd2settingR {
        Cmd2settingR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting(&self) -> Cmd3settingR {
        Cmd3settingR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd1(&mut self) -> Cmd1W<Fmc150Spec> {
        Cmd1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd2(&mut self) -> Cmd2W<Fmc150Spec> {
        Cmd2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd3(&mut self) -> Cmd3W<Fmc150Spec> {
        Cmd3W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd(&mut self) -> _3b4bcmdW<Fmc150Spec> {
        _3b4bcmdW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting(&mut self) -> Cmd1settingW<Fmc150Spec> {
        Cmd1settingW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting(&mut self) -> Cmd2settingW<Fmc150Spec> {
        Cmd2settingW::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting(&mut self) -> Cmd3settingW<Fmc150Spec> {
        Cmd3settingW::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc150::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc150::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc150Spec;
impl crate::RegisterSpec for Fmc150Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc150::R`](R) reader structure"]
impl crate::Readable for Fmc150Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc150::W`](W) writer structure"]
impl crate::Writable for Fmc150Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC150 to value 0x0800_0002"]
impl crate::Resettable for Fmc150Spec {
    const RESET_VALUE: u32 = 0x0800_0002;
}
