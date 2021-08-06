#[doc = "Register `EVENTS_RXSTARTED` reader"]
pub struct R(crate::R<EVENTS_RXSTARTED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVENTS_RXSTARTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVENTS_RXSTARTED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVENTS_RXSTARTED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVENTS_RXSTARTED` writer"]
pub struct W(crate::W<EVENTS_RXSTARTED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVENTS_RXSTARTED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<EVENTS_RXSTARTED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVENTS_RXSTARTED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EVENTS_RXSTARTED` reader - "]
pub struct EVENTS_RXSTARTED_R(crate::FieldReader<bool, bool>);
impl EVENTS_RXSTARTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        EVENTS_RXSTARTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EVENTS_RXSTARTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EVENTS_RXSTARTED` writer - "]
pub struct EVENTS_RXSTARTED_W<'a> {
    w: &'a mut W,
}
impl<'a> EVENTS_RXSTARTED_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_rxstarted(&self) -> EVENTS_RXSTARTED_R {
        EVENTS_RXSTARTED_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn events_rxstarted(&mut self) -> EVENTS_RXSTARTED_W {
        EVENTS_RXSTARTED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive sequence started\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [events_rxstarted](index.html) module"]
pub struct EVENTS_RXSTARTED_SPEC;
impl crate::RegisterSpec for EVENTS_RXSTARTED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [events_rxstarted::R](R) reader structure"]
impl crate::Readable for EVENTS_RXSTARTED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [events_rxstarted::W](W) writer structure"]
impl crate::Writable for EVENTS_RXSTARTED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVENTS_RXSTARTED to value 0"]
impl crate::Resettable for EVENTS_RXSTARTED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}