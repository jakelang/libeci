pub struct EcicContext {
    input_wast: String
}

impl EcicContext {
    /*
     * Basic context constructors
     */
    pub fn new() -> Self {
        EcicContext::empty()
    }

    pub fn empty() -> Self {
        EcicContext {
            input_wast: String::new()
        }
    }

    pub fn from_string(wast: &String) -> Self {
        debug!("Loading context with wast: {}", wast);
        EcicContext {
            input_wast: wast.clone()
        }
    }

    /*
     * Debug stuff
     */
    pub fn debug_printwast(self) {
        debug!("Context has wast:\n {}", self.input_wast);
    }

    pub fn wast_len(&self) -> usize {
        self.input_wast.len()
    }
}

#[cfg(test)]
mod tests {
    use context::EcicContext;

    #[test]
    fn empty_wast() {
        let empty_ctx = EcicContext::new();
        assert!(empty_ctx.input_wast.is_empty());
    }
    
    #[test]
    fn basic_wast() {
        let basic_wast = String::from("(module)");
        let basic_ctx = EcicContext::from_string(&basic_wast);
        assert_eq!(8, basic_wast.len());
        assert_eq!(8, basic_ctx.wast_len());
    }
}
