#[doc = "Register `HACE34` reader"]
pub type R = crate::R<Hace34Spec>;
#[doc = "Register `HACE34` writer"]
pub type W = crate::W<Hace34Spec>;
#[doc = "Field `HashDataPaddingLen` reader - Hash data padding length"]
pub type HashDataPaddingLenR = crate::FieldReader<u32>;
#[doc = "Field `HashDataPaddingLen` writer - Hash data padding length"]
pub type HashDataPaddingLenW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:27 - Hash data padding length"]
    #[inline(always)]
    pub fn hash_data_padding_len(&self) -> HashDataPaddingLenR {
        HashDataPaddingLenR::new(self.bits & 0x0fff_ffff)
    }
    #[doc = "Bits 28:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:27 - Hash data padding length"]
    #[inline(always)]
    pub fn hash_data_padding_len(&mut self) -> HashDataPaddingLenW<Hace34Spec> {
        HashDataPaddingLenW::new(self, 0)
    }
}
#[doc = "Hash Data Padding Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hace34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hace34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hace34Spec;
impl crate::RegisterSpec for Hace34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hace34::R`](R) reader structure"]
impl crate::Readable for Hace34Spec {}
#[doc = "`write(|w| ..)` method takes [`hace34::W`](W) writer structure"]
impl crate::Writable for Hace34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HACE34 to value 0"]
impl crate::Resettable for Hace34Spec {}
