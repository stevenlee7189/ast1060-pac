#[doc = "Register `SCU4BC` reader"]
pub type R = crate::R<Scu4bcSpec>;
#[doc = "Register `SCU4BC` writer"]
pub type W = crate::W<Scu4bcSpec>;
#[doc = "Field `Reserved2` reader - Reserved, must keep at 0x0"]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `Reserved2` writer - Reserved, must keep at 0x0"]
pub type Reserved2W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `EnblJTAGMaster1TRSTNFnPin` reader - Enable JTAG Master #1 TRSTN function pin"]
pub type EnblJtagmaster1trstnfnPinR = crate::BitReader;
#[doc = "Field `EnblJTAGMaster1TRSTNFnPin` writer - Enable JTAG Master #1 TRSTN function pin"]
pub type EnblJtagmaster1trstnfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblJTAGMaster1TCKFnPin` reader - Enable JTAG Master #1 TCK function pin"]
pub type EnblJtagmaster1tckfnPinR = crate::BitReader;
#[doc = "Field `EnblJTAGMaster1TCKFnPin` writer - Enable JTAG Master #1 TCK function pin"]
pub type EnblJtagmaster1tckfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblJTAGMaster1TDIFnPin` reader - Enable JTAG Master #1 TDI function pin"]
pub type EnblJtagmaster1tdifnPinR = crate::BitReader;
#[doc = "Field `EnblJTAGMaster1TDIFnPin` writer - Enable JTAG Master #1 TDI function pin"]
pub type EnblJtagmaster1tdifnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblJTAGMaster1TDOFnPin` reader - Enable JTAG Master #1 TDO function pin"]
pub type EnblJtagmaster1tdofnPinR = crate::BitReader;
#[doc = "Field `EnblJTAGMaster1TDOFnPin` writer - Enable JTAG Master #1 TDO function pin"]
pub type EnblJtagmaster1tdofnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblJTAGMaster1TMSFnPin` reader - Enable JTAG Master #1 TMS function pin"]
pub type EnblJtagmaster1tmsfnPinR = crate::BitReader;
#[doc = "Field `EnblJTAGMaster1TMSFnPin` writer - Enable JTAG Master #1 TMS function pin"]
pub type EnblJtagmaster1tmsfnPinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:24 - Reserved, must keep at 0x0"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(self.bits & 0x01ff_ffff)
    }
    #[doc = "Bit 25 - Enable JTAG Master #1 TRSTN function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1trstnfn_pin(&self) -> EnblJtagmaster1trstnfnPinR {
        EnblJtagmaster1trstnfnPinR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable JTAG Master #1 TCK function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tckfn_pin(&self) -> EnblJtagmaster1tckfnPinR {
        EnblJtagmaster1tckfnPinR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable JTAG Master #1 TDI function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tdifn_pin(&self) -> EnblJtagmaster1tdifnPinR {
        EnblJtagmaster1tdifnPinR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable JTAG Master #1 TDO function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tdofn_pin(&self) -> EnblJtagmaster1tdofnPinR {
        EnblJtagmaster1tdofnPinR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Enable JTAG Master #1 TMS function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tmsfn_pin(&self) -> EnblJtagmaster1tmsfnPinR {
        EnblJtagmaster1tmsfnPinR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:24 - Reserved, must keep at 0x0"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<Scu4bcSpec> {
        Reserved2W::new(self, 0)
    }
    #[doc = "Bit 25 - Enable JTAG Master #1 TRSTN function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1trstnfn_pin(&mut self) -> EnblJtagmaster1trstnfnPinW<Scu4bcSpec> {
        EnblJtagmaster1trstnfnPinW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable JTAG Master #1 TCK function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tckfn_pin(&mut self) -> EnblJtagmaster1tckfnPinW<Scu4bcSpec> {
        EnblJtagmaster1tckfnPinW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable JTAG Master #1 TDI function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tdifn_pin(&mut self) -> EnblJtagmaster1tdifnPinW<Scu4bcSpec> {
        EnblJtagmaster1tdifnPinW::new(self, 27)
    }
    #[doc = "Bit 28 - Enable JTAG Master #1 TDO function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tdofn_pin(&mut self) -> EnblJtagmaster1tdofnPinW<Scu4bcSpec> {
        EnblJtagmaster1tdofnPinW::new(self, 28)
    }
    #[doc = "Bit 29 - Enable JTAG Master #1 TMS function pin"]
    #[inline(always)]
    pub fn enbl_jtagmaster1tmsfn_pin(&mut self) -> EnblJtagmaster1tmsfnPinW<Scu4bcSpec> {
        EnblJtagmaster1tmsfnPinW::new(self, 29)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Scu4bcSpec> {
        Reserved1W::new(self, 30)
    }
}
#[doc = "Multi-function Pin Control \\#16\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4bcSpec;
impl crate::RegisterSpec for Scu4bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4bc::R`](R) reader structure"]
impl crate::Readable for Scu4bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu4bc::W`](W) writer structure"]
impl crate::Writable for Scu4bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4BC to value 0"]
impl crate::Resettable for Scu4bcSpec {}
