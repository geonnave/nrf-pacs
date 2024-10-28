#[doc = "Register `ROSC3` reader"]
pub type R = crate::R<Rosc3Spec>;
#[doc = "Field `ROSC3` reader - Sample count for ring oscillator 3"]
pub type Rosc3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Sample count for ring oscillator 3"]
    #[inline(always)]
    pub fn rosc3(&self) -> Rosc3R {
        Rosc3R::new(self.bits)
    }
}
#[doc = "Sample count for ring oscillator 3\n\nYou can [`read`](crate::Reg::read) this register and get [`rosc3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rosc3Spec;
impl crate::RegisterSpec for Rosc3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rosc3::R`](R) reader structure"]
impl crate::Readable for Rosc3Spec {}
#[doc = "`reset()` method sets ROSC3 to value 0xffff_ffff"]
impl crate::Resettable for Rosc3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
