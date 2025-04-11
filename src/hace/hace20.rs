#[doc = "Register `HACE20` reader"]
pub type R = crate::R<Hace20Spec>;
#[doc = "Register `HACE20` writer"]
pub type W = crate::W<Hace20Spec>;
#[doc = "Field `Reserved01` reader - Reserved (0)"]
pub type Reserved01R = crate::FieldReader;
#[doc = "Field `BaseAddrOfHashSrcData300` reader - Base address of hash source data\\[30:0\\] (byte aligned)"]
pub type BaseAddrOfHashSrcData300R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfHashSrcData300` writer - Base address of hash source data\\[30:0\\] (byte aligned)"]
pub type BaseAddrOfHashSrcData300W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `BaseAddrOfSgListForHashSrcData303` reader - Base address of scatter-gather list for hash source data\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfSgListForHashSrcData303R = crate::FieldReader<u32>;
#[doc = "Field `BaseAddrOfSgListForHashSrcData303` writer - Base address of scatter-gather list for hash source data\\[30:3\\] (8-byte aligned)"]
pub type BaseAddrOfSgListForHashSrcData303W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::BitReader;
#[doc = "Field `Reserved02` reader - Reserved (0)"]
pub type Reserved02R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved01(&self) -> Reserved01R {
        Reserved01R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 0:30 - Base address of hash source data\\[30:0\\] (byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_hash_src_data300(&self) -> BaseAddrOfHashSrcData300R {
        BaseAddrOfHashSrcData300R::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bits 3:30 - Base address of scatter-gather list for hash source data\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_sg_list_for_hash_src_data303(&self) -> BaseAddrOfSgListForHashSrcData303R {
        BaseAddrOfSgListForHashSrcData303R::new((self.bits >> 3) & 0x0fff_ffff)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved02(&self) -> Reserved02R {
        Reserved02R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - Base address of hash source data\\[30:0\\] (byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_hash_src_data300(&mut self) -> BaseAddrOfHashSrcData300W<Hace20Spec> {
        BaseAddrOfHashSrcData300W::new(self, 0)
    }
    #[doc = "Bits 3:30 - Base address of scatter-gather list for hash source data\\[30:3\\] (8-byte aligned)"]
    #[inline(always)]
    pub fn base_addr_of_sg_list_for_hash_src_data303(
        &mut self,
    ) -> BaseAddrOfSgListForHashSrcData303W<Hace20Spec> {
        BaseAddrOfSgListForHashSrcData303W::new(self, 3)
    }
}
#[doc = "Hash Data Source Base Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace20Spec;
impl crate::RegisterSpec for Hace20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace20::R`](R) reader structure"]
impl crate::Readable for Hace20Spec {}
#[doc = "`write(|w| ..)` method takes [`hace20::W`](W) writer structure"]
impl crate::Writable for Hace20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE20 to value 0"]
impl crate::Resettable for Hace20Spec {}
