use rand::rngs::OsRng;
use rand::RngCore;

pub struct Form {
    boundary: String,
    body: Vec<u8>,
}

impl Form {
    pub fn new() -> Form {
        let mut raw = [0u8; 16];
        OsRng.fill_bytes(&mut raw);
        let tail: String = raw.iter().map(|byte| format!("{:02x}", byte)).collect();
        Form {
            boundary: format!("----WebKitFormBoundary{}", tail),
            body: Vec::new(),
        }
    }

    pub fn field(&mut self, name: &str, value: &str) -> &mut Form {
        let head = format!(
            "--{}\r\nContent-Disposition: form-data; name=\"{}\"\r\n\r\n",
            self.boundary, name
        );
        self.body.extend_from_slice(head.as_bytes());
        self.body.extend_from_slice(value.as_bytes());
        self.body.extend_from_slice(b"\r\n");
        self
    }

    pub fn finish(mut self) -> (String, Vec<u8>) {
        let tail = format!("--{}--\r\n", self.boundary);
        self.body.extend_from_slice(tail.as_bytes());
        (
            format!("multipart/form-data; boundary={}", self.boundary),
            self.body,
        )
    }
}

impl Default for Form {
    fn default() -> Self {
        Form::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fields_land_in_order_and_the_body_closes() {
        let mut form = Form::new();
        form.field("_1_email", "a@b.test")
            .field("0", "[\"$undefined\"]");
        let (content_type, body) = form.finish();
        let text = String::from_utf8(body).unwrap();

        assert!(content_type.starts_with("multipart/form-data; boundary=----WebKitFormBoundary"));
        assert!(text.contains("name=\"_1_email\""));
        assert!(text.contains("a@b.test"));
        assert!(text.contains("name=\"0\""));
        assert!(text.contains("[\"$undefined\"]"));
        assert!(text.ends_with("--\r\n"));
        assert!(text.find("_1_email").unwrap() < text.find("name=\"0\"").unwrap());
    }

    #[test]
    fn every_form_gets_its_own_boundary() {
        let first = Form::new().finish().0;
        let second = Form::new().finish().0;
        assert_ne!(first, second);
    }
}
