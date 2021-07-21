// Used to hash new content or check existing content and existing hashes.
export type Hasher<Content, Hash> = (content: Content) => Promise<Hash>;
