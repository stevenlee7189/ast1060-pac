#[doc = "Register `GPIO2AC` reader"]
pub type R = crate::R<Gpio2acSpec>;
#[doc = "Register `GPIO2AC` writer"]
pub type W = crate::W<Gpio2acSpec>;
#[doc = "Field `IndexNumber` reader - Index Number"]
pub type IndexNumberR = crate::FieldReader;
#[doc = "Field `IndexNumber` writer - Index Number"]
pub type IndexNumberW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Index Command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IndexCmd {
    #[doc = "0: Write"]
    Write = 0,
    #[doc = "1: Read"]
    Read = 1,
}
impl From<IndexCmd> for bool {
    #[inline(always)]
    fn from(variant: IndexCmd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IndexCmd` reader - Index Command"]
pub type IndexCmdR = crate::BitReader<IndexCmd>;
impl IndexCmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IndexCmd {
        match self.bits {
            false => IndexCmd::Write,
            true => IndexCmd::Read,
        }
    }
    #[doc = "Write"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == IndexCmd::Write
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == IndexCmd::Read
    }
}
#[doc = "Field `IndexCmd` writer - Index Command"]
pub type IndexCmdW<'a, REG> = crate::BitWriter<'a, REG, IndexCmd>;
impl<'a, REG> IndexCmdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(IndexCmd::Write)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(IndexCmd::Read)
    }
}
#[doc = "Field `IndexType` reader - Index Type"]
pub type IndexTypeR = crate::FieldReader;
#[doc = "Field `IndexType` writer - Index Type"]
pub type IndexTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IndexData` reader - Index Data"]
pub type IndexDataR = crate::FieldReader<u16>;
#[doc = "Field `IndexData` writer - Index Data"]
pub type IndexDataW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:7 - Index Number"]
    #[inline(always)]
    pub fn index_number(&self) -> IndexNumberR {
        IndexNumberR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Index Command"]
    #[inline(always)]
    pub fn index_cmd(&self) -> IndexCmdR {
        IndexCmdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Index Type"]
    #[inline(always)]
    pub fn index_type(&self) -> IndexTypeR {
        IndexTypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Index Data"]
    #[inline(always)]
    pub fn index_data(&self) -> IndexDataR {
        IndexDataR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Index Number"]
    #[inline(always)]
    pub fn index_number(&mut self) -> IndexNumberW<Gpio2acSpec> {
        IndexNumberW::new(self, 0)
    }
    #[doc = "Bit 12 - Index Command"]
    #[inline(always)]
    pub fn index_cmd(&mut self) -> IndexCmdW<Gpio2acSpec> {
        IndexCmdW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Index Type"]
    #[inline(always)]
    pub fn index_type(&mut self) -> IndexTypeW<Gpio2acSpec> {
        IndexTypeW::new(self, 16)
    }
    #[doc = "Bits 20:31 - Index Data"]
    #[inline(always)]
    pub fn index_data(&mut self) -> IndexDataW<Gpio2acSpec> {
        IndexDataW::new(self, 20)
    }
}
#[doc = "GPIO Index Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio2ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio2ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2acSpec;
impl crate::RegisterSpec for Gpio2acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2ac::R`](R) reader structure"]
impl crate::Readable for Gpio2acSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio2ac::W`](W) writer structure"]
impl crate::Writable for Gpio2acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO2AC to value 0"]
impl crate::Resettable for Gpio2acSpec {}
