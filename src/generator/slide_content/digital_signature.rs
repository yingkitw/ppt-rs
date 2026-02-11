//! Digital signature support for PPTX presentations
//!
//! Provides digital signature metadata and XML generation for the
//! `_xmlsignatures/` package part per the OOXML digital signature spec.

/// Hash algorithm used for signing
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum HashAlgorithm {
    #[default]
    Sha256,
    Sha384,
    Sha512,
    Sha1,
}

impl HashAlgorithm {
    pub fn uri(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "http://www.w3.org/2001/04/xmlenc#sha256",
            HashAlgorithm::Sha384 => "http://www.w3.org/2001/04/xmldsig-more#sha384",
            HashAlgorithm::Sha512 => "http://www.w3.org/2001/04/xmlenc#sha512",
            HashAlgorithm::Sha1 => "http://www.w3.org/2000/09/xmldsig#sha1",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            HashAlgorithm::Sha256 => "SHA-256",
            HashAlgorithm::Sha384 => "SHA-384",
            HashAlgorithm::Sha512 => "SHA-512",
            HashAlgorithm::Sha1 => "SHA-1",
        }
    }
}

/// Signer identity information
#[derive(Clone, Debug, Default)]
pub struct SignerInfo {
    pub name: String,
    pub email: Option<String>,
    pub organization: Option<String>,
    pub title: Option<String>,
}

impl SignerInfo {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn organization(mut self, org: &str) -> Self {
        self.organization = Some(org.to_string());
        self
    }

    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

/// Digital signature configuration for a presentation
#[derive(Clone, Debug, Default)]
pub struct DigitalSignature {
    pub signer: SignerInfo,
    pub hash_algorithm: HashAlgorithm,
    pub sign_date: Option<String>,
    pub commitment_type: SignatureCommitment,
    pub comments: Option<String>,
}

/// Commitment type for the signature
#[derive(Clone, Debug, Copy, PartialEq, Eq, Default)]
pub enum SignatureCommitment {
    #[default]
    Created,
    Approved,
    Reviewed,
}

impl SignatureCommitment {
    pub fn uri(&self) -> &'static str {
        match self {
            SignatureCommitment::Created => "http://uri.etsi.org/01903/v1.2.2#ProofOfCreation",
            SignatureCommitment::Approved => "http://uri.etsi.org/01903/v1.2.2#ProofOfApproval",
            SignatureCommitment::Reviewed => "http://uri.etsi.org/01903/v1.2.2#ProofOfReview",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            SignatureCommitment::Created => "Created",
            SignatureCommitment::Approved => "Approved",
            SignatureCommitment::Reviewed => "Reviewed",
        }
    }
}

impl DigitalSignature {
    pub fn new(signer: SignerInfo) -> Self {
        Self {
            signer,
            hash_algorithm: HashAlgorithm::default(),
            sign_date: None,
            commitment_type: SignatureCommitment::default(),
            comments: None,
        }
    }

    pub fn hash_algorithm(mut self, algo: HashAlgorithm) -> Self {
        self.hash_algorithm = algo;
        self
    }

    pub fn sign_date(mut self, date: &str) -> Self {
        self.sign_date = Some(date.to_string());
        self
    }

    pub fn commitment(mut self, commitment: SignatureCommitment) -> Self {
        self.commitment_type = commitment;
        self
    }

    pub fn comments(mut self, comments: &str) -> Self {
        self.comments = Some(comments.to_string());
        self
    }

    /// Generate the `_xmlsignatures/origin.sigs` relationship XML
    pub fn to_origin_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?><Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"/>"#.to_string()
    }

    /// Generate signature info XML for `_xmlsignatures/sig1.xml`
    pub fn to_signature_xml(&self) -> String {
        let date = self.sign_date.as_deref().unwrap_or("2025-01-01T00:00:00Z");
        let comments_xml = self.comments.as_ref()
            .map(|c| format!("<SignatureComments>{}</SignatureComments>", xml_escape(c)))
            .unwrap_or_default();

        let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>"#);
        xml.push_str(r#"<Signature xmlns="http://www.w3.org/2000/09/xmldsig#">"#);
        xml.push_str(r#"<SignedInfo>"#);
        xml.push_str(r#"<CanonicalizationMethod Algorithm="http://www.w3.org/TR/2001/REC-xml-c14n-20010315"/>"#);
        xml.push_str(&format!(
            r#"<SignatureMethod Algorithm="{}"/>"#,
            self.hash_algorithm.uri()
        ));
        xml.push_str(r#"</SignedInfo>"#);
        xml.push_str(r#"<SignatureValue/>"#);
        xml.push_str(r#"<KeyInfo>"#);
        xml.push_str(&format!(
            r#"<KeyName>{}</KeyName>"#,
            xml_escape(&self.signer.name)
        ));
        xml.push_str(r#"</KeyInfo>"#);
        xml.push_str("<Object>");
        xml.push_str(&format!(
            "<SignatureProperties><SignatureProperty Target=\"#SignatureInfo\"><SignatureInfoV1 xmlns=\"http://schemas.microsoft.com/office/2006/digsig\"><SetupID/><SignatureText>{}</SignatureText>{}<SignatureType>1</SignatureType><SignatureProviderUrl/><SignatureProviderDetails>9</SignatureProviderDetails><ManifestHashAlgorithm>{}</ManifestHashAlgorithm><SignatureProviderId>{{{{00000000-0000-0000-0000-000000000000}}}}</SignatureProviderId><CommitmentTypeId>{}</CommitmentTypeId><CommitmentTypeQualifier>{}</CommitmentTypeQualifier><SigningTime>{}</SigningTime></SignatureInfoV1></SignatureProperty></SignatureProperties>",
            xml_escape(&self.signer.name),
            comments_xml,
            self.hash_algorithm.uri(),
            self.commitment_type.uri(),
            self.commitment_type.label(),
            date,
        ));
        xml.push_str(r#"</Object>"#);
        xml.push_str(r#"</Signature>"#);
        xml
    }

    /// Generate content type entry for digital signatures
    pub fn content_type_entry() -> &'static str {
        r#"<Override PartName="/_xmlsignatures/sig1.xml" ContentType="application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml"/>"#
    }
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithm_default() {
        let algo = HashAlgorithm::default();
        assert_eq!(algo, HashAlgorithm::Sha256);
        assert!(algo.uri().contains("sha256"));
        assert_eq!(algo.name(), "SHA-256");
    }

    #[test]
    fn test_hash_algorithm_variants() {
        assert!(HashAlgorithm::Sha384.uri().contains("sha384"));
        assert!(HashAlgorithm::Sha512.uri().contains("sha512"));
        assert!(HashAlgorithm::Sha1.uri().contains("sha1"));
        assert_eq!(HashAlgorithm::Sha384.name(), "SHA-384");
        assert_eq!(HashAlgorithm::Sha512.name(), "SHA-512");
        assert_eq!(HashAlgorithm::Sha1.name(), "SHA-1");
    }

    #[test]
    fn test_signer_info_new() {
        let signer = SignerInfo::new("Alice");
        assert_eq!(signer.name, "Alice");
        assert!(signer.email.is_none());
        assert!(signer.organization.is_none());
    }

    #[test]
    fn test_signer_info_builder() {
        let signer = SignerInfo::new("Bob")
            .email("bob@example.com")
            .organization("Acme Corp")
            .title("Engineer");
        assert_eq!(signer.name, "Bob");
        assert_eq!(signer.email.as_deref(), Some("bob@example.com"));
        assert_eq!(signer.organization.as_deref(), Some("Acme Corp"));
        assert_eq!(signer.title.as_deref(), Some("Engineer"));
    }

    #[test]
    fn test_signature_commitment_variants() {
        assert!(SignatureCommitment::Created.uri().contains("Creation"));
        assert!(SignatureCommitment::Approved.uri().contains("Approval"));
        assert!(SignatureCommitment::Reviewed.uri().contains("Review"));
        assert_eq!(SignatureCommitment::Created.label(), "Created");
        assert_eq!(SignatureCommitment::Approved.label(), "Approved");
        assert_eq!(SignatureCommitment::Reviewed.label(), "Reviewed");
    }

    #[test]
    fn test_digital_signature_new() {
        let sig = DigitalSignature::new(SignerInfo::new("Alice"));
        assert_eq!(sig.signer.name, "Alice");
        assert_eq!(sig.hash_algorithm, HashAlgorithm::Sha256);
        assert_eq!(sig.commitment_type, SignatureCommitment::Created);
    }

    #[test]
    fn test_digital_signature_builder() {
        let sig = DigitalSignature::new(SignerInfo::new("Bob"))
            .hash_algorithm(HashAlgorithm::Sha512)
            .sign_date("2025-06-15T10:00:00Z")
            .commitment(SignatureCommitment::Approved)
            .comments("Looks good");
        assert_eq!(sig.hash_algorithm, HashAlgorithm::Sha512);
        assert_eq!(sig.sign_date.as_deref(), Some("2025-06-15T10:00:00Z"));
        assert_eq!(sig.commitment_type, SignatureCommitment::Approved);
        assert_eq!(sig.comments.as_deref(), Some("Looks good"));
    }

    #[test]
    fn test_signature_xml() {
        let sig = DigitalSignature::new(SignerInfo::new("Alice"))
            .sign_date("2025-01-01T00:00:00Z");
        let xml = sig.to_signature_xml();
        assert!(xml.contains("<Signature"));
        assert!(xml.contains("Alice"));
        assert!(xml.contains("sha256"));
        assert!(xml.contains("SigningTime"));
    }

    #[test]
    fn test_signature_xml_with_comments() {
        let sig = DigitalSignature::new(SignerInfo::new("Bob"))
            .comments("Reviewed & approved");
        let xml = sig.to_signature_xml();
        assert!(xml.contains("Reviewed &amp; approved"));
    }

    #[test]
    fn test_origin_xml() {
        let sig = DigitalSignature::new(SignerInfo::new("X"));
        let xml = sig.to_origin_xml();
        assert!(xml.contains("Relationships"));
    }

    #[test]
    fn test_content_type_entry() {
        let ct = DigitalSignature::content_type_entry();
        assert!(ct.contains("digital-signature"));
        assert!(ct.contains("sig1.xml"));
    }
}
