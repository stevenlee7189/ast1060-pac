#[doc = "Register `FMC160` reader"]
pub type R = crate::R<Fmc160Spec>;
#[doc = "Register `FMC160` writer"]
pub type W = crate::W<Fmc160Spec>;
#[doc = "Field `Cmd14` reader - Command #1"]
pub type Cmd14R = crate::FieldReader;
#[doc = "Field `Cmd14` writer - Command #1"]
pub type Cmd14W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd24` reader - Command #2"]
pub type Cmd24R = crate::FieldReader;
#[doc = "Field `Cmd24` writer - Command #2"]
pub type Cmd24W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Cmd34` reader - Command #3"]
pub type Cmd34R = crate::FieldReader;
#[doc = "Field `Cmd34` writer - Command #3"]
pub type Cmd34W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "3B/4B Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum _3b4bcmd4 {
    #[doc = "0: Command \\#3/\\#2/\\#1 are for 3B mode."]
    Command321AreFor3bMode = 0,
    #[doc = "1: Command \\#3/\\#2/\\#1 are for 4B mode."]
    Command321AreFor4bMode = 1,
}
impl From<_3b4bcmd4> for bool {
    #[inline(always)]
    fn from(variant: _3b4bcmd4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `3B4BCmd4` reader - 3B/4B Command"]
pub type _3b4bcmd4R = crate::BitReader<_3b4bcmd4>;
impl _3b4bcmd4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> _3b4bcmd4 {
        match self.bits {
            false => _3b4bcmd4::Command321AreFor3bMode,
            true => _3b4bcmd4::Command321AreFor4bMode,
        }
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_3b_mode(&self) -> bool {
        *self == _3b4bcmd4::Command321AreFor3bMode
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn is_command_321_are_for_4b_mode(&self) -> bool {
        *self == _3b4bcmd4::Command321AreFor4bMode
    }
}
#[doc = "Field `3B4BCmd4` writer - 3B/4B Command"]
pub type _3b4bcmd4W<'a, REG> = crate::BitWriter<'a, REG, _3b4bcmd4>;
impl<'a, REG> _3b4bcmd4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command \\#3/\\#2/\\#1 are for 3B mode."]
    #[inline(always)]
    pub fn command_321_are_for_3b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd4::Command321AreFor3bMode)
    }
    #[doc = "Command \\#3/\\#2/\\#1 are for 4B mode."]
    #[inline(always)]
    pub fn command_321_are_for_4b_mode(self) -> &'a mut crate::W<REG> {
        self.variant(_3b4bcmd4::Command321AreFor4bMode)
    }
}
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Command #1 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd1setting4 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd1setting4> for u8 {
    #[inline(always)]
    fn from(variant: Cmd1setting4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd1setting4 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd1setting4 {}
#[doc = "Field `Cmd1Setting4` reader - Command #1 setting"]
pub type Cmd1setting4R = crate::FieldReader<Cmd1setting4>;
impl Cmd1setting4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd1setting4 {
        match self.bits {
            0 => Cmd1setting4::Disable,
            1 => Cmd1setting4::ForRead,
            2 => Cmd1setting4::ForWrite,
            3 => Cmd1setting4::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd1setting4::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd1setting4::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd1setting4::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd1setting4::ForReadAndWrite
    }
}
#[doc = "Field `Cmd1Setting4` writer - Command #1 setting"]
pub type Cmd1setting4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd1setting4, crate::Safe>;
impl<'a, REG> Cmd1setting4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting4::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting4::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting4::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd1setting4::ForReadAndWrite)
    }
}
#[doc = "Command #2 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd2setting4 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd2setting4> for u8 {
    #[inline(always)]
    fn from(variant: Cmd2setting4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd2setting4 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd2setting4 {}
#[doc = "Field `Cmd2Setting4` reader - Command #2 setting"]
pub type Cmd2setting4R = crate::FieldReader<Cmd2setting4>;
impl Cmd2setting4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd2setting4 {
        match self.bits {
            0 => Cmd2setting4::Disable,
            1 => Cmd2setting4::ForRead,
            2 => Cmd2setting4::ForWrite,
            3 => Cmd2setting4::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd2setting4::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd2setting4::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd2setting4::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd2setting4::ForReadAndWrite
    }
}
#[doc = "Field `Cmd2Setting4` writer - Command #2 setting"]
pub type Cmd2setting4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd2setting4, crate::Safe>;
impl<'a, REG> Cmd2setting4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting4::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting4::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting4::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd2setting4::ForReadAndWrite)
    }
}
#[doc = "Command #3 setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd3setting4 {
    #[doc = "0: Disable."]
    Disable = 0,
    #[doc = "1: For Read."]
    ForRead = 1,
    #[doc = "2: For Write."]
    ForWrite = 2,
    #[doc = "3: For Read and Write."]
    ForReadAndWrite = 3,
}
impl From<Cmd3setting4> for u8 {
    #[inline(always)]
    fn from(variant: Cmd3setting4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cmd3setting4 {
    type Ux = u8;
}
impl crate::IsEnum for Cmd3setting4 {}
#[doc = "Field `Cmd3Setting4` reader - Command #3 setting"]
pub type Cmd3setting4R = crate::FieldReader<Cmd3setting4>;
impl Cmd3setting4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cmd3setting4 {
        match self.bits {
            0 => Cmd3setting4::Disable,
            1 => Cmd3setting4::ForRead,
            2 => Cmd3setting4::ForWrite,
            3 => Cmd3setting4::ForReadAndWrite,
            _ => unreachable!(),
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Cmd3setting4::Disable
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn is_for_read(&self) -> bool {
        *self == Cmd3setting4::ForRead
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn is_for_write(&self) -> bool {
        *self == Cmd3setting4::ForWrite
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn is_for_read_and_write(&self) -> bool {
        *self == Cmd3setting4::ForReadAndWrite
    }
}
#[doc = "Field `Cmd3Setting4` writer - Command #3 setting"]
pub type Cmd3setting4W<'a, REG> = crate::FieldWriter<'a, REG, 2, Cmd3setting4, crate::Safe>;
impl<'a, REG> Cmd3setting4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting4::Disable)
    }
    #[doc = "For Read."]
    #[inline(always)]
    pub fn for_read(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting4::ForRead)
    }
    #[doc = "For Write."]
    #[inline(always)]
    pub fn for_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting4::ForWrite)
    }
    #[doc = "For Read and Write."]
    #[inline(always)]
    pub fn for_read_and_write(self) -> &'a mut crate::W<REG> {
        self.variant(Cmd3setting4::ForReadAndWrite)
    }
}
impl R {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd14(&self) -> Cmd14R {
        Cmd14R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd24(&self) -> Cmd24R {
        Cmd24R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd34(&self) -> Cmd34R {
        Cmd34R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd4(&self) -> _3b4bcmd4R {
        _3b4bcmd4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting4(&self) -> Cmd1setting4R {
        Cmd1setting4R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting4(&self) -> Cmd2setting4R {
        Cmd2setting4R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting4(&self) -> Cmd3setting4R {
        Cmd3setting4R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Command #1"]
    #[inline(always)]
    pub fn cmd14(&mut self) -> Cmd14W<Fmc160Spec> {
        Cmd14W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Command #2"]
    #[inline(always)]
    pub fn cmd24(&mut self) -> Cmd24W<Fmc160Spec> {
        Cmd24W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Command #3"]
    #[inline(always)]
    pub fn cmd34(&mut self) -> Cmd34W<Fmc160Spec> {
        Cmd34W::new(self, 16)
    }
    #[doc = "Bit 24 - 3B/4B Command"]
    #[inline(always)]
    pub fn _3b4bcmd4(&mut self) -> _3b4bcmd4W<Fmc160Spec> {
        _3b4bcmd4W::new(self, 24)
    }
    #[doc = "Bits 26:27 - Command #1 setting"]
    #[inline(always)]
    pub fn cmd1setting4(&mut self) -> Cmd1setting4W<Fmc160Spec> {
        Cmd1setting4W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Command #2 setting"]
    #[inline(always)]
    pub fn cmd2setting4(&mut self) -> Cmd2setting4W<Fmc160Spec> {
        Cmd2setting4W::new(self, 28)
    }
    #[doc = "Bits 30:31 - Command #3 setting"]
    #[inline(always)]
    pub fn cmd3setting4(&mut self) -> Cmd3setting4W<Fmc160Spec> {
        Cmd3setting4W::new(self, 30)
    }
}
#[doc = "Address Qualified Command\n\nYou can [`read`](crate::Reg::read) this register and get [`fmc160::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmc160::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmc160Spec;
impl crate::RegisterSpec for Fmc160Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc160::R`](R) reader structure"]
impl crate::Readable for Fmc160Spec {}
#[doc = "`write(|w| ..)` method takes [`fmc160::W`](W) writer structure"]
impl crate::Writable for Fmc160Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FMC160 to value 0"]
impl crate::Resettable for Fmc160Spec {}
