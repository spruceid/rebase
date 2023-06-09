use crate::types::error::ContentError;
use serde_json::json;
use ssi::jsonld::ContextLoader;
use std::collections::HashMap;

pub const REBASE_XYZ_V1_CONTEXT: &str = "https://spec.rebase.xyz/contexts/v1";

pub fn context_loader() -> Result<ContextLoader, ContentError> {
    let m = HashMap::from([(
        REBASE_XYZ_V1_CONTEXT.to_string(),
        // NOTE: Due to a formatting error, this can't just be retrieved.
        // It must be wrapped in the "@context" container.
        // NOTE: This doesn't actually line up at all, but SSI accepts it.
        serde_json::to_string(&json!({
            "@context": {
              "DnsVerification": {
                "@id": "https://w3id.org/rebase#DnsVerification",
                "@context": {
                  "sameAs": {
                    "@id": "https://schema.org/sameAs",
                    "@type": "@id"
                  }
                }
              },
              "DnsVerificationMessage": {
                "@id": "https://w3id.org/rebase#DnsVerificationMessage",
                "@context": {
                  "dnsServer": {
                    "@id": "https://schema.org/url",
                    "@type": "@id"
                  },
                  "timestamp": {
                    "@id": "https://schema.org/temporal",
                    "@type": "https://schema.org/DateTime"
                  }
                }
              },
              "EmailVerification": {
                "@id": "https://w3id.org/rebase#EmailVerification",
                "@context": {
                  "sameAs": {
                    "@id": "https://schema.org/sameAs",
                    "@type": "@id"
                  }
                }
              },
              "EmailVerificationMessage": {
                "@id": "https://w3id.org/rebase#EmailVerificationMessage",
                "@context": {
                  "email": {
                    "@id": "https://schema.org/email",
                    "@type": "https://schema.org/Text"
                  },
                  "timestamp": {
                    "@id": "https://schema.org/temporal",
                    "@type": "https://schema.org/DateTime"
                  }
                }
              },
              "GitHubVerification": {
                "@id": "https://w3id.org/rebase#GitHubVerification",
                "@context": {
                  "sameAs": {
                    "@id": "https://schema.org/sameAs",
                    "@type": "@id"
                  }
                }
              },
              "GitHubVerificationMessage": {
                "@id": "https://w3id.org/rebase#GitHubVerificationMessage",
                "@context": {
                  "gistId": {
                    "@id": "https://schema.org/url",
                    "@type": "@id"
                  },
                  "handle": {
                    "@id": "https://schema.org/name",
                    "@type": "https://schema.org/Text"
                  },
                  "timestamp": {
                    "@id": "https://schema.org/temporal",
                    "@type": "https://schema.org/DateTime"
                  }
                }
              },
              "RedditVerification": {
                "@id": "https://w3id.org/rebase#RedditVerification",
                "@context": {
                  "sameAs": {
                    "@id": "https://schema.org/sameAs",
                    "@type": "@id"
                  }
                }
              },
              "RedditVerificationMessage": {
                "@id": "https://w3id.org/rebase#RedditVerificationMessage",
                "@context": {
                  "handle": {
                    "@id": "https://schema.org/name",
                    "@type": "https://schema.org/Text"
                  },
                  "timestamp": {
                    "@id": "https://schema.org/temporal",
                    "@type": "https://schema.org/DateTime"
                  }
                }
              },
              "SameControllerAssertion": "https://w3id.org/rebase#SameControllerAssertion",
              "SameControllerEvidence": {
                "@id": "https://w3id.org/rebase#SameControllerEvidence",
                "@context": {
                  "signature1": {
                    "@id": "https://w3id.org/rebase#signature1",
                    "@type": "https://schema.org/Text"
                  },
                  "signature2": {
                    "@id": "https://w3id.org/rebase#signature2",
                    "@type": "https://schema.org/Text"
                  },
                  "statement": {
                    "@id": "https://w3id.org/rebase#statement",
                    "@type": "https://schema.org/Text"
                  }
                }
              },
              "SoundCloudVerification": {
                "@id": "https://w3id.org/rebase#SoundCloudVerification",
                "@context": {
                  "sameAs": {
                    "@id": "https://schema.org/sameAs",
                    "@type": "@id"
                  }
                }
              },
              "SoundCloudVerificationMessage": {
                "@id": "https://w3id.org/rebase#SoundCloudVerificationMessage",
                "@context": {
                  "permalink": {
                    "@id": "https://schema.org/identifier",
                    "@type": "https://schema.org/Text"
                  },
                  "timestamp": {
                    "@id": "https://schema.org/temporal",
                    "@type": "https://schema.org/DateTime"
                  }
                }
              },
              "TwitterVerification": {
                "@id": "https://w3id.org/rebase#TwitterVerification",
                "@context": {
                  "sameAs": {
                    "@id": "https://schema.org/sameAs",
                    "@type": "@id"
                  }
                }
              },
              "TwitterVerificationPublicTweet": {
                "@id": "https://w3id.org/rebase#TwitterVerificationPublicTweet",
                "@context": {
                  "handle": {
                    "@id": "https://schema.org/name",
                    "@type": "https://schema.org/Text"
                  },
                  "timestamp": {
                    "@id": "https://schema.org/temporal",
                    "@type": "https://schema.org/DateTime"
                  },
                  "tweetId": {
                    "@id": "https://schema.org/identifier",
                    "@type": "https://schema.org/Text"
                  }
                }
              }
            }
        }))
        .map_err(|e| ContentError::Invalid(e.to_string()))?,
    )]);
    ContextLoader::default()
        .with_context_map_from(m)
        .map_err(|e| ContentError::Invalid(e.to_string()))
}
