#[doc = "Register `FMC15C` reader"]
pub type R = crate::R<Fmc15cSpec>;
#[doc = "Register `FMC15C` writer"]
pub type W = crate::W<Fmc15cSpec>;
#[doc = "Field `Cmd13` reader - Command #1"]
pub type Cmd13R = crate::FieldReader;
#[doc = "Field `Cmd13` writer - Command #1"]
pub type Cmd13W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd23` reader - Command #2"]
pub type Cmd23R = crate::FieldReader;
#[doc = "Field `Cmd23` writer - Command #2"]
pub type Cmd23W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd33` reader - Command #3"]
pub type Cmd33R = crate::FieldReader;
#[doc = "Field `Cmd33` writer - Command #3"]
pub type Cmd33W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd3 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd3> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd3` reader - 3B/4B Command"]
pub type _3b4bcmd3R = crate::BitReader<_3b4bcmd3>;
impl _3b4bcmd3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd3 {
        match self.bits {
            false => _3b4bcmd3::Command321AreFor3bMode,
            true => _3b4bcmd3::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd3::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd3::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd3` writer - 3B/4B Command"]
pub type _3b4bcmd3W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd3>;
impl<'a, REG> _3b4bcmd3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd3::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd3::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting3 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting3> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting3 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting3 {}
#[doc = "Field `Cmd1Setting3` reader - Command #1 setting"]
pub type Cmd1setting3R = crate::FieldReader<Cmd1setting3>;
impl Cmd1setting3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting3 {
        match self.bits {
            0 => Cmd1setting3::Disable,
            1 => Cmd1setting3::ForRead,
            2 => Cmd1setting3::ForWrite,
            3 => Cmd1setting3::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting3::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting3::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting3::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting3::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting3` writer - Command #1 setting"]
pub type Cmd1setting3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting3, crate::Safe>;
impl<'a, REG> Cmd1setting3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting3::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting3::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting3::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting3::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting3 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting3> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting3 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting3 {}
#[doc = "Field `Cmd2Setting3` reader - Command #2 setting"]
pub type Cmd2setting3R = crate::FieldReader<Cmd2setting3>;
impl Cmd2setting3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting3 {
        match self.bits {
            0 => Cmd2setting3::Disable,
            1 => Cmd2setting3::ForRead,
            2 => Cmd2setting3::ForWrite,
            3 => Cmd2setting3::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting3::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting3::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting3::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting3::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting3` writer - Command #2 setting"]
pub type Cmd2setting3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting3, crate::Safe>;
impl<'a, REG> Cmd2setting3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting3::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting3::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting3::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting3::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting3 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting3> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting3) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting3 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting3 {}
#[doc = "Field `Cmd3Setting3` reader - Command #3 setting"]
pub type Cmd3setting3R = crate::FieldReader<Cmd3setting3>;
impl Cmd3setting3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting3 {
        match self.bits {
            0 => Cmd3setting3::Disable,
            1 => Cmd3setting3::ForRead,
            2 => Cmd3setting3::ForWrite,
            3 => Cmd3setting3::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting3::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting3::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting3::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting3::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting3` writer - Command #3 setting"]
pub type Cmd3setting3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting3, crate::Safe>;
impl<'a, REG> Cmd3setting3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting3::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting3::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting3::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting3::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd13(&self) -> Cmd13R {
        Cmd13R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd23(&self) -> Cmd23R {
        Cmd23R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd33(&self) -> Cmd33R {
        Cmd33R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd3(&self) -> _3b4bcmd3R {
        _3b4bcmd3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting3(&self) -> Cmd1setting3R {
        Cmd1setting3R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting3(&self) -> Cmd2setting3R {
        Cmd2setting3R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting3(&self) -> Cmd3setting3R {
        Cmd3setting3R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd13(&mut self) -> Cmd13W<Fmc15cSpec> {
        Cmd13W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd23(&mut self) -> Cmd23W<Fmc15cSpec> {
        Cmd23W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd33(&mut self) -> Cmd33W<Fmc15cSpec> {
        Cmd33W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd3(&mut self) -> _3b4bcmd3W<Fmc15cSpec> {
        _3b4bcmd3W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting3(&mut self) -> Cmd1setting3W<Fmc15cSpec> {
        Cmd1setting3W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting3(&mut self) -> Cmd2setting3W<Fmc15cSpec> {
        Cmd2setting3W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting3(&mut self) -> Cmd3setting3W<Fmc15cSpec> {
        Cmd3setting3W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc15c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc15c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc15cSpec;
impl crate::RegisterSpec for Fmc15cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc15c::R`](R) reader structure"]
impl crate::Readable for Fmc15cSpec {}
#[doc = "`write(|w| ..)` method takes [`fmc15c::W`](W) writer structure"]
impl crate::Writable for Fmc15cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC15C to value 0"]
impl crate::Resettable for Fmc15cSpec {}
